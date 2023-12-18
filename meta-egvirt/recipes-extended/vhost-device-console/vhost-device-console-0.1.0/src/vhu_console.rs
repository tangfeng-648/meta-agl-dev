// vhost device console
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
use std::io::{Write};
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
use vhost_user_backend::VringEpollHandler;
use crate::console::{ConsoleController};

use std::thread::{JoinHandle, spawn};
use queues::{Queue, IsQueue};
use std::thread;
use std::time::Duration;
use console::Term;

/// Feature bit numbers
pub const VIRTIO_CONSOLE_F_SIZE: u16 = 0;
pub const VIRTIO_CONSOLE_F_MULTIPORT: u16 = 1;
pub const VIRTIO_CONSOLE_F_EMERG_WRITE: u16 = 2;

/// Virtio configuration
const QUEUE_SIZE: usize = 128;
const NUM_QUEUES: usize = 4;

/// Queues
const RX_QUEUE: u16 = 0;
const TX_QUEUE: u16 = 1;
const CTRL_RX_QUEUE: u16 = 2;
const CTRL_TX_QUEUE: u16 = 3;
const BACKEND_EFD: u16 = (NUM_QUEUES + 1) as u16;
const BACKEND_RX_EFD: u16 = (NUM_QUEUES + 2) as u16;


/// Console virtio control messages
const VIRTIO_CONSOLE_DEVICE_READY: u16 = 0 ;
const VIRTIO_CONSOLE_PORT_ADD: u16 = 1;
const VIRTIO_CONSOLE_PORT_REMOVE: u16 = 2; 
const VIRTIO_CONSOLE_PORT_READY: u16 = 3;
const VIRTIO_CONSOLE_CONSOLE_PORT: u16 = 4; 
const VIRTIO_CONSOLE_RESIZE: u16 = 5;
const VIRTIO_CONSOLE_PORT_OPEN: u16 = 6; 
const VIRTIO_CONSOLE_PORT_NAME: u16 = 7;

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, ThisError)]
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
	#[error("Failed to send notification")]
	NotificationFailed,
    #[error("Descriptor read failed")]
    DescriptorReadFailed,
    #[error("Descriptor write failed")]
    DescriptorWriteFailed,
    #[error("Failed to create new EventFd")]
    EventFdFailed,
    #[error("Failed to remove rx queue")]
    EmptyQueue,
}

impl convert::From<Error> for io::Error {
    fn from(e: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

/// Virtio Console Config
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub(crate) struct VirtioConsoleConfig {
    pub cols: Le16,
    pub rows: Le16,
    pub max_nr_ports: Le32,
    pub emerg_wr: Le32,
}

// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioConsoleConfig {}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub(crate) struct VirtioConsoleControl {
    pub id: Le32,
    pub event: Le16,
    pub value: Le16,
}

use std::io::Cursor;

impl VirtioConsoleControl {
    fn to_le_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

		buffer.extend_from_slice(&self.id.to_native().to_le_bytes());
		buffer.extend_from_slice(&self.event.to_native().to_le_bytes());
		buffer.extend_from_slice(&self.value.to_native().to_le_bytes());
        buffer
    }
}

// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioConsoleControl {}

pub(crate) struct VhostUserConsoleBackend {
    controller: Arc<RwLock<ConsoleController>>,
	acked_features: u64,
    event_idx: bool,
	rx_fifo: Queue<VirtioConsoleControl>,
    pub(crate) ready: bool,
    pub(crate) ready_to_write: bool,
	pub(crate) output_buffer: String,
    pub(crate) rx_event: EventFd,
    pub(crate) rx_ctrl_event: EventFd,
    pub(crate) exit_event: EventFd,
    mem: Option<GuestMemoryAtomic<GuestMemoryMmap>>,
}

type ConsoleDescriptorChain = DescriptorChain<GuestMemoryLoadGuard<GuestMemoryMmap<()>>>;

