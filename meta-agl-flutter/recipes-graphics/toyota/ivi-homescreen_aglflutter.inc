FILESEXTRAPATHS:append := "${THISDIR}/files:"

SRC_URI += " \
    file://0001-Allow-the-embedder-to-run-as-a-regular-normal-applic.patch \
    file://0002-shell-configuration-Obey-json-configuration-file.patch \
    file://0003-shell-configuration-Fixes-to-general-options.patch \
    file://0004-Add-app-id-command-line-argument.patch \
    file://config.json \
"

# Upstream is now pinning ivi-homescreen, but if it did need to be pinned
# it can be done here.
#SRCREV = ""

# For now disable gstreamer to avoid needing to enable "commercial"
# licenses for the stated ffmpeg dependency.
PACKAGECONFIG:remove = "gstreamer"

#
# AGL specifics
#

IVI_HOMESCREEN_APP_OVERRIDE = "--b=/usr/share/flutter/gallery --j=/usr/share/flutter/config.json"

SERVICE_UNIT        = "Requires=agl-compositor.service\nAfter=agl-compositor.service"
SERVICE_INSTALL     = "WantedBy=agl-session.target"

# The "homescreen" naming of the embedder binary conflicts with the
# Qt homescreen in meta-agl-demo.  At least for now, rename it to
# allow running Flutter apps in an image that uses the Qt homescreen
# and launcher for testing.
SERVICE_EXEC_START  = "ExecStart=/usr/bin/flutter --f ${IVI_HOMESCREEN_APP_OVERRIDE} ${SERVICE_EXEC_START_PARAMS}"

# we have a regular/agl-driver user, so avoid setting one
SERVICE_ENVIRONMENT = ""
SERVICE_USER_GROUP  = ""
SERVICE_RESTART	    = "Restart=on-failure"

# we need to install as user session, not root.
do_install:append() {
    # The system level systemd unit is not required in AGL
    rm -f ${D}{systemd_system_unitdir}/homescreen.service

    install -d ${D}${systemd_user_unitdir}/agl-session.target.wants
    install -D -m 0644 ${WORKDIR}/config.json ${D}${datadir}/flutter/config.json
    install -m 0644 ${WORKDIR}/homescreen.service ${D}${systemd_user_unitdir}/homescreen.service
    ln -s ../homescreen.service ${D}${systemd_user_unitdir}/agl-session.target.wants/homescreen.service

    # Rename embedder binary, see explanation above
    mv ${D}${bindir}/homescreen ${D}${bindir}/flutter
}

# Split the embedder binary into a separate package to allow reusing it
# without the generated upstream systemd unit that ATM is still useful
# for testing with the agl-image-flutter image.  AGL will use either
# the applaunchd systemd template or custom units in meta-agl-demo.
PACKAGE_BEFORE_PN = "flutter-embedder-${FLUTTER_RUNTIME}"

FILES:flutter-embedder-${FLUTTER_RUNTIME} = "${bindir}"

FILES:${PN} += "${systemd_user_unitdir} ${datadir}/flutter/config.json"

RDEPENDS:${PN} += "flutter-embedder-${FLUTTER_RUNTIME}"

RDEPENDS:flutter-embedder-${FLUTTER_RUNTIME} += "flutter-engine-${FLUTTER_RUNTIME}"
