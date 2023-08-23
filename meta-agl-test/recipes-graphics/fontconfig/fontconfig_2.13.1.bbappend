# Add ptest support
# backport of yocto
#
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://run-ptest \
    file://ptest.patch \
"

inherit ptest

do_install_ptest() {
    install ${S}/test-driver ${D}${PTEST_PATH}/

    install -d ${D}${PTEST_PATH}/test/
    find ${B}/test/.libs/ -type f -not -name "test-migration" \
        -exec install {} ${D}${PTEST_PATH}/test/ \;
    install ${S}/test/*.pcf ${D}${PTEST_PATH}/test

    # update PTEST_PATH in run-ptest
    sed -i "s#@PTEST_PATH@#${PTEST_PATH}#g" ${D}${PTEST_PATH}/run-ptest
}

