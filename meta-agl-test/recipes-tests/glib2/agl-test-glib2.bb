SUMMARY = "Glib2 Test"
HOMEPAGE = "https://git.automotivelinux.org/src/qa-test-misc/"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'agl-test-glib2'
PV = '1'

FILESEXTRAPATHS:append := "${TMPDIR}/work/aarch64-agl-linux/qa-test-misc/1-r0/git/:"
SRC_URI = "file://glib2/ \
           file://LICENSE "

do_fetch[depends] = "qa-test-misc:do_unpack"

S = "${WORKDIR}"

FILES:${PN} += " \
        /usr/AGL/agl-test/tests/glib2 \
"

do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/glib2/
    install -m 0644 ${WORKDIR}/glib2/rootfs-scripts/*.py ${D}/usr/AGL/agl-test/tests/glib2/
}

RDEPENDS:${PN} += " \
    agl-test-framework \
    glib-2.0 \
"
