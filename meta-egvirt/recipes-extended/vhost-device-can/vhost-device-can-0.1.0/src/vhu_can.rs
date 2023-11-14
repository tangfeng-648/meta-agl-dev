// vhost device can
//
// Copyright 2023 VIRTUAL OPEN SYSTEMS SAS. All Rights Reserved.
//          Timos Ampelikiotis <t.ampelikiotis@virtualopensystems.com>
//
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

use log::{warn, error};
use std::mem::size_of;
use std::slice::from_raw_parts;
use std::sync::{Arc, RwLock};
use std::{
    convert,
    io::{self, Result as IoResult},
};
use std::os::fd::AsRawFd;
use thiserror::Error as ThisError;
use vhost::vhost_user::message::{VhostUserProtocolFeatures, VhostUserVirtioFeatures};
use vhost_user_backend::{VhostUserBackendMut, VringRwLock, VringT};
use virtio_bindings::bindings::virtio_config::{VIRTIO_F_NOTIFY_ON_EMPTY, VIRTIO_F_VERSION_1};
use virtio_bindings::bindings::virtio_ring::{
    VIRTIO_RING_F_EVENT_IDX, VIRTIO_RING_F_INDIRECT_DESC,
};
use virtio_queue::{DescriptorChain, QueueOwnedT};
use vm_memory::{
    ByteValued, Bytes, GuestAddressSpace, GuestMemoryAtomic, GuestMemoryLoadGuard,
    GuestMemoryMmap, Le16, Le32,
};
use vmm_sys_util::epoll::EventSet;
use vmm_sys_util::eventfd::{EventFd, EFD_NONBLOCK};
use crate::can::{
	CanController, CAN_EFF_FLAG, CAN_RTR_FLAG, CAN_ERR_FLAG, CAN_SFF_MASK,
	CAN_EFF_MASK, VIRTIO_CAN_FLAGS_FD, VIRTIO_CAN_FLAGS_RTR,
	VIRTIO_CAN_FLAGS_EXTENDED, VIRTIO_CAN_TX, VIRTIO_CAN_RX,
	CAN_FRMF_TYPE_FD, CAN_ERR_BUSOFF,
};
use vhost_user_backend::VringEpollHandler;

/// Feature bit numbers
pub const VIRTIO_CAN_F_CAN_CLASSIC: u16 = 0;
pub const VIRTIO_CAN_F_CAN_FD: u16 = 1;
//pub const VIRTIO_CAN_F_LATE_TX_ACK: u16 = 2;
pub const VIRTIO_CAN_F_RTR_FRAMES: u16 = 3;

/// Possible values of the status field
pub const VIRTIO_CAN_STATUS_OK: u8 = 0x0;
pub const VIRTIO_CAN_STATUS_ERR: u8 = 0x1;

/// CAN Control messages
const VIRTIO_CAN_SET_CTRL_MODE_START: u16 = 0x0201;
const VIRTIO_CAN_SET_CTRL_MODE_STOP: u16 = 0x0202;

/// Virtio configuration
const QUEUE_SIZE: usize = 64;
const NUM_QUEUES: usize = 3;

/// Queues
const TX_QUEUE: u16 = 0;
const RX_QUEUE: u16 = 1;
const CTRL_QUEUE: u16 = 2;
const BACKEND_EFD: u16 = 4;

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, ThisError)]
/// Errors related to vhost-device-gpio-daemon.
pub(crate) enum Error {
    #[error("Failed to handle event, didn't match EPOLLIN")]
    HandleEventNotEpollIn,
    #[error("Failed to handle unknown event")]
    HandleEventUnknown,
    #[error("Received unexpected write only descriptor at index {0}")]
    UnexpectedWriteOnlyDescriptor(usize),
    #[error("Received unexpected readable descriptor at index {0}")]
    UnexpectedReadableDescriptor(usize),
    #[error("Invalid descriptor count {0}")]
    UnexpectedDescriptorCount(usize),
    #[error("Invalid descriptor size, expected: {0}, found: {1}")]
    UnexpectedDescriptorSize(usize, u32),
    #[error("Descriptor not found")]
    DescriptorNotFound,
    #[error("Descriptor read failed")]
    DescriptorReadFailed,
    #[error("Descriptor write failed")]
    DescriptorWriteFailed,
    #[error("Failed to send notification")]
    NotificationFailed,
    #[error("Failed to create new EventFd")]
    EventFdFailed,
    #[error("Unknown can message type: {0}")]
	UnexpectedCanMsgType(u16),
    #[error("RTR frames not negotiated")]
	UnexpectedRtrFlag,
    #[error("Can FD frames not negotiated")]
	UnexpectedFdFlag,
    #[error("Classic CAN frames not negotiated")]
	UnexpectedClassicFlag,
}

