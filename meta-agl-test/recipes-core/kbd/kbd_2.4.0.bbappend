# Add ptest support
# backport of yocto
#
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://run-ptest \
"

inherit ptest

do_compile_ptest() {
    # update DATADIR in Makefile
    sed -i 's,-DDATADIR=.*,-DDATADIR=\\\"${PTEST_PATH}/tests\\\" \\,g' ${B}/tests/libkeymap/Makefile
    sed -i 's,-DDATADIR=.*,-DDATADIR=\\\"${PTEST_PATH}/tests\\\" \\,g' ${B}/tests/helpers/Makefile
    sed -i 's,-DDATADIR=.*,-DDATADIR=\\\"${PTEST_PATH}/tests\\\" \\,g' ${B}/tests/libkbdfile/Makefile

    # unset -D_TIME_BITS=64 in Makefile
    sed -i 's,-D_TIME_BITS=64,,g' ${B}/tests/libtswrap/Makefile

    # recompile tests
    oe_runmake -C ${B}/tests/ clean
    oe_runmake -C ${B}/tests/
}

do_install_ptest() {
    # install files from build directory
    install -d ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/atconfig ${D}${PTEST_PATH}/tests/
    install --mode=755 ${B}/tests/testsuite ${D}${PTEST_PATH}/tests/
    install -d ${D}${PTEST_PATH}/tests/libkeymap/
    find ${B}/tests/libkeymap/ -type f -not -name "*.o" -not -name "Makefile" \
        -exec install --mode=755 {} ${D}${PTEST_PATH}/tests/libkeymap/ \;
    install -d ${D}${PTEST_PATH}/tests/helpers/
    find ${B}/tests/helpers/ -type f -not -name "*.o" -not -name "Makefile" \
        -exec install --mode=755 {} ${D}${PTEST_PATH}/tests/helpers/ \;
    install -d ${D}${PTEST_PATH}/tests/libkbdfile/
    find ${B}/tests/libkbdfile/ -type f -not -name "*.o" -not -name "Makefile" \
        -exec install --mode=755 {} ${D}${PTEST_PATH}/tests/libkbdfile/ \;
    install -d ${D}${PTEST_PATH}/src/
    install --mode=755 ${B}/src/loadkeys ${D}${PTEST_PATH}/src/

    # install files from src/data directory
    install -d ${D}${PTEST_PATH}/data/keymaps/i386/qwerty/
    install ${S}/data/keymaps/i386/qwerty/defkeymap.map ${D}${PTEST_PATH}/data/keymaps/i386/qwerty/

    # install files from src/tests/data directory
    install -d ${D}${PTEST_PATH}/tests/data/
    install -d ${D}${PTEST_PATH}/tests/data/libkeymap/
    install ${S}/tests/data/libkeymap/* ${D}${PTEST_PATH}/tests/data/libkeymap/
    install -d ${D}${PTEST_PATH}/tests/data/alt-is-meta/
    install ${S}/tests/data/alt-is-meta/* ${D}${PTEST_PATH}/tests/data/alt-is-meta/
    install -d ${D}${PTEST_PATH}/tests/data/bkeymap-2.0.4/
    install ${S}/tests/data/bkeymap-2.0.4/* ${D}${PTEST_PATH}/tests/data/bkeymap-2.0.4/
    install -d ${D}${PTEST_PATH}/tests/data/dumpkeys-mktable/
    install ${S}/tests/data/dumpkeys-mktable/* ${D}${PTEST_PATH}/tests/data/dumpkeys-mktable/
    install -d ${D}${PTEST_PATH}/tests/data/dumpkeys-fulltable/
    install ${S}/tests/data/dumpkeys-fulltable/* ${D}${PTEST_PATH}/tests/data/dumpkeys-fulltable/
    install -d ${D}${PTEST_PATH}/tests/data/findfile/test_1/consolefonts/
    install ${S}/tests/data/findfile/test_1/consolefonts/* ${D}${PTEST_PATH}/tests/data/findfile/test_1/consolefonts/
    install -d ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/include/
    install ${S}/tests/data/findfile/test_0/keymaps/test0.map ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/
    install ${S}/tests/data/findfile/test_0/keymaps/include/* ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/include/
    install -d ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/include/
    install ${S}/tests/data/findfile/test_0/keymaps/i386/include/* ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/include/
    install -d ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/qwerty/
    install ${S}/tests/data/findfile/test_0/keymaps/i386/qwerty/* ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/qwerty/
    install -d ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/qwertz/
    install ${S}/tests/data/findfile/test_0/keymaps/i386/qwertz/* ${D}${PTEST_PATH}/tests/data/findfile/test_0/keymaps/i386/qwertz/


    # update PTEST_PATH in run-ptest
    sed -i "s#@PTEST_PATH@#${PTEST_PATH}#g" ${D}${PTEST_PATH}/run-ptest
    sed -i -e 's,${B},${PTEST_PATH},g' -e 's,/\.\./kbd-2.4.0,,g' ${D}${PTEST_PATH}/tests/atconfig
}

