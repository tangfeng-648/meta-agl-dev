SUMMARY = "AGL Instrument Cluster Demo Qt Packages"
DESCRIPTION = "This pacage group including Qt packages for AGL Instrument Cluster Demo."
HOMEPAGE = "https://confluence.automotivelinux.org/display/IC"

LICENSE = "Apache-2.0"

PACKAGE_ARCH = "${TUNE_PKGARCH}"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-ic-qt \
"
RDEPENDS:${PN} += "\
    qtbase qtbase-plugins qtbase-qmlplugins \
    qtdeclarative qtdeclarative-plugins qtdeclarative-qmlplugins \
    qtgraphicaleffects qtgraphicaleffects-plugins qtgraphicaleffects-qmlplugins \
    qtmultimedia qtmultimedia-plugins qtmultimedia-qmlplugins \
    qtquickcontrols qtquickcontrols-plugins qtquickcontrols-qmlplugins \
    qtquickcontrols2 qtquickcontrols2-plugins qtquickcontrols2-qmlplugins \
    qtwayland qtwayland-plugins qtwayland-qmlplugins \
    qt3d qt3d-plugins qt3d-qmlplugins \
"