impl VhostUserConsoleBackend {
    pub(crate) fn new(controller: Arc<RwLock<ConsoleController>>) -> Result<Self> {
        Ok(VhostUserConsoleBackend {
            controller: controller,
            event_idx: false,
			rx_fifo: Queue::new(),
			acked_features: 0x0,
			ready: false,
			ready_to_write: false,
			output_buffer: String::new(),
            rx_event: EventFd::new(EFD_NONBLOCK).map_err(|_| Error::EventFdFailed)?,
            rx_ctrl_event: EventFd::new(EFD_NONBLOCK).map_err(|_| Error::EventFdFailed)?,
            exit_event: EventFd::new(EFD_NONBLOCK).map_err(|_| Error::EventFdFailed)?,
            mem: None,
        })
    }

	fn check_features (&self, features: u16) -> bool {
		(self.acked_features & (1 << features)) != 0
	}

    fn print_console_frame (&self, control_msg: VirtioConsoleControl) {
        println!("id 0x{:x}", control_msg.id.to_native());
        println!("event 0x{:x}", control_msg.event.to_native());
        println!("value 0x{:x}", control_msg.value.to_native());
    }

    fn process_rx_requests(
		&mut self,
		requests: Vec<ConsoleDescriptorChain>,
		vring: &VringRwLock
	) -> Result<bool> {
		log::trace!("process_rx_requests");

        if requests.is_empty() {
			log::trace!("requests.is_empty");
			vring.signal_used_queue();
            return Ok(true);
        }

		log::trace!("requests.len: {:?}", requests.len());
        let desc_chain = &requests[0];
        let descriptors: Vec<_> = desc_chain.clone().collect();

		log::trace!("descriptors.len(): {:?}", descriptors.len());
        if descriptors.len() != 1 {
			log::trace!("Error::UnexpectedDescriptorCount");
            return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
        }

        let desc_request = descriptors[0];
        if !desc_request.is_write_only() {
			log::trace!("!desc_request.is_write_only()");
            return Err(Error::UnexpectedReadableDescriptor(1));
        }

        // TODO: if buffer is more than the the desc_request length,
        //       write the remaining in the next chain.
		log::trace!("desc_request.len(): {}", desc_request.len());
		let response = self.output_buffer.clone();

        desc_chain
            .memory()
            .write_slice(response.as_bytes(), desc_request.addr())
            .map_err(|_| Error::DescriptorWriteFailed)?;

        if vring.add_used(desc_chain.head_index(), response.as_bytes().len() as u32).is_err() {
            warn!("Couldn't return used descriptors to the ring");
        }

        Ok(true)
    }

    fn process_tx_requests(
		&self,
		requests: Vec<ConsoleDescriptorChain>,
		vring: &VringRwLock
	) -> Result<bool> {
		log::trace!("process_tx_requests");

        if requests.is_empty() {
			log::trace!("requests.is_empty");
            return Ok(true);
        }

		log::trace!("requests.len: {:?}", requests.len());
        for desc_chain in requests {
            let descriptors: Vec<_> = desc_chain.clone().collect();

			log::trace!("descriptors.len(): {:?}", descriptors.len());
            if descriptors.len() != 1 {
				log::trace!("Error::UnexpectedDescriptorCount");
                return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
            }

            let desc_request = descriptors[0];
            if desc_request.is_write_only() {
				log::trace!("Error::UnexpectedReadOnlyDescriptor");
                return Err(Error::UnexpectedWriteOnlyDescriptor(0));
			}

			log::trace!("desc_request.len(): {}", desc_request.len());
			let desc_len = desc_request.len();

			let mut buffer = [0 as u8; 4096];

            let request = desc_chain
                .memory()
                .read_slice(&mut buffer, desc_request.addr())
                .map_err(|_| Error::DescriptorReadFailed)?;

			let new_buff = &buffer[0..desc_len as usize];

		    let my_string = String::from_utf8(new_buff.to_vec()).unwrap();
		    log::trace!("{}", my_string);
		    print!("{}", my_string);
    	    io::stdout().flush().unwrap(); // Ensure the prompt is displayed.

            if vring.add_used(desc_chain.head_index(), desc_request.len()).is_err() {
                log::trace!("Couldn't return used descriptors to the ring");
                warn!("Couldn't return used descriptors to the ring");
            }
        }

        Ok(true)
    }

