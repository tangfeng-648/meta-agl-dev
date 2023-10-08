SUMMARY = "An in-car wireless monitor demo."
HOMEPAGE = "https://git.automotivelinux.org/staging/incar-wireless-monitor/"
LICENSE = "Apache-2.0"

S = "${WORKDIR}/git"

LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

PN = 'incar-wireless-monitor'
PV = '1'

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/staging/incar-wireless-monitor;protocol=https;branch=${AGL_BRANCH}"
SRCREV = "927829c95bac0874b07b0aaee1663f42f5e59530"

RDEPENDS:${PN} += "boa"
RDEPENDS:${PN} += "mjpg-streamer"
RDEPENDS:${PN} += "libcgic"

do_install() {
    install -d ${D}/etc/boa/www/
    install -m 0755 ${WORKDIR}/git/ui/www/*.html ${D}/etc/boa/www/
}
