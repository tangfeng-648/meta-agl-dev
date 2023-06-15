DESCRIPTION = "Mode 'fast' components"
LICENSE = "Apache-2.0"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-extend-test-fast \
"

ALLOW_EMPTY:${PN} = "1"

RDEPENDS:${PN} += " \
                    agl-test-framework-base \
                    agl-test-framework-fast \
"
