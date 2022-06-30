DESCRIPTION = "Agl Extend Test Group"
LICENSE = "Apache-2.0"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-extend-test \
"

ALLOW_EMPTY:${PN} = "1"

RDEPENDS:${PN} += " \
    agl-test-framework \
"
