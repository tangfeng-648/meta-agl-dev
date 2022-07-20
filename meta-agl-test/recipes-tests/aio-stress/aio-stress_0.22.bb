SUMMARY = "Aio Stress Test"
HOMEPAGE = "https://oss.oracle.com/~mason/aio-stress/"
LICENSE = "GPL-2.0-only"
SRC_URI[sha256sum] = "3f4cffcc946fb717fff9d8fe932c7c2ee606efff198408d9fbe16955151445f7"
LIC_FILES_CHKSUM = "file://aio-stress.c;md5=ccb5d196a3736bbd835d582a4e2329c3"

PN = 'aio-stress'

DEPENDS = "libaio"

TARGET_CC_ARCH += "${LDFLAGS}"

SRC_URI = "https://oss.oracle.com/~mason/aio-stress/aio-stress.c \
           file://Makefile "

S = "${WORKDIR}"

FILES:${PN} += " \
    /usr/AGL/agl-test/tests/aio-stress/resource \
"

do_compile() {
    oe_runmake
}

do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/aio-stress/resource/
    install -m 0755 ${WORKDIR}/aio-stress ${D}/usr/AGL/agl-test/tests/aio-stress/resource/
}

RDEPENDS:${PN} += " \
    agl-test-framework \
"
