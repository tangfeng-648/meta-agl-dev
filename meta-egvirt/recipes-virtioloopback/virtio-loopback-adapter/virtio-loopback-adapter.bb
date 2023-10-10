SUMMARY = "Virtio-loopback-adapter application"
DESCRIPTION = "Adapter bridge for virtio-loopback"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://README.md;md5=ecc9c54ada6f0c33054d3bde010744f7"

FILESEXTRAPATHS:prepend := "${THISDIR}/${PN}-${PV}:"
SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/virtio/adapter_app;protocol=https;branch=master"
SRCREV = "ddc98ee7562fa01abbd4d1b027efd5e98ceb93bf"

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
