# Add ptest support
# backport of yocto
#
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://run-ptest \
"

inherit ptest

do_compile_ptest(){
    oe_runmake -C ${B}/tests/ check TESTS=
}

do_install_ptest() {
    install -d ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/.libs/* ${D}${PTEST_PATH}/tests/

    install -d ${D}${PTEST_PATH}/tests/files/
    install --mode=755 ${S}/tests/files/* ${D}${PTEST_PATH}/tests/files/
}

RDEPENDS:${PN}-ptest += "bash"

