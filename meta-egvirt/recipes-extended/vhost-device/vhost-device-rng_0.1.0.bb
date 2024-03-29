SUMMARY = "vhost rng backend device"
DESCRIPTION = "A vhost-user backend that emulates a VirtIO random number \
    generator device"
HOMEPAGE = "https://github.com/rust-vmm/vhost-device"
LICENSE = "Apache-2.0 | BSD-3-Clause"
LIC_FILES_CHKSUM = " \
    file://LICENSE-APACHE;md5=3b83ef96387f14655fc854ddc3c6bd57 \
    file://LICENSE-BSD-3-Clause;md5=2489db1359f496fff34bd393df63947e \
"

SRC_URI += "crate://crates.io/vhost-device-rng/0.1.0"
SRC_URI[vhost-device-rng-0.1.0.sha256sum] = "51e0af4acaaefc19a3d6c2e2cb00caf2130d728d857494c857662781da9f1329"

inherit cargo

include vhost-device-rng-crates.inc
