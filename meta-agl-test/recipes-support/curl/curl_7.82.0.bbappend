# Add ptest support
# backport of yocto
# http://cgit.openembedded.org/openembedded-core/commit/meta/recipes-support/curl?id=a0ea00daace826129cdec8f714ca7b7c60e9dadf
# http://cgit.openembedded.org/openembedded-core/commit/meta/recipes-support/curl?id=0b1e3746478e9ad1800b027ab5dc96495997807e
# http://cgit.openembedded.org/openembedded-core/commit/meta/recipes-support/curl?id=e885875f2af9cee0e7557ee130d3180492e507dd
#
FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += " \
    file://run-ptest \
    file://disable-tests \
"

inherit ptest

do_compile_ptest() {
    oe_runmake test
    oe_runmake -C ${B}/tests/server
}

do_install_ptest() {
    cat ${WORKDIR}/disable-tests >> ${S}/tests/data/DISABLED
    rm -f ${B}/tests/configurehelp.pm
    cp -rf ${B}/tests ${D}${PTEST_PATH}
    cp -rf ${S}/tests ${D}${PTEST_PATH}
    find ${D}${PTEST_PATH}/ -type f -name Makefile.am -o -name Makefile.in -o -name Makefile -delete
    install -d ${D}${PTEST_PATH}/src
    ln -sf ${bindir}/curl ${D}${PTEST_PATH}/src/curl
    cp -rf ${D}${bindir}/curl-config ${D}${PTEST_PATH}
}

RDEPENDS:${PN}-ptest += "bash \
    perl-modules \
    perl-module-time-hires \
    perl-module-digest-md5 \
    perl-module-digest \
    perl-module-ipc-open2"


