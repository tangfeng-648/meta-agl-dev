DESCRIPTION = "Mode 'extreme' components"
LICENSE = "Apache-2.0"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-extend-test-extreme \
"

ALLOW_EMPTY:${PN} = "1"

RDEPENDS:${PN} += " \
    packagegroup-agl-extend-test-standard \
    agl-test-framework-extreme \
"
