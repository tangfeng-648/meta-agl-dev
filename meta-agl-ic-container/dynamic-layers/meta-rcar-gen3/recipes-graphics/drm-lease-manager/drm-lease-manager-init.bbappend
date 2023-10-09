FILESEXTRAPATHS:prepend := "${THISDIR}/${PN}:"
SRC_URI = "file://drm-lease-manager.ini"

do_install:append:r8a7796() {
    sed -i -e "s/connectors=\[\"HDMI-A-2\"\]/connectors=\[\"VGA-1\"\]/g" \
        ${D}${sysconfdir}/xdg/drm-lease-manager/drm-lease-manager.ini
}
