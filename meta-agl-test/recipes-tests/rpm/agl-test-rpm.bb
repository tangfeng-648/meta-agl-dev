SUMMARY = "Rpm Test Set"
HOMEPAGE = "https://git.automotivelinux.org/src/qa-test-misc/"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'agl-test-rpm'
PV = '1'

FILESEXTRAPATHS:append := "${TMPDIR}/work/aarch64-agl-linux/qa-test-misc/1-r0/git/:"

SRC_URI = "file://rpm/ \
           file://LICENSE "

do_fetch[depends] = "qa-test-misc:do_unpack"

S = "${WORKDIR}"

FILES:${PN} += " \
        /usr/AGL/agl-test/tests/rpm \
"

do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/rpm/resource/tests/
    install -m 0644 ${WORKDIR}/rpm/rootfs-scripts/*.py ${D}/usr/AGL/agl-test/tests/rpm/
    install -m 0644 ${WORKDIR}/rpm/rootfs-scripts/resource/rpm_test.sh ${D}/usr/AGL/agl-test/tests/rpm/resource/
    install -m 0644 ${WORKDIR}/rpm/rootfs-scripts/resource/test-manual-1.2.3.noarch.rpm ${D}/usr/AGL/agl-test/tests/rpm/resource/
    install -m 0644 ${WORKDIR}/rpm/rootfs-scripts/resource/tests/* ${D}/usr/AGL/agl-test/tests/rpm/resource/tests/
}

RDEPENDS:${PN} += " \
    agl-test-framework \
    rpm \
"
