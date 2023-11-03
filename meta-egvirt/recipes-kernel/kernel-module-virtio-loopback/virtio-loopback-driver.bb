SUMMARY = "Virtio-loopback driver"
DESCRIPTION = "Virtio-Loopback kernel driver"
LICENSE = "GPL"
LIC_FILES_CHKSUM = "file://README.md;md5=c912e5645ed908bc9570ba05c92b3723"

inherit module

SRC_URI = "git://git.virtualopensystems.com/virtio-loopback/loopback_driver.git;protocol=https;rev=b94d81b014f322677036776e19bdf9d6fa61dd8e;branch=epsilon-release-5.15"

S = "${WORKDIR}/git"

MAKE_TARGETS = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR}/git MODULE_GIT_REPOSITORY_DIR=${METADIR}/meta-agl-devel"
MODULES_INSTALL_TARGET = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR}/git modules_install"

