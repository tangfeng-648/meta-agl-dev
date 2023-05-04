DESCRIPTION = "Container Manager config for Momi IVI demo"
LICENSE = "BSD-3-Clause"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/BSD-3-Clause;md5=550794465ba0ec5312d6919e203a55f9"

SRC_URI = "file://agl-momi-ivi-demo.json \
           file://system.conf.agl-momi-ivi.in \
          "

inherit allarch

do_install:append() {
    install -Dm644 ${WORKDIR}/agl-momi-ivi-demo.json ${D}/opt/container/conf/agl-momi-ivi-demo.json
    install -d ${D}/opt/container/guests/agl-momi-ivi-demo/rootfs
    install -d ${D}/opt/container/guests/agl-momi-ivi-demo/nv
    install -Dm644 ${WORKDIR}/system.conf.agl-momi-ivi.in ${D}/opt/container/guests/agl-momi-ivi-demo/system.conf
}

FILES:${PN} += " \
    /opt/container/conf/* \
    /opt/container/guests/agl-momi-ivi-demo/* \
    "
