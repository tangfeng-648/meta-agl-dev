DESCRIPTION = "AGL flutter IVI demo container config"
LICENSE = "BSD-3-Clause"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/BSD-3-Clause;md5=550794465ba0ec5312d6919e203a55f9"

SRC_URI = "file://agl-flutter-ivi-demo.json \
           file://system.conf.agl-flutter-ivi-demo.in \
          "

inherit allarch

do_install:append() {
    install -Dm644 ${WORKDIR}/agl-flutter-ivi-demo.json ${D}/opt/container/conf/agl-flutter-ivi-demo.json
    install -d ${D}/opt/container/guests/agl-flutter-ivi-demo/rootfs
    install -d ${D}/opt/container/guests/agl-flutter-ivi-demo/nv
    install -Dm644 ${WORKDIR}/system.conf.agl-flutter-ivi-demo.in ${D}/opt/container/guests/agl-flutter-ivi-demo/system.conf
}

FILES:${PN} += " \
    /opt/container/conf/* \
    /opt/container/guests/agl-flutter-ivi-demo/* \
    "
