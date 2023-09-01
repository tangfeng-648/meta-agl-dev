DESCRIPTION = "AGL Wireless Monitor Group"
LICENSE = "Apache-2.0"

inherit packagegroup

PACKAGES = "\
    packagegroup-agl-wireless-monitor \
"

RDEPENDS:${PN} += " \
    incar-wireless-monitor \
"
