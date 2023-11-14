// VIRTIO CAN Emulation via vhost-user
//
// Copyright 2023 VIRTUAL OPEN SYSTEMS SAS. All Rights Reserved.
//          Timos Ampelikiotis <t.ampelikiotis@virtualopensystems.com>
//
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

#[cfg(target_env = "gnu")]
mod backend;
#[cfg(target_env = "gnu")]
mod can;
#[cfg(target_env = "gnu")]
mod vhu_can;

#[cfg(target_env = "gnu")]
fn main() {
	println!("Hello to main vhost-user-can");
    backend::can_init()
}

// Rust vmm container (https://github.com/rust-vmm/rust-vmm-container) doesn't
// have tools to do a musl build at the moment, and adding that support is
// tricky as well to the container. Skip musl builds until the time pre-built
// libgpiod library is available for musl.
#[cfg(target_env = "musl")]
fn main() {}