    fn process_ctrl_rx_requests(
		&mut self,
		requests: Vec<ConsoleDescriptorChain>,
		vring: &VringRwLock,
	) -> Result<bool> {
		log::trace!("process_ctrl_rx_requests");

        if requests.is_empty() {
			log::trace!("requests.is_empty()");
            return Ok(true);
        }
		log::trace!("\trequests.len(): {}", requests.len());

        for desc_chain in requests {

			let descriptors: Vec<_> = desc_chain.clone().collect();

			log::trace!("descriptors.len(): {:?}", descriptors.len());
            if descriptors.len() < 1 {
				warn!("Error::UnexpectedDescriptorCount");
                return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
            }

			log::trace!("self.rx_fifo.size(): {}", self.rx_fifo.size());
			let ctrl_msg: VirtioConsoleControl = match self.rx_fifo.remove() {
				Ok(item) => item,         
				_ => {
						log::trace!("No rx elements");
						return Ok(false)
					 },
			};

            let desc_request = descriptors[0];
            if !desc_request.is_write_only() {
				warn!("Error::UnexpectedWriteOnlyDescriptor");
                return Err(Error::UnexpectedWriteOnlyDescriptor(0));
            }

            if (desc_request.len() as usize) < size_of::<VirtioConsoleControl>() {
				log::trace!("UnexpectedDescriptorSize, len = {:?}", desc_request.len());
                return Err(Error::UnexpectedDescriptorSize(
                    size_of::<VirtioConsoleControl>(),
                    desc_request.len(),
                ));
            }

			log::trace!("desc_request.len(): {}", desc_request.len());
			self.print_console_frame(ctrl_msg);

		    let mut buffer: Vec<u8> = Vec::new();
		    buffer.extend_from_slice(&ctrl_msg.to_le_bytes());
		
			if ctrl_msg.event.to_native() == VIRTIO_CONSOLE_PORT_NAME {
		    	let string_bytes = "org.fedoraproject.console.foo!".as_bytes();
		    	buffer.extend_from_slice(string_bytes);
			};

			desc_chain
        	    .memory()
        	    .write_slice(&buffer, desc_request.addr())
        	    .map_err(|_| Error::DescriptorWriteFailed)?;

            if vring.add_used(desc_chain.head_index(), desc_request.len()).is_err() {
                log::trace!("Couldn't return used descriptors to the ring");
                warn!("Couldn't return used descriptors to the ring");
            }
        }

        Ok(true)
    }

	fn handle_control_msg (
		&mut self,
		vring: &VringRwLock,
		ctrl_msg: VirtioConsoleControl
	) -> Result<()> {

		let mut ctrl_msg_reply = VirtioConsoleControl {
								id: 0.into(),
								event: 0.into(),
								value: 1.into(),
							};
		match ctrl_msg.event.to_native() {
			VIRTIO_CONSOLE_DEVICE_READY => {
				log::trace!("VIRTIO_CONSOLE_DEVICE_READY");
				self.ready = true;
				ctrl_msg_reply.event = VIRTIO_CONSOLE_PORT_ADD.into();
				self.rx_fifo.add(ctrl_msg_reply);
				self.process_ctrl_rx_queue(vring)?;
			},
		    VIRTIO_CONSOLE_PORT_READY => {
				log::trace!("VIRTIO_CONSOLE_PORT_READY");
				ctrl_msg_reply.event = VIRTIO_CONSOLE_CONSOLE_PORT.into();
				self.rx_fifo.add(ctrl_msg_reply.clone());
				self.process_ctrl_rx_queue(vring)?;

				ctrl_msg_reply.event = VIRTIO_CONSOLE_PORT_NAME.into();
				self.rx_fifo.add(ctrl_msg_reply.clone());
				self.process_ctrl_rx_queue(vring)?;

				ctrl_msg_reply.event = VIRTIO_CONSOLE_PORT_OPEN.into();
				self.rx_fifo.add(ctrl_msg_reply.clone());
				self.process_ctrl_rx_queue(vring)?;
			},
		    VIRTIO_CONSOLE_PORT_OPEN => {
				log::trace!("VIRTIO_CONSOLE_PORT_OPEN");
			},
		    _ => {
				log::trace!("Uknown control event");
				return Err(Error::HandleEventUnknown);
			}
		};
        Ok(())
	}

