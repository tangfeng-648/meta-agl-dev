SUMMARY = "Agl Test Framework"
HOMEPAGE = "https://git.automotivelinux.org/src/agl-test-framework/"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'agl-test-framework'
PV = '1'

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/agl-test-framework;protocol=https;branch=master"
SRCREV = "aa5fab53993f29311f1aef83488eb0f759dabca8"

S = "${WORKDIR}/git"

FILES:${PN} += " \
        /usr/AGL/agl-test/ \
"

do_install:append() {
    install -d ${D}/usr/bin/
    install -m 0755 ${WORKDIR}/git/agl-test ${D}/usr/bin/
    install -d ${D}/usr/AGL/agl-test/plugins/
    install -m 0644 ${WORKDIR}/git/pytest.ini ${D}/usr/AGL/agl-test/
    install -m 0644 ${WORKDIR}/git/conftest.py ${D}/usr/AGL/agl-test/
    install -m 0644 ${WORKDIR}/git/plugins/* ${D}/usr/AGL/agl-test/plugins/
}

RDEPENDS:${PN} += " \
	python3-pytest \
	python3-jinja2 \
"
