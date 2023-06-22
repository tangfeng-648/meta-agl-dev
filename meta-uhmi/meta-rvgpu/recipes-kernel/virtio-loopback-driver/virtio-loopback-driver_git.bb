SUMMARY = "Virtio Loopback Driver for Remote virtual display device"

LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://LICENSE.md;md5=688693ebbe31e3eadf819d7d007fa654"

require virtio-loopback-driver.inc

SRC_URI:append = " \
    file://0001-Remove-card_index-option.patch \
    file://Makefile.driver \
    file://Kbuild \
"

S = "${WORKDIR}/git"
EXTRA_OEMAKE = "KDIR=${STAGING_KERNEL_BUILDDIR}"
MODULES_MODULE_SYMVERS_LOCATION = "src"

inherit module

do_compile:prepend() {
    cp ${WORKDIR}/Makefile.driver ${S}/Makefile
    cp ${WORKDIR}/Kbuild ${S}/src/
}

do_install:append() {
    install -d ${D}${PKG_CONFIG_SYSTEM_INCLUDE_PATH}/remote-virtio-gpu
    install -m 755 ${S}/src/virtio_lo.h ${D}${PKG_CONFIG_SYSTEM_INCLUDE_PATH}/remote-virtio-gpu/virtio_lo.h
}

FILES:${PN} = " \
    ${libdir}/modules/${KERNEL_VERSION}/extra/virtio_lo* \
    ${sysconfdir}/modules-load.d \
"

RPROVIDES:${PN} += " \
    kernel-module-virtiolo \
    kernel-module-virtio-lo${KERNEL_MODULE_PACKAGE_SUFFIX} \
"

# Autoload virtio lo driver
KERNEL_MODULE_AUTOLOAD:append = " virtio_lo"
