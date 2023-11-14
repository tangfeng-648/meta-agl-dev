SUMMARY = "Virtio-loopback driver"
DESCRIPTION = "Virtio-Loopback kernel driver"
LICENSE = "GPL-2.0-or-later"
LIC_FILES_CHKSUM = "file://README.md;md5=c912e5645ed908bc9570ba05c92b3723"

inherit module

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/src/virtio/virtio-loopback-driver.git;protocol=http;branch=${AGL_BRANCH}"

SRCREV = "f9c1c04c0a3c2a133d969c3aae8490cb25145985"

S = "${WORKDIR}/git"

MAKE_TARGETS = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR}/git"
MODULES_INSTALL_TARGET = "-C ${STAGING_KERNEL_DIR} M=${WORKDIR}/git modules_install"

