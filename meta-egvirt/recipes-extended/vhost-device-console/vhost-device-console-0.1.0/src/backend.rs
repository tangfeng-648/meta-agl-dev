// VIRTIO CONSOLE Emulation via vhost-user
//
// Copyright 2023 VIRTUAL OPEN SYSTEMS SAS. All Rights Reserved.
//          Timos Ampelikiotis <t.ampelikiotis@virtualopensystems.com>
//
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

use log::{error, info, warn};
use std::process::exit;
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

use clap::Parser;
use thiserror::Error as ThisError;
use vhost::{vhost_user, vhost_user::Listener};
use vhost_user_backend::VhostUserDaemon;
use vm_memory::{GuestMemoryAtomic, GuestMemoryMmap};

use crate::console::{ConsoleController};
use crate::vhu_console::VhostUserConsoleBackend;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
/// Errors related to low level Console helpers
pub(crate) enum Error {
    #[error("Invalid socket count: {0}")]
    SocketCountInvalid(usize),
    #[error("Failed to join threads")]
    FailedJoiningThreads,
    #[error("Could not create console controller: {0}")]
    CouldNotCreateConsoleController(crate::console::Error),
    #[error("Could not create console backend: {0}")]
    CouldNotCreateBackend(crate::vhu_console::Error),
    #[error("Could not create daemon: {0}")]
    CouldNotCreateDaemon(vhost_user_backend::Error),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ConsoleArgs {
    /// Location of vhost-user Unix domain socket. This is suffixed by 0,1,2..socket_count-1.
    #[clap(short, long)]
    socket_path: String,

    /// A console device name to be used for reading (ex. vconsole, console0, console1, ... etc.)
    #[clap(short = 'i', long)]
    console_path: String,

    /// Number of guests (sockets) to connect to.
    #[clap(short = 'c', long, default_value_t = 1)]
    socket_count: u32,
}

#[derive(PartialEq, Debug)]
struct ConsoleConfiguration {
    socket_path: String,
    socket_count: u32,
    console_path: String,
}

impl TryFrom<ConsoleArgs> for ConsoleConfiguration {
    type Error = Error;

    fn try_from(args: ConsoleArgs) -> Result<Self> {

        if args.socket_count == 0 {
            return Err(Error::SocketCountInvalid(0));
        }

		let console_path = args.console_path.trim().to_string();

        Ok(ConsoleConfiguration {
            socket_path: args.socket_path,
			socket_count: args.socket_count,
            console_path,
        })
    }
}

fn start_backend(args: ConsoleArgs) -> Result<()> {

	println!("start_backend function!\n");

    let config = ConsoleConfiguration::try_from(args).unwrap();
    let mut handles = Vec::new();

    for _ in 0..config.socket_count {
        let socket = config.socket_path.to_owned();
        let console_path = config.console_path.to_owned();

        let handle: JoinHandle<Result<()>> = spawn(move || loop {
            // A separate thread is spawned for each socket and console connect to a separate guest.
            // These are run in an infinite loop to not require the daemon to be restarted once a
            // guest exits.
            //
            // There isn't much value in complicating code here to return an error from the
            // threads, and so the code uses unwrap() instead. The panic on a thread won't cause
            // trouble to other threads/guests or the main() function and should be safe for the
            // daemon.

            let controller =
                ConsoleController::new(console_path.clone()).map_err(Error::CouldNotCreateConsoleController)?;
			let arc_controller =  Arc::new(RwLock::new(controller));
            let vu_console_backend = Arc::new(RwLock::new(
                VhostUserConsoleBackend::new(arc_controller).map_err(Error::CouldNotCreateBackend)?,
            ));

            let mut daemon = VhostUserDaemon::new(
                String::from("vhost-device-console-backend"),
                vu_console_backend.clone(),
                GuestMemoryAtomic::new(GuestMemoryMmap::new()),
            )
            .map_err(Error::CouldNotCreateDaemon)?;

            /* Start the read thread -- need to handle it after termination */
            let vring_workers = daemon.get_epoll_handlers();
            vu_console_backend.read()
                          .unwrap()
                          .set_vring_worker(&vring_workers[0]);
			VhostUserConsoleBackend::start_console_thread(&vu_console_backend);

            let listener = Listener::new(socket.clone(), true).unwrap();
            daemon.start(listener).unwrap();

            match daemon.wait() {
                Ok(()) => {
                    info!("Stopping cleanly.");
                }
                Err(vhost_user_backend::Error::HandleRequest(
                    vhost_user::Error::PartialMessage | vhost_user::Error::Disconnected,
                )) => {
                    info!("vhost-user connection closed with partial message. If the VM is shutting down, this is expected behavior; otherwise, it might be a bug.");
                }
                Err(e) => {
                    warn!("Error running daemon: {:?}", e);
                }
            }

            // No matter the result, we need to shut down the worker thread.
            vu_console_backend.read().unwrap().exit_event.write(1).unwrap();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| Error::FailedJoiningThreads)??;
    }

    Ok(())
}

pub(crate) fn console_init() {
    env_logger::init();
	println!("Console_init function!");
    if let Err(e) = start_backend(ConsoleArgs::parse()) {
        error!("{e}");
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_configuration_try_from_valid_args() {
        let args = ConsoleArgs {
            socket_path: String::from("/path/to/socket"),
            console_path: String::from("vconsole"),
            socket_count: 3,
        };

        let result = ConsoleConfiguration::try_from(args);

        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.socket_path, "/path/to/socket");
        assert_eq!(config.console_path, "vconsole");
        assert_eq!(config.socket_count, 3);
    }

    #[test]
    fn test_console_configuration_try_from_invalid_args() {
        // Test with socket_count = 0
        let args_invalid_count = ConsoleArgs {
            socket_path: String::from("/path/to/socket"),
            console_path: String::from("vconsole"),
            socket_count: 0,
        };

        let result_invalid_count = ConsoleConfiguration::try_from(args_invalid_count);
        assert!(result_invalid_count.is_err());
    }

    #[test]
    fn test_start_backend_success() {
        // Test start_backend with valid arguments
        let args = ConsoleArgs {
            socket_path: String::from("/path/to/socket"),
            console_path: String::from("vconsole"),
            socket_count: 2,
        };

        let result = start_backend(args);

        assert!(result.is_ok());
    }
}
