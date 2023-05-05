# Add ptest support
# backport of yocto
# http://cgit.openembedded.org/openembedded-core/commit/?id=2ee144a0bfb88823bfa788697bb7afc9a572c413
#
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://run-ptest \
"

inherit ptest

RDEPENDS:${PN}-ptest += "bash"

do_compile_ptest() {
        oe_runmake -C ${B}/tests/
}

do_install_ptest() {
        install -d ${D}${PTEST_PATH}/tests/
        install --mode=755 ${B}/tests/frametest ${D}${PTEST_PATH}/tests/
        sed -i "s#@PTEST_PATH@#${PTEST_PATH}#g" ${D}${PTEST_PATH}/run-ptest

}


