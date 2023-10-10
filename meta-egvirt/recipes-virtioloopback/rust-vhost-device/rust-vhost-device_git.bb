SUMMARY = "VHOST device in rust"
HOMEPAGE = "https://git.virtualopensystems.com/virtio-loopback/vhost-user-rng-rust/vhost-device"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM="file://README.md;md5=b92c66b94b87c250f37ea0234c9e2d9a"

inherit cargo

SRC_URI = "git://git.virtualopensystems.com/virtio-loopback/vhost-user-rng-rust/vhost-device.git;protocol=https;rev=de0c5cf1ceb079992f7680ffc7f1f04447b869e2"
SRC_URI += "git://git.virtualopensystems.com/virtio-loopback/vhost-user-rng-rust/vhost.git;protocol=https;rev=89fcc56b86b1f165c5b00144df26430a07e1a051;destsuffix=vhost"
SRC_URI += "git://git.virtualopensystems.com/virtio-loopback/vhost-user-rng-rust/vhost-user-backend.git;protocol=https;rev=78ce5fc3ba73e5ac359dcd030b1e8b08ee7e9f2c;destsuffix=vhost-user-backend"
SRC_URI += "git://git.virtualopensystems.com/virtio-loopback/vhost-user-rng-rust/vm-virtio.git;protocol=https;rev=edb16fd8900c14c17c3ab781160aac87813d3b4a;destsuffix=vm-virtio"

S = "${WORKDIR}/git"

BBCLASSEXTEND = "native"
CARGO_DISABLE_BITBAKE_VENDORING = "1"
