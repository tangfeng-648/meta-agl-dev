// VIRTIO CONSOLE Emulation via vhost-user
//
// Copyright 2023 VIRTUAL OPEN SYSTEMS SAS. All Rights Reserved.
//          Timos Ampelikiotis <t.ampelikiotis@virtualopensystems.com>
//
// SPDX-License-Identifier: Apache-2.0 or BSD-3-Clause

#[cfg(target_env = "gnu")]
mod backend;
#[cfg(target_env = "gnu")]
mod console;
#[cfg(target_env = "gnu")]
mod vhu_console;

#[cfg(target_env = "gnu")]
fn main() {
    backend::console_init()
}
