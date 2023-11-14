// CAN backend device
//
// Copyright 2023 VIRTUAL OPEN SYSTEMS SAS. All Rights Reserved.
//          Timos Ampelikiotis <t.ampelikiotis@virtualopensystems.com>
//
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

use log::{warn, error};
use std::sync::{Arc, RwLock};

use thiserror::Error as ThisError;
use vm_memory::{ByteValued, Le16};

extern crate socketcan;
use socketcan::{
	CanFdSocket, CanFdFrame, CanAnyFrame, EmbeddedFrame, Socket,
	Frame, StandardId,
};

use std::thread::{JoinHandle, spawn};
use vmm_sys_util::eventfd::{EFD_NONBLOCK, EventFd};

extern crate queues;
use queues::*;

use crate::vhu_can::{VirtioCanFrame, VIRTIO_CAN_STATUS_OK};

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, ThisError)]
/// Errors related to low level gpio helpers
pub(crate) enum Error {
    //#[error("Can not enabled yet")]
    //CanNotEnabled,
    #[error("Can open socket operation failed")]
    CanSocketFailed,
    #[error("Can write socket operation failed")]
    CanSocketWriteFailed,
    #[error("Can read socket operation failed")]
    CanSocketReadFailed,
    #[error("Pop can element operation failed")]
    CanPopFailed,
    #[error("Creating Eventfd for CAN events failed")]
	CanEventFdFailed,
    #[error("CanQueueFailed")]
    CanQueueFailed,
}

/* CAN flags to determine type of CAN Id */
pub(crate) const VIRTIO_CAN_FLAGS_EXTENDED: u32 = 0x8000;
pub(crate) const VIRTIO_CAN_FLAGS_FD: u32 = 0x4000;
pub(crate) const VIRTIO_CAN_FLAGS_RTR: u32 = 0x2000;

pub(crate) const VIRTIO_CAN_TX: u16 = 0x0001;
pub(crate) const VIRTIO_CAN_RX: u16 = 0x0101;

pub(crate) const CAN_EFF_FLAG: u32 = 0x80000000; /* EFF/SFF is set in the MSB */
pub(crate) const CAN_RTR_FLAG: u32 = 0x40000000; /* remote transmission request */
pub(crate) const CAN_ERR_FLAG: u32 = 0x20000000; /* error message frame */

pub(crate) const CAN_SFF_MASK: u32 = 0x000007FF; /* standard frame format (SFF) */
pub(crate) const CAN_EFF_MASK: u32 = 0x1FFFFFFF; /* extended frame format (EFF) */

//pub(crate) const CAN_FRMF_BRS: u32 = 0x01; /* bit rate switch (2nd bitrate for data) */
//pub(crate) const CAN_FRMF_ESI: u32 = 0x02; /* error state ind. of transmitting node */
pub(crate) const CAN_FRMF_TYPE_FD: u32 = 0x10; /* internal bit ind. of CAN FD frame */
pub(crate) const CAN_ERR_BUSOFF: u32 = 0x00000040; /* bus off */

/// Virtio Can Configuration
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub(crate) struct VirtioCanConfig {
	/* CAN controller status */
    pub(crate) status: Le16,
}

// SAFETY: The layout of the structure is fixed and can be initialized by
// reading its content from byte array.
unsafe impl ByteValued for VirtioCanConfig {}

#[derive(Debug)]
pub(crate) struct CanController {
    config: VirtioCanConfig,
    pub can_in_name: String,
    pub can_out_name: String,
	can_out_socket: CanFdSocket,
	pub rx_event_fd: EventFd,
	rx_fifo: Queue<VirtioCanFrame>,
}

impl CanController {
    // Creates a new controller corresponding to `device`.
    pub(crate) fn new(can_in_name: String, can_out_name: String) -> Result<CanController> {

        let can_in_name = can_in_name.to_owned();
		println!("can_in_name: {:?}", can_in_name);

        let can_out_name = can_out_name.to_owned();
		println!("can_out_name: {:?}", can_out_name);

		let can_out_socket = Self::open_can_sockets(can_out_name.clone());

		let rx_fifo = Queue::new();

		let rx_efd = EventFd::new(EFD_NONBLOCK).map_err(|_| Error::CanEventFdFailed)?;

        Ok(CanController {
            config: VirtioCanConfig {
                status: 0x0.into(),
            },
            can_in_name,
            can_out_name,
			can_out_socket,
			rx_event_fd: rx_efd,
			rx_fifo
        })
    }