impl convert::From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

/// Virtio CAN Request / Response messages
///
/// The response message is a stream of bytes, where first byte represents the
/// status, and rest is message specific data.

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct VirtioCanTxResponse {
    result: i8,
}
// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioCanTxResponse {}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct VirtioCanFrame {
	pub msg_type: Le16,
    pub length: Le16,   /* 0..8 CC, 0..64 CAN­FD, 0..2048 CAN­XL, 12 bits */
	pub reserved: Le32, /* May be needed in part for CAN XL priority */
    pub flags: Le32,
    pub can_id: Le32,
    pub sdu: [u8; 64],
}

impl Default for VirtioCanFrame {
    fn default() -> Self {
        VirtioCanFrame {
            msg_type: Le16::default(),
            length: Le16::default(),
            reserved: Le32::default(),
            flags: Le32::default(),
            can_id: Le32::default(),
            sdu: [0; 64], // Initialize "asd" with default value (0 in this case)
        }
    }
}

// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioCanFrame {}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct VirtioCanCtrlRequest {
	msg_type: Le16,
}
// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioCanCtrlRequest {}

#[derive(Copy, Clone, Default)]
#[repr(C)]
struct VirtioCanCtrlResponse {
    result: i8,
}
// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioCanCtrlResponse {}

pub(crate) struct VhostUserCanBackend {
    controller: Arc<RwLock<CanController>>,
	acked_features: u64,
    event_idx: bool,
    pub(crate) exit_event: EventFd,
    mem: Option<GuestMemoryAtomic<GuestMemoryMmap>>,
}

type CanDescriptorChain = DescriptorChain<GuestMemoryLoadGuard<GuestMemoryMmap<()>>>;

impl VhostUserCanBackend {
    pub(crate) fn new(controller: Arc<RwLock<CanController>>) -> Result<Self> {
        Ok(VhostUserCanBackend {
            controller: controller,
            event_idx: false,
			acked_features: 0x0,
            exit_event: EventFd::new(EFD_NONBLOCK).map_err(|_| Error::EventFdFailed)?,
            mem: None,
        })
    }

	fn check_features (&self, features: u16) -> bool {
		(self.acked_features & (1 << features)) != 0
	}

    fn process_ctrl_requests(
		&self,
		requests: Vec<CanDescriptorChain>,
		vring: &VringRwLock
	) -> Result<bool> {
		dbg!("process_ctrl_requests");

        if requests.is_empty() {
            return Ok(true);
        }

        for desc_chain in requests {
			let descriptors: Vec<_> = desc_chain.clone().collect();

            if descriptors.len() < 1 {
				warn!("Error::UnexpectedDescriptorCount");
                return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
            }

			println!("descriptors.len(): {:?}", descriptors.len());

            let desc_request = descriptors[0];
            if desc_request.is_write_only() {
				warn!("Error::UnexpectedWriteOnlyDescriptor");
                return Err(Error::UnexpectedWriteOnlyDescriptor(0));
            }

            if desc_request.len() as usize != size_of::<VirtioCanCtrlRequest>() {
				println!("UnexpectedDescriptorSize, len = {:?}", desc_request.len());
                return Err(Error::UnexpectedDescriptorSize(
                    size_of::<VirtioCanCtrlRequest>(),
                    desc_request.len(),
                ));
            }

            let request = desc_chain
                .memory()
                .read_obj::<VirtioCanCtrlRequest>(desc_request.addr())
                .map_err(|_| Error::DescriptorReadFailed)?;


           match request.msg_type.into() {
				VIRTIO_CAN_SET_CTRL_MODE_START => {
					println!("VIRTIO_CAN_SET_CTRL_MODE_START");
					//vcan->busoff = false;
					Ok(())
				}
				VIRTIO_CAN_SET_CTRL_MODE_STOP => {
					println!("VIRTIO_CAN_SET_CTRL_MODE_STOP");
	            	//vcan->busoff = false;
					Ok(())
				}
				_ => {
					println!("Ctrl queue: msg type 0x{:?} unknown", request.msg_type);
					return Err(Error::HandleEventUnknown.into())
				},
			}?;

            let desc_response = descriptors[1];
            if !desc_response.is_write_only() {
				println!("This is not wirtable");
                return Err(Error::UnexpectedReadableDescriptor(1));
            }

			let response = VIRTIO_CAN_STATUS_OK;

            desc_chain
                .memory()
                .write_slice(response.as_slice(), desc_response.addr())
                .map_err(|_| Error::DescriptorWriteFailed)?;

            if vring.add_used(desc_chain.head_index(), desc_response.len()).is_err() {
                println!("Couldn't return used descriptors to the ring");
                warn!("Couldn't return used descriptors to the ring");
            }
        }

        Ok(true)
    }

