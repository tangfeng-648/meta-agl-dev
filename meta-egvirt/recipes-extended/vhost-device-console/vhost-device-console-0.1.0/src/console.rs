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

use crate::vhu_console::{VirtioConsoleConfig, VirtioConsoleControl};

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, ThisError)]
pub(crate) enum Error {
    #[error("Console not enabled yet")]
    ConsoleNotEnabled,
}

#[derive(Debug)]
pub(crate) struct ConsoleController {
    config: VirtioConsoleConfig,
    pub console_name: String,
}

impl ConsoleController {
    // Creates a new controller corresponding to `device`.
    pub(crate) fn new(console_name: String) -> Result<ConsoleController> {

        let console_name = console_name.to_owned();
		println!("console_name: {:?}", console_name);

        Ok(ConsoleController {
            config: VirtioConsoleConfig {
						cols: 20.into(),
						rows: 20.into(),
						max_nr_ports: 1.into(),
						emerg_wr: 64.into(),
					},
            console_name,
        })
    }

    pub(crate) fn config(&self) -> &VirtioConsoleConfig {
		log::trace!("Get config\n");
        &self.config
    }

    pub(crate) fn operation(&self, tx_request: VirtioConsoleControl) -> Result<()> {
		log::trace!("Console operation\n");
		Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_console_controller() {
        let console_name = String::from("test_console");
        let controller = ConsoleController::new(console_name.clone());

        assert!(controller.is_ok());

        let controller = controller.unwrap();
        assert_eq!(controller.console_name, "test_console");
    }

    #[test]
    fn test_console_controller_config() {
        let console_name = String::from("test_console");
        let controller = ConsoleController::new(console_name).unwrap();

        let config = controller.config();
        assert_eq!(config.cols, 20);
        assert_eq!(config.rows, 20);
        assert_eq!(config.max_nr_ports, 1);
        assert_eq!(config.emerg_wr, 64);
    }
}

