SUMMARY     = "Momiyama home screen example"
DESCRIPTION = "The momiscreen is a home screen example. \
               The momiscreen is not require agl-appfw."
LICENSE = "GPL-3.0-only"
LIC_FILES_CHKSUM = "file://LICENSE;md5=d32239bcb673463ab874e80d47fae504"

DEPENDS = " \
    qtbase \
    qtquickcontrols2 \
    qtgraphicaleffects \
    qtsvg \
    "

PV = "1.0.0"

SRC_URI = "git://git.automotivelinux.org/apps/momiscreen;protocol=https;branch=${AGL_BRANCH} \
           file://momiscreen.service \
           file://momiscreen \
          "
SRCREV = "22b44f911bf6c53298055626cef671a24e9e9069"

S = "${WORKDIR}/git"

inherit qmake5 systemd

QT_INSTALL_PREFIX = "/usr"

do_install:append() {
	install -d ${D}/${systemd_unitdir}/system
	install -m 0644 ${WORKDIR}/momiscreen.service ${D}/${systemd_unitdir}/system

	install -m 0755 -d ${D}${sysconfdir}/default/
	install -m 0755 ${WORKDIR}/momiscreen ${D}${sysconfdir}/default/
}

FILES:${PN} += " \
    ${systemd_unitdir} \
    ${sysconfdir}/*/* \
    "
SYSTEMD_PACKAGES = "${PN}"
SYSTEMD_SERVICE:${PN} = "momiscreen.service"

RDEPENDS:${PN} = "qtsvg "
