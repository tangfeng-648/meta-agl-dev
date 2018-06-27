SUMMARY     = "4A - Generic HAL"
DESCRIPTION = "Generic HAL in 4A (AGL Advanced Audio Agent)"
HOMEPAGE    = "https://github.com/iotbzh/4a-hal-generic/"
SECTION     = "apps"

LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=86d3f3a95c324c9479bd8986968f4327"

SRC_URI = "gitsm://github.com/iotbzh/4a-hal-generic;protocol=https;branch=${AGL_BRANCH}"
SRCREV = "81766dc459a1a7938c973bdb6730d1a07cbfe224"

DEPENDS += "lua"

PV = "0.1+git${SRCPV}"
S  = "${WORKDIR}/git"

inherit afb-system-cmake

# FIXME:
#FILES_${PN}-dev += "${INSTALL_PREFIX}/4a-hal/htdocs"
#FILES_${PN} += "${INSTALL_PREFIX}/afb-aaaa"
#FILES_${PN} += "${INSTALL_PREFIX}/lib"
