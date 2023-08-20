SUMMARY     = "Momiyama navigation example based on mapbox."
DESCRIPTION = "The mominavi is a navigation example based on mapbox. It's based on aglqtnavigation. \
               The mominavi is not require agl-appfw."
LICENSE = "GPL-3.0-only"
LIC_FILES_CHKSUM = "file://LICENSE;md5=d32239bcb673463ab874e80d47fae504"

DEPENDS = " \
    qtbase \
    qtquickcontrols2 \
    qtlocation \
    qtgraphicaleffects \
    qtsvg \
    qtwebsockets \
    "

PV = "1.0.0"

SRC_URI = "git://git.automotivelinux.org/apps/mominavi;protocol=https;branch=${AGL_BRANCH} \
           file://mominavi.service \
           file://mominavi \
          "
SRCREV = "e08336ba085d798e88e33c24b850956d6e50cc51"

S = "${WORKDIR}/git"

inherit qmake5 systemd

MOMIMAP_MAPBOX_ACCESS_TOKEN ??= "YOU_NEED_TO_SET_IT_IN_LOCAL_CONF"
QT_INSTALL_PREFIX = "/usr"

do_configure:prepend() {
	if [ "${MOMIMAP_MAPBOX_ACCESS_TOKEN}" = "YOU_NEED_TO_SET_IT_IN_LOCAL_CONF" ]; then
		bbwarn "WARNING: You should set MapBox development key to MOMIMAP_MAPBOX_ACCESS_TOKEN variable in local.conf."
	fi
}
do_install:append() {
    install -d ${D}/${systemd_unitdir}/system
    install -m 0644 ${WORKDIR}/mominavi.service ${D}/${systemd_unitdir}/system

    install -m 0755 -d ${D}${sysconfdir}/default/
    install -m 0755 ${WORKDIR}/mominavi ${D}${sysconfdir}/default/

    echo 'MOMIMAP_MAPBOX_ACCESS_TOKEN=${MOMIMAP_MAPBOX_ACCESS_TOKEN}' >> ${D}${sysconfdir}/default/mominavi
}

FILES:${PN} += " \
    ${systemd_unitdir} \
    ${sysconfdir}/*/* \
    "
SYSTEMD_PACKAGES = "${PN}"
SYSTEMD_SERVICE:${PN} = "mominavi.service"

RDEPENDS:${PN} = " \
    qtsvg \
    qtwebsockets \
    qtlocation \
    "