    fn process_ctrl_tx_requests(
		&mut self,
		requests: Vec<ConsoleDescriptorChain>,
		vring: &VringRwLock,
		rx_ctrl_vring: &VringRwLock
	) -> Result<bool> {
		log::trace!("process_ctrl_tx_requests");

        if requests.is_empty() {
			log::trace!("requests.is_empty()");
            return Ok(true);
        }

        for desc_chain in requests {
			let descriptors: Vec<_> = desc_chain.clone().collect();

            if descriptors.len() < 1 {
				warn!("Error::UnexpectedDescriptorCount");
                return Err(Error::UnexpectedDescriptorCount(descriptors.len()));
            }

			log::trace!("descriptors.len(): {:?}", descriptors.len());

            let desc_request = descriptors[0];
            if desc_request.is_write_only() {
				log::trace!("This is write only");
                return Err(Error::UnexpectedWriteOnlyDescriptor(0));
            } else {
				log::trace!("This is read only");
			}

            if desc_request.len() as usize != size_of::<VirtioConsoleControl>() {
				log::trace!("UnexpectedDescriptorSize, len = {:?}", desc_request.len());
                return Err(Error::UnexpectedDescriptorSize(
                    size_of::<VirtioConsoleControl>(),
                    desc_request.len(),
                ));
            }

			log::trace!("desc_request.len: {}", desc_request.len());

            let mut request = desc_chain
                .memory()
                .read_obj::<VirtioConsoleControl>(desc_request.addr())
                .map_err(|_| Error::DescriptorReadFailed)?;

			self.print_console_frame(request);

			self.handle_control_msg(rx_ctrl_vring, request);

			if let Some(event_fd) = rx_ctrl_vring.get_ref().get_kick() {
				self.rx_ctrl_event.write(1).unwrap();
			} else {
			    // Handle the case where `state` is `None`.
			    log::trace!("EventFd is not available.");
			}

            if vring.add_used(desc_chain.head_index(), desc_request.len()).is_err() {
                log::trace!("Couldn't return used descriptors to the ring");
                warn!("Couldn't return used descriptors to the ring");
            }
        }

        Ok(true)
    }

    /// Process the messages in the vring and dispatch replies
    fn process_rx_queue(&mut self, vring: &VringRwLock) -> Result<()> {
		log::trace!("process_rx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_rx_requests(requests, vring)? {
            // Send notification once all the requests are processed
			log::trace!("Send notification once all the requests of queue 0 are processed");
            vring
                .signal_used_queue()
                .map_err(|_| {
					log::trace!("NotificationFailed");
					Error::NotificationFailed
				})?;
        }
		Ok(())
    }

    /// Process the messages in the vring and dispatch replies
    fn process_tx_queue(&self, vring: &VringRwLock) -> Result<()> {
		log::trace!("process_tx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_tx_requests(requests, vring)? {
            // Send notification once all the requests are processed
			log::trace!("Send notification once all the requests of queue 1 are processed");
            vring
                .signal_used_queue()
                .map_err(|_| {
					log::trace!("signal_used_queue error");
					Error::NotificationFailed
				})?;
        }

        Ok(())
    }

    /// Process the messages in the vring and dispatch replies
    fn process_ctrl_rx_queue(&mut self, vring: &VringRwLock) -> Result<()> {
		log::trace!("process_ctrl_rx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_ctrl_rx_requests(requests, vring)? {
			log::trace!("Send notification once all the requests of queue 2 are processed");
            // Send notification once all the requests are processed
            vring
                .signal_used_queue()
                .map_err(|_| Error::NotificationFailed)?;
        }
		Ok(())
    }