    fn process_tx_requests(
		&self,
		requests: Vec<CanDescriptorChain>,
		vring: &VringRwLock
	) -> Result<bool> {
		dbg!("process_tx_requests");

        if requests.is_empty() {
            return Ok(true);
        }

		println!("requests.len: {:?}", requests.len());
        for desc_chain in requests {
            let descriptors: Vec<_> = desc_chain.clone().collect();

            if descriptors.len() != 2 {
				println!("Error::UnexpectedDescriptorCount");
                return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
            }

            let desc_request = descriptors[0];
            if desc_request.is_write_only() {
				println!("Error::UnexpectedReadOnlyDescriptor");
                return Err(Error::UnexpectedWriteOnlyDescriptor(0));
			}

            if desc_request.len() as usize != size_of::<VirtioCanFrame>() {
			    println!("Tx UnexpectedDescriptorSize, len = {:?}", desc_request.len());
                //return Err(Error::UnexpectedDescriptorSize(
                //    size_of::<VirtioCanFrame>(),
                //    desc_request.len(),
                //));
            }

            let request = desc_chain
                .memory()
                .read_obj::<VirtioCanFrame>(desc_request.addr())
                .map_err(|_| Error::DescriptorReadFailed)?;

			CanController::print_can_frame(request);

			let msg_type = request.msg_type.to_native();
			let mut can_id = request.can_id.to_native();
			let mut flags = request.flags.to_native();
			let mut length = request.length.to_native();

	        if msg_type != VIRTIO_CAN_TX {
	            warn!("TX: Message type 0x{:x} unknown\n", msg_type);
                return Err(Error::UnexpectedCanMsgType(msg_type));
	        }

	        if (flags & VIRTIO_CAN_FLAGS_FD) != 0 {
	            if length > 64 {
	                println!("Cut sdu_len from {:?} to 64\n", request.length);
	                length = 64;
	            }
	        } else {
	            if length > 8 {
	                println!("Cut sdu_len from {:?} to 8\n", request.length);
	                length = 8;
	            }
	        }

	        /*
	         * Copy Virtio frame structure to qemu frame structure and
	         * check while doing this whether the frame type was negotiated
	         */
	        if (flags & VIRTIO_CAN_FLAGS_EXTENDED) != 0 {
	            flags &= CAN_EFF_MASK;
	            flags |= CAN_EFF_FLAG;
	        } else {
	            flags &= CAN_SFF_MASK;
	        }

	        if (flags & VIRTIO_CAN_FLAGS_RTR) != 0 {
	            if !self.check_features(VIRTIO_CAN_F_CAN_CLASSIC) ||
	                !self.check_features(VIRTIO_CAN_F_RTR_FRAMES) {
	                warn!("TX: RTR frames not negotiated");
					return Err(Error::UnexpectedRtrFlag);
	            }
	            can_id |= flags | CAN_RTR_FLAG;
	        }

	        if (flags & VIRTIO_CAN_FLAGS_FD) != 0 {
	            if !self.check_features(VIRTIO_CAN_F_CAN_FD) {
	                warn!("TX: FD frames not negotiated\n");
					return Err(Error::UnexpectedFdFlag);
	            }
	            flags |= CAN_FRMF_TYPE_FD;
	        } else {
	            if !self.check_features(VIRTIO_CAN_F_CAN_CLASSIC) {
	                warn!("TX: Classic frames not negotiated\n");
					return Err(Error::UnexpectedClassicFlag);
	            }
	            flags = 0;
	        }

	        let mut corrected_request = VirtioCanFrame::default();
			corrected_request.msg_type = msg_type.into();
			corrected_request.can_id = can_id.into();
			corrected_request.flags = flags.into();
			corrected_request.length = length.into();
			corrected_request.sdu.copy_from_slice(&request.sdu[0..64]);

            let desc_response = descriptors[1];
            if !desc_response.is_write_only() {
				println!("Error::UnexpectedWriteOnlyDescriptor");
                return Err(Error::UnexpectedReadableDescriptor(1));
            }

			let response = match self.controller.write().unwrap().operation(corrected_request) {
				Ok(result) => {
					result
    			}
    			Err(_) => {
    			    warn!("We got an error from controller send func");
					VIRTIO_CAN_STATUS_ERR
    			}
			};

            desc_chain
                .memory()
                .write_slice(response.as_slice(), desc_response.addr())
                .map_err(|_| Error::DescriptorWriteFailed)?;

            if vring.add_used(desc_chain.head_index(), desc_response.len()).is_err() {
                println!("Couldn't return used descriptors to the ring");
                warn!("Couldn't return used descriptors to the ring");
            }
        }

        Ok(true)
    }

