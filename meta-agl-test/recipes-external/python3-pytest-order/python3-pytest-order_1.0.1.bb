DESCRIPTION = "pytest plugin for test sequence."
DEPENDS += "${PYTHON_PN}-setuptools-scm-native"

LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://LICENSE;md5=759a21ef176b699c267d76f658c38340"

SRC_URI[sha256sum] = "5dd6b929fbd7eaa6d0ee07586f65c623babb0afe72b4843c5f15055d6b3b1b1f"

PYPI_PACKAGE = "pytest-order"

inherit pypi python_setuptools_build_meta

RDEPENDS:${PN} += " \
    ${PYTHON_PN}-pytest \
"

BBCLASSEXTEND = "native nativesdk"