    /// Process the messages in the vring and dispatch replies
    fn process_ctrl_tx_queue(&mut self, vring: &VringRwLock, rx_ctrl_vring: &VringRwLock) -> Result<()> {
		log::trace!("process_ctrl_tx_queue");
        let requests: Vec<_> = vring
            .get_mut()
            .get_queue_mut()
            .iter(self.mem.as_ref().unwrap().memory())
            .map_err(|_| Error::DescriptorNotFound)?
            .collect();

        if self.process_ctrl_tx_requests(requests, vring, rx_ctrl_vring)? {
            // Send notification once all the requests are processed
            vring
                .signal_used_queue()
                .map_err(|_| Error::NotificationFailed)?;
        }
		Ok(())
    }

    /// Set self's VringWorker.
    pub(crate) fn set_vring_worker(
		&self,
        vring_worker: &Arc<VringEpollHandler<Arc<RwLock<VhostUserConsoleBackend>>, VringRwLock, ()>>,
    ) {
		let rx_event_fd = self.rx_event.as_raw_fd();
		vring_worker
            .register_listener(
				rx_event_fd,
				EventSet::IN,
				u64::from(BACKEND_EFD))
            .unwrap();

		let rx_ctrl_event_fd = self.rx_ctrl_event.as_raw_fd();
		vring_worker
            .register_listener(
				rx_ctrl_event_fd,
				EventSet::IN,
				u64::from(BACKEND_RX_EFD))
            .unwrap();
    }

    /// Start console thread.
    pub(crate) fn start_console_thread(
		vhu_console: &Arc<RwLock<VhostUserConsoleBackend>>,
    ) {

		let vhu_console = Arc::clone(&vhu_console);
    	print!("Enter text and press Enter: ");

		// Spawn a new thread to handle input.
    	spawn( move || {
			loop {
                let ready = vhu_console.read().unwrap().ready_to_write;
                if ready {

                    let term = Term::stdout();
                    let character = term.read_char().unwrap();
				    log::trace!("You entered: {}", character);

                    // Pass the data to vhu_console and trigger an EventFd
				    vhu_console.write().unwrap().output_buffer = character.to_string();
            	    vhu_console.write().unwrap().rx_event.write(1).unwrap();
                }
			}
    	});
    }
}