    fn process_rx_requests(
		&mut self,
		requests: Vec<CanDescriptorChain>,
		vring: &VringRwLock
	) -> Result<bool> {
		dbg!("process_rx_requests");

        if requests.is_empty() {
            return Ok(true);
        }

        let desc_chain = &requests[0];
        let descriptors: Vec<_> = desc_chain.clone().collect();

        if descriptors.len() != 1 {
			println!("Error::UnexpectedDescriptorCount");
            return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
        }

        let desc_response = descriptors[0];
        if !desc_response.is_write_only() {
            return Err(Error::UnexpectedReadableDescriptor(1));
        }

		let mut response = match self.controller.write().unwrap().pop() {
		    Ok(item) => item,
		    Err(_) => return Err(Error::HandleEventUnknown),
		};

		CanController::print_can_frame(response);

        if (response.can_id.to_native() & CAN_ERR_FLAG) != 0 {
            if (response.can_id.to_native() & CAN_ERR_BUSOFF) != 0 {
                warn!("Got BusOff error frame, device does a local bus off\n");
                //vcan->busoff = true;
            } else {
                println!("Dropping error frame 0x{:x}\n", response.can_id.to_native());
            }
            return Ok(true);
        }

		let mut can_rx = VirtioCanFrame::default();
		can_rx.msg_type = VIRTIO_CAN_RX.into();
		can_rx.can_id = response.can_id;
		can_rx.length = response.length;
		can_rx.flags = (can_rx.flags.to_native() | VIRTIO_CAN_FLAGS_FD).into();

        if (response.flags.to_native() & CAN_FRMF_TYPE_FD) != 0 {
            if !self.check_features(VIRTIO_CAN_F_CAN_FD) {
				warn!("Drop non-supported CAN FD frame");
				return Err(Error::UnexpectedFdFlag);
            }
        } else {
            if !self.check_features(VIRTIO_CAN_F_CAN_CLASSIC) {
				warn!("Drop non-supported CAN classic frame");
				return Err(Error::UnexpectedClassicFlag);
            }
        }
        if (response.can_id.to_native() & CAN_RTR_FLAG) != 0 &&
			!self.check_features(VIRTIO_CAN_F_RTR_FRAMES) {
				warn!("Drop non-supported RTR frame");
				return Err(Error::UnexpectedRtrFlag);
        }

		if (response.can_id.to_native() & CAN_EFF_FLAG) != 0 {
		    can_rx.flags = VIRTIO_CAN_FLAGS_EXTENDED.into();
		    can_rx.can_id = (response.can_id.to_native() & CAN_EFF_MASK).into();
		} else {
		    can_rx.can_id = (response.can_id.to_native() & CAN_SFF_MASK).into();
		}
		if (response.can_id.to_native() & CAN_RTR_FLAG) != 0 {
		    can_rx.flags = (can_rx.flags.to_native() & VIRTIO_CAN_FLAGS_RTR).into();
		}

		// HACK AHEAD: Vcan can not be comfigured as CANFD interface, but possible
		// to configure its MTU to 64 bytes. So if a messages bigger than 8 bytes
		// is being received we consider it as CANFD message.
		let can_in_name = self.controller.read().unwrap().can_in_name.clone();
	    if self.check_features(VIRTIO_CAN_F_CAN_FD) &&
		   response.length.to_native() > 8 && can_in_name == "vcan0" {
			response.flags = (response.flags.to_native() | CAN_FRMF_TYPE_FD).into();
			warn!("\n\n\nCANFD VCAN0\n\n");
		}

		if (response.flags.to_native() & CAN_FRMF_TYPE_FD) != 0 {
		    can_rx.flags = (can_rx.flags.to_native() | VIRTIO_CAN_FLAGS_FD).into();
		    if response.length.to_native() > 64 {
		        warn!("%s(): Cut length from {} to 64\n", response.length.to_native());
		        can_rx.length = 64.into();
		    }
		} else {
		    if response.length.to_native() > 8 {
		        warn!("%s(): Cut length from {} to 8\n", response.length.to_native());
		        can_rx.length = 8.into();
		    }
		}

		can_rx.sdu.copy_from_slice(&response.sdu[0..64]);
		CanController::print_can_frame(can_rx);

        desc_chain
            .memory()
            .write_slice(can_rx.as_slice(), desc_response.addr())
            .map_err(|_| Error::DescriptorWriteFailed)?;

        if vring.add_used(desc_chain.head_index(), desc_response.len()).is_err() {
            warn!("Couldn't return used descriptors to the ring");
        }

        Ok(true)
    }

