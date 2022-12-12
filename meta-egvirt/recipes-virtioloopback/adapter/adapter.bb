SUMMARY = "Adapter application"
DESCRIPTION = "Application description here"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://README.md;md5=ecc9c54ada6f0c33054d3bde010744f7"

FILESEXTRAPATHS:prepend := "${THISDIR}/${PN}-${PV}:"
SRC_URI = "git://git.virtualopensystems.com/virtio-loopback/adapter_app.git;protocol=https;rev=da82d88bd72cc9e11b4df11d0b594d3554a75d4f;branch=beta-release"
S = "${WORKDIR}/git"

do_compile() {
	cd ${S}
	make
}

do_install() {
	mkdir ${D}/usr/bin/ -p
	install -m 0755 ${S}/adapter ${D}/usr/bin/
}

DEPENDS = ""
FILES:${PN} += "/usr/bin/adapter"