	pub fn print_can_frame (canframe: VirtioCanFrame) {
		println!("canframe.msg_type 0x{:x}", canframe.msg_type.to_native());
        println!("canframe.can_id 0x{:x}", canframe.can_id.to_native());
        println!("canframe.length {}", canframe.length.to_native());
        println!("canframe.flags 0x{:x}", canframe.flags.to_native());
		if canframe.length.to_native() == 0 {
			println!("[]");
			return;
		}
        print!("[");
        let last_elem = canframe.length.to_native() as usize - 1;
        for (index, sdu) in canframe.sdu.iter().enumerate() {
            print!("0x{:x}, ", sdu);
          if index == last_elem {
              print!("0x{:x}", sdu);
			  break;
          }
        }
        println!("]");
	}

	/* FIXME: This thread is not handle after termination */
	pub fn start_read_thread (controller: Arc<RwLock<CanController>>) -> JoinHandle<Result<()>> {
		spawn(move || {
				CanController::read_can_socket(controller)
			}
		)
	}

    pub fn push(&mut self, rx_elem: VirtioCanFrame) -> Result<()> {
		match self.rx_fifo.add(rx_elem) {
			Ok(_) => Ok(()), // Successfully added, so return Ok(())
			_ => Err(Error::CanQueueFailed), // Handle other errors
		}
    }

    pub fn pop(&mut self) -> Result<VirtioCanFrame> {
		match self.rx_fifo.remove() {
    	    Ok(item) => Ok(item),
    	    _ => Err(Error::CanPopFailed),
    	}
    }

	fn open_can_sockets (can_out_name: String) -> CanFdSocket {
	    let can_out_socket = match CanFdSocket::open(&can_out_name) {
			Ok(socket) => socket,
	        Err(_) => {
	            warn!("Error opening CAN socket");
	            panic!("Failed to open CAN socket.");
				//return Err(Error::CanSocketFailed);
	        }
	    };

		can_out_socket
	}

	pub fn read_can_socket (controller: Arc<RwLock<CanController>>) -> Result<()> {
		let can_in_name = &controller.read().unwrap().can_in_name.clone();
		dbg!("Start reading from {} socket!", &can_in_name);
	    let socket = match CanFdSocket::open(&can_in_name) {
			Ok(socket) => socket,
	        Err(_) => {
	            warn!("Error opening CAN socket");
				return Err(Error::CanSocketFailed);
	        }
	    };

		// Receive CAN messages
	    loop {
			if let Ok(frame) = socket.read_frame() {

				let mut controller = controller.write().unwrap();
			    match frame {
					CanAnyFrame::Normal(frame) => {
        			    // Regular CAN frame
        			    println!("Received CAN message: {:?}", frame);
        			}
        			CanAnyFrame::Fd(frame) => {
        			    // CAN FD frame
        			    println!("Received CAN FD message: {:?}", frame);

						let read_can_frame = VirtioCanFrame {
							msg_type: VIRTIO_CAN_RX.into(),
							can_id: frame.raw_id().into(),
							length:	(frame.data().len() as u16).into(),
							reserved: 0.into(),
							flags: frame.id_flags().bits().into(),
							sdu: {
								let mut sdu_data: [u8; 64] = [0; 64];
								for i in 0..frame.data().len() {
								    sdu_data[i] = frame.data()[i];
								}
							    sdu_data
							},
						};

						match controller.push(read_can_frame) {
							Ok(_) => warn!("New Can frame was received"),
	    				    Err(_) => {
	    				        warn!("Error read/push CAN frame");
								return Err(Error::CanSocketReadFailed);
	    				    }
	    				}
        			}
        			CanAnyFrame::Remote(frame) => {
        			    // Remote CAN frame
        			    println!("Received Remote CAN message: {:?}", frame);
        			}
        			CanAnyFrame::Error(frame) => {
        			    // Error frame
        			    println!("Received Error frame: {:?}", frame);
        			}
			    }

				controller.rx_event_fd.write(1).unwrap();
	        }
	    }
	}

    pub(crate) fn config(&self) -> &VirtioCanConfig {
		log::trace!("Get config\n");
        &self.config
    }

    pub(crate) fn operation(&self, tx_request: VirtioCanFrame) -> Result<u8> {
		log::trace!("Can operation\n");

	    // Create a CAN frame with a specific CAN-ID and the data buffer
		let can_id = StandardId::new(tx_request.can_id.to_native().try_into().unwrap()).unwrap();
		let data_len = tx_request.length.to_native() as usize;

		let data: Vec<u8> = tx_request.sdu.iter().cloned().take(data_len).collect();
	    let frame = CanFdFrame::new(can_id, &data).unwrap();

	    // Send the CAN frame
	    let write_result = self.can_out_socket.write_frame(&frame); 
	    match write_result {
			Ok(_) => Ok(VIRTIO_CAN_STATUS_OK),
	        Err(_) => {
	            warn!("Error write CAN socket");
				Err(Error::CanSocketWriteFailed)
	        }
	    }
    }
}