    /// Process the messages in the vring and dispatch replies
    fn process_ctrl_queue(&mut self, vring: &VringRwLock) -> Result<()> {
		dbg!("process_ctrl_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_ctrl_requests(requests, vring)? {
            // Send notification once all the requests are processed
            vring
                .signal_used_queue()
                .map_err(|_| Error::NotificationFailed)?;
        }
		Ok(())
    }

    /// Process the messages in the vring and dispatch replies
    fn process_tx_queue(&self, vring: &VringRwLock) -> Result<()> {
		dbg!("process_tx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_tx_requests(requests, vring)? {
            // Send notification once all the requests are processed
            vring
                .signal_used_queue()
                .map_err(|_| {
					println!("signal_used_queue error");
					Error::NotificationFailed
				})?;
        }

        Ok(())
    }

    /// Process the messages in the vring and dispatch replies
    fn process_rx_queue(&mut self, vring: &VringRwLock) -> Result<()> {
		dbg!("process_rx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_rx_requests(requests, vring)? {
            // Send notification once all the requests are processed
            vring
                .signal_used_queue()
                .map_err(|_| {
					println!("NotificationFailed");
					Error::NotificationFailed
				})?;
        }
		Ok(())
    }

    fn process_rx_queue_dump(&mut self, _vring: &VringRwLock) -> Result<()> {
		dbg!("Do nothing, if you reach that point!");
		Ok(())
    }

    /// Set self's VringWorker.
    pub(crate) fn set_vring_worker(
		&self,
        vring_worker: &Arc<VringEpollHandler<Arc<RwLock<VhostUserCanBackend>>, VringRwLock, ()>>,
    ) {
		let rx_event_fd = self.controller.read().unwrap().rx_event_fd.as_raw_fd();
		vring_worker
            .register_listener(
				rx_event_fd,
				EventSet::IN,
				//u64::from(BACKEND_EFD))
				4u64  as u64)
            .unwrap();
    }
}

