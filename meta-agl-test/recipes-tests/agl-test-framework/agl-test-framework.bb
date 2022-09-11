SUMMARY = "Agl Test Framework"
HOMEPAGE = "https://git.automotivelinux.org/src/agl-test-framework/"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'agl-test-framework'
PV = '1'

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/agl-test-framework;protocol=https;branch=master"
SRCREV = "e35e4d16b0a231ed34a744bcb0a08c1278874df9"

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
    install -d ${D}/usr/AGL/agl-test/template/
    install -m 0644 ${WORKDIR}/git/template/* ${D}/usr/AGL/agl-test/template/

    install -d ${D}/usr/AGL/agl-test/tests/aio-stress/
    install -m 0644 ${WORKDIR}/git/tests/aio-stress/* ${D}/usr/AGL/agl-test/tests/aio-stress/
    install -d ${D}/usr/AGL/agl-test/tests/bzip2/
    install -m 0644 ${WORKDIR}/git/tests/bzip2/* ${D}/usr/AGL/agl-test/tests/bzip2/
    install -d ${D}/usr/AGL/agl-test/tests/expat/
    install -m 0644 ${WORKDIR}/git/tests/expat/* ${D}/usr/AGL/agl-test/tests/expat/
    install -d ${D}/usr/AGL/agl-test/tests/glib2/
    install -m 0644 ${WORKDIR}/git/tests/glib2/* ${D}/usr/AGL/agl-test/tests/glib2/
}

RDEPENDS:${PN} += " \
	python3-pytest \
	python3-jinja2 \
    python3-pytest-order \
"
