SUMMARY = "Agl Test Framework"
HOMEPAGE = "https://git.automotivelinux.org/src/agl-test-framework/"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'agl-test-framework'
PV = '1'

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/agl-test-framework;protocol=https;branch=master"
SRCREV = "50df2b6904dc448c2a8c5ed4f974c631fd97b070"

S = "${WORKDIR}/git"

FILES:${PN} += " \
        /usr/AGL/agl-test/ \
"

do_install() {
    install -d ${D}/usr/bin/
    install -m 0755 ${WORKDIR}/git/agl-test ${D}/usr/bin/
    install -d ${D}/usr/AGL/agl-test/plugins/
    install -m 0644 ${WORKDIR}/git/pytest.ini ${D}/usr/AGL/agl-test/
    install -m 0644 ${WORKDIR}/git/conftest.py ${D}/usr/AGL/agl-test/
    install -m 0644 ${WORKDIR}/git/plugins/* ${D}/usr/AGL/agl-test/plugins/
    install -d ${D}/usr/AGL/agl-test/template/
    install -m 0644 ${WORKDIR}/git/template/* ${D}/usr/AGL/agl-test/template/
}

RDEPENDS:${PN} += " \
	python3-pytest \
	python3-jinja2 \
    python3-pytest-order \
"
