DESCRIPTION = "Builds the EDAC inject module"
HOMEPAGE = "https://github.com/elisa-tech/linux/commits/edac_inject"
LICENSE = "GPLv2"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/GPL-2.0;md5=801f80980d171dd6425610833a22dbe6"

DEPENDS += "virtual/kernel"

SRC_URI = " \
	file://edac_device.h  \
	file://edac_inject.c  \
	file://edac_mc.h  \
	file://edac_module.h  \
	file://edac_pci.h  \
	file://Makefile  \
"

S = "${WORKDIR}/"

inherit module

EXTRA_OEMAKE += "KDIR=${STAGING_KERNEL_DIR}"

do_compile_prepend () {
    sed -i "s/arm-none-linux-gnueabi-/${TARGET_PREFIX}/g" ${S}/Makefile
}

do_install () {
        install -d ${D}/lib/modules/${KERNEL_VERSION}/drivers/edac/
        install -m 0755 ${S}/edac_inject.ko ${D}/lib/modules/${KERNEL_VERSION}/drivers/edac/
}
