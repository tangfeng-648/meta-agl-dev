DESCRIPTION = "Mode 'standard' components"
LICENSE = "Apache-2.0"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-extend-test-standard \
"

ALLOW_EMPTY:${PN} = "1"

RDEPENDS:${PN} += " \
    packagegroup-agl-extend-test-fast \
    agl-test-framework-standard \
"