/// VhostUserBackendMut trait methods
impl VhostUserBackendMut<VringRwLock, ()>
    for VhostUserCanBackend
{
    fn num_queues(&self) -> usize {
		println!("num_queues: {:?}", NUM_QUEUES);
        NUM_QUEUES
    }

    fn max_queue_size(&self) -> usize {
		println!("max_queue_size: {:?}", QUEUE_SIZE);
        QUEUE_SIZE
    }

    fn features(&self) -> u64 {
        // this matches the current libvhost defaults except VHOST_F_LOG_ALL
        let features = 1 << VIRTIO_F_VERSION_1
            | 1 << VIRTIO_F_NOTIFY_ON_EMPTY
            | 1 << VIRTIO_RING_F_EVENT_IDX
			| 1 << VIRTIO_CAN_F_CAN_CLASSIC
			| 1 << VIRTIO_CAN_F_CAN_FD
            | 1 << VIRTIO_RING_F_INDIRECT_DESC
            | VhostUserVirtioFeatures::PROTOCOL_FEATURES.bits();

		println!("vhu_can->features: {:x}", features);
		features
    }

	fn acked_features(&mut self, _features: u64) {
		println!("\nacked_features: 0x{:x}\n", _features);
		self.acked_features = _features;
	}

    fn protocol_features(&self) -> VhostUserProtocolFeatures {
        let protocol_features = VhostUserProtocolFeatures::MQ
            | VhostUserProtocolFeatures::CONFIG
            | VhostUserProtocolFeatures::REPLY_ACK;
            //| VhostUserProtocolFeatures::STATUS

		println!("protocol_features: {:x}", protocol_features);
		protocol_features
    }

    fn get_config(&self, offset: u32, size: u32) -> Vec<u8> {
        // SAFETY: The layout of the structure is fixed and can be initialized by
        // reading its content from byte array.
		dbg!("vhu_can->get_config");
        unsafe {
            from_raw_parts(
                self.controller.write().unwrap()
                    .config()
                    .as_slice()
                    .as_ptr()
                    .offset(offset as isize) as *const _ as *const _,
                size as usize,
            )
            .to_vec()
        }
    }

    fn set_event_idx(&mut self, enabled: bool) {
        dbg!(self.event_idx = enabled);
    }

    fn update_memory(&mut self, mem: GuestMemoryAtomic<GuestMemoryMmap>) -> IoResult<()> {
		dbg!("update_memory\n");
        self.mem = Some(mem);
        Ok(())
    }

    fn handle_event(
        &mut self,
        device_event: u16,
        evset: EventSet,
        vrings: &[VringRwLock],
        _thread_id: usize,
    ) -> IoResult<bool> {
		dbg!("\nhandle_event:");

        if evset != EventSet::IN {
            return Err(Error::HandleEventNotEpollIn.into());
        }
		if device_event == RX_QUEUE {
			println!("RX_QUEUE\n");
			return Ok(false);
		};
		let vring = if device_event != BACKEND_EFD {
			&vrings[device_event as usize]
		} else {
			println!("BACKEND_EFD\n");
			let _ = self.controller.write().unwrap().rx_event_fd.read();
			&vrings[RX_QUEUE as usize]
		};
        if self.event_idx {
            // vm-virtio's Queue implementation only checks avail_index
            // once, so to properly support EVENT_IDX we need to keep
            // calling process_request_queue() until it stops finding
            // new requests on the queue.
            loop {
                vring.disable_notification().unwrap();
                //match queue_idx {
                match device_event {
                    CTRL_QUEUE => self.process_ctrl_queue(vring),
                    TX_QUEUE => self.process_tx_queue(vring),
                    RX_QUEUE => self.process_rx_queue_dump(vring),
                    BACKEND_EFD => self.process_rx_queue(vring),
                    _ => Err(Error::HandleEventUnknown.into()),
                }?;
                if !vring.enable_notification().unwrap() {
                    break;
                }
            }
        } else {
            // Without EVENT_IDX, a single call is enough.
            match device_event {
                CTRL_QUEUE => self.process_ctrl_queue(vring),
                TX_QUEUE => self.process_tx_queue(vring),
                RX_QUEUE => self.process_rx_queue_dump(vring),
                BACKEND_EFD => self.process_rx_queue(vring),
                _ => Err(Error::HandleEventUnknown.into()),
            }?;
        }
        Ok(false)
    }

    fn exit_event(&self, _thread_index: usize) -> Option<EventFd> {
		dbg!("exit_event\n");
        self.exit_event.try_clone().ok()
    }
}

