SUMMARY = "Virtio-loopback-adapter application"
DESCRIPTION = "Adapter bridge for virtio-loopback"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://README.md;md5=ecc9c54ada6f0c33054d3bde010744f7"

FILESEXTRAPATHS:prepend := "${THISDIR}/${PN}-${PV}:"
SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/virtio/adapter_app;protocol=https;branch=master"
SRCREV = "5810ae7ac9e5e1526f4d64c11f0c28ee2ee8f1a5"

S = "${WORKDIR}/git"
TARGET_CC_ARCH += "${LDFLAGS}"

do_compile() {
	cd ${S}
	make
}

do_install() {
	mkdir ${D}/usr/bin/ -p
	install -m 0755 ${S}/adapter ${D}/usr/bin/virtio-loopback-adapter
}

DEPENDS = ""
FILES:${PN} += "/usr/bin/virtio-loopback-adapter"
