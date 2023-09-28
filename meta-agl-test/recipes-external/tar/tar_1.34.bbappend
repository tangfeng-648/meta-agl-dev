# Add ptest support

# backport of yocto begin
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

# add for ptest support
SRC_URI += " \
    file://run-ptest \
    file://0001-tests-fix-TESTSUITE_AT.patch \
    file://0002-tests-check-for-recently-fixed-bug.patch \
    file://0003-Exclude-VCS-directory-with-writing-from-an-archive.patch \
"

inherit ptest

do_compile_ptest() {
    oe_runmake -C ${B}/gnu/ check
    oe_runmake -C ${B}/lib/ check
    oe_runmake -C ${B}/rmt/ check
    oe_runmake -C ${B}/src/ check
    rm -rf ${S}/tests/testsuite
    oe_runmake -C ${B}/tests/ testsuite
    oe_runmake -C ${B}/tests/ genfile checkseekhole ckmtime
}

do_install_ptest() {
    install -d ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/atconfig ${D}${PTEST_PATH}/tests/
    sed -i "/abs_/d" ${D}${PTEST_PATH}/tests/atconfig
    echo "abs_builddir=${PTEST_PATH}/tests/" >> ${D}${PTEST_PATH}/tests/atconfig
    install --mode=755 ${B}/tests/atlocal ${D}${PTEST_PATH}/tests/
    sed -i "/PATH=/d" ${D}${PTEST_PATH}/tests/atlocal
    install --mode=755 ${B}/tests/genfile ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/checkseekhole ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/ckmtime ${D}${PTEST_PATH}/tests/
    install --mode=755 ${S}/tests/testsuite ${D}${PTEST_PATH}/tests/
    sed -i "s#@PTEST_PATH@#${PTEST_PATH}#g" ${D}${PTEST_PATH}/run-ptest
}
# backport of yocto end

# add for agl-test-framework
do_install_ptest:append() {
    sed -i "s/\$at_am_msg: \$at_desc/\$at_am_msg: \$at_desc_line/g" ${D}${PTEST_PATH}/tests/testsuite
}
