SUMMARY = "linus_stress test"
HOMEPAGE = "https://chromium.googlesource.com/chromiumos/third_party/autotest/+/master/client/tests/linus_stress"
LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://${WORKDIR}/git/linus_stress/LICENSE;md5=b234ee4d69f5fce4486a80fdaf4a4263"

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/qa-test-misc;protocol=https;branch=${AGL_BRANCH}"
SRCREV = "87cdfd4626c0cb47fc22f328867e49d6268df85c"

S = "${WORKDIR}/git/linus_stress"

do_install() {
    install -d ${D}/usr/AGL/agl-test/tests/linus_stress/resource/
    install -m 0755 ${S}/linus_stress ${D}/usr/AGL/agl-test/tests/linus_stress/resource/
}

INSANE_SKIP:${PN} = "ldflags"

FILES:${PN} += " \
    /usr/AGL/agl-test/tests/linus_stress/resource \
"
