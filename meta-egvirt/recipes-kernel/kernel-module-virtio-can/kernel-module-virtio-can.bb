SUMMARY = "VIRTIO CAN device driver"

LICENSE = "BSD-3-Clause & GPL-2.0-only"
LIC_FILES_CHKSUM = " \
    file://virtio_can.h;endline=4;md5=d93ffb4ab090b382cbcda4cb2c0e5c9c \
    file://virtio_can.c;endline=5;md5=06e45bdf8cb26f6a72e240cb1b0c18c2 \
"

inherit module

SRC_URI = " \
    file://Kbuild \
    file://virtio_can.c \
    file://virtio_can.h \
"

S = "${WORKDIR}"

MAKE_TARGETS = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR} MODULE_GIT_REPOSITORY_DIR=${METADIR}/meta-agl-devel"
MODULES_INSTALL_TARGET = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR} modules_install"