/// VhostUserBackendMut trait methods
impl VhostUserBackendMut<VringRwLock, ()>
    for VhostUserConsoleBackend
{
    fn num_queues(&self) -> usize {
		log::trace!("num_queues: {:?}", NUM_QUEUES);
        NUM_QUEUES
    }

    fn max_queue_size(&self) -> usize {
		log::trace!("max_queue_size: {:?}", QUEUE_SIZE);
        QUEUE_SIZE
    }

    fn features(&self) -> u64 {
        // this matches the current libvhost defaults except VHOST_F_LOG_ALL
        let features = 1 << VIRTIO_F_VERSION_1
            | 1 << VIRTIO_F_NOTIFY_ON_EMPTY
            | 1 << VIRTIO_RING_F_EVENT_IDX
			| 1 << VIRTIO_CONSOLE_F_EMERG_WRITE
            | 1 << VIRTIO_RING_F_INDIRECT_DESC
			| 1 << VIRTIO_CONSOLE_F_MULTIPORT // This could be disabled
            | VhostUserVirtioFeatures::PROTOCOL_FEATURES.bits();

		log::trace!("vhu_can->features: {:x}", features);
		features
    }

	fn acked_features(&mut self, _features: u64) {
		log::trace!("\nacked_features: 0x{:x}\n", _features);
		self.acked_features = _features;
	}

    fn protocol_features(&self) -> VhostUserProtocolFeatures {
        let protocol_features = VhostUserProtocolFeatures::MQ
            | VhostUserProtocolFeatures::CONFIG
            | VhostUserProtocolFeatures::REPLY_ACK;

		log::trace!("protocol_features: {:x}", protocol_features);
		protocol_features
    }

    fn get_config(&self, offset: u32, size: u32) -> Vec<u8> {
        // SAFETY: The layout of the structure is fixed and can be initialized by
        // reading its content from byte array.
		log::trace!("vhu_can->get_config");
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
		log::trace!("update_memory\n");
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
		log::trace!("\nhandle_event:");

		if device_event == RX_QUEUE {
		    log::trace!("RX_QUEUE\n");
		    return Ok(false);
		};

		if device_event == CTRL_RX_QUEUE {
		    log::trace!("CTRL_RX_QUEUE\n");
			if !self.ready {
				return Ok(false);
			}
		};

		let vring = if device_event == BACKEND_EFD {
		    log::trace!("BACKEND_EFD\n");
		    &vrings[RX_QUEUE as usize]
		} else if device_event == BACKEND_RX_EFD {
		    log::trace!("BACKEND_RX_EFD\n");
		    &vrings[CTRL_RX_QUEUE as usize]
		} else {
			&vrings[device_event as usize]
		};

        if self.event_idx {
            // vm-virtio's Queue implementation only checks avail_index
            // once, so to properly support EVENT_IDX we need to keep
            // calling process_request_queue() until it stops finding
            // new requests on the queue.
            loop {
                vring.disable_notification().unwrap();
                match device_event {
                    TX_QUEUE => {
				        self.ready_to_write = true;
                        self.process_tx_queue(vring)
                    },
                    CTRL_RX_QUEUE => self.process_ctrl_rx_queue(vring),
					CTRL_TX_QUEUE => {
						let rx_ctrl_vring = &vrings[CTRL_RX_QUEUE as usize];
						self.process_ctrl_tx_queue(vring, rx_ctrl_vring)
					},
                    BACKEND_EFD => {
						let _ = self.rx_event.read();
						self.process_rx_queue(vring)
					},
                    BACKEND_RX_EFD => {
						let _ = self.rx_ctrl_event.read();
						self.process_ctrl_rx_queue(vring)
					},
                    _ => Err(Error::HandleEventUnknown.into()),
                }?;
                if !vring.enable_notification().unwrap() {
                    break;
                }
            }
        } else {                                                                           
            // Without EVENT_IDX, a single call is enough.                                 
            match device_event {
                TX_QUEUE => {
			        self.ready_to_write = true;
                    self.process_tx_queue(vring)
                },
                CTRL_RX_QUEUE => self.process_ctrl_rx_queue(vring),
				CTRL_TX_QUEUE => {
					let rx_ctrl_vring = &vrings[CTRL_RX_QUEUE as usize];
					self.process_ctrl_tx_queue(vring, rx_ctrl_vring)
				},
                BACKEND_EFD => {
					let _ = self.rx_event.read();
					self.process_rx_queue(vring)
				},
                BACKEND_RX_EFD => {
					let _ = self.rx_ctrl_event.read();
					self.process_ctrl_rx_queue(vring)
				},
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtio_console_control_byte_valued() {
        let control = VirtioConsoleControl {
            id: Le32::from(1),
            event: Le16::from(2),
            value: Le16::from(3),
        };

        let bytes = control.to_le_bytes();

        assert_eq!(bytes.len(), 10);
    }

    #[test]
    fn test_vhost_user_console_backend_creation() {
        let console_controller = Arc::new(RwLock::new(ConsoleController::new(String::from("test_console")).unwrap()));
        let vhost_user_console_backend = VhostUserConsoleBackend::new(console_controller).unwrap();

        assert_eq!(vhost_user_console_backend.acked_features, 0);
        assert_eq!(vhost_user_console_backend.event_idx, false);
        assert_eq!(vhost_user_console_backend.ready, false);
        assert_eq!(vhost_user_console_backend.ready_to_write, false);
        assert_eq!(vhost_user_console_backend.output_buffer, String::new());
    }
}
