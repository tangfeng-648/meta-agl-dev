SUMMARY = "AGL container integration feature packages for logging."
DESCRIPTION = "This pacage group including AGL container integration feature packages \
               for logging."
HOMEPAGE = "https://confluence.automotivelinux.org/display/IC"

LICENSE = "Apache-2.0"

PACKAGE_ARCH = "${TUNE_PKGARCH}"

inherit packagegroup

PACKAGES = "\
    ${PN}-host \
    ${PN}-guest \
    ${PN}-hostdev \
    ${PN}-guestdev \
"
RDEPENDS:${PN}-host += "\
    dlt-daemon \
    dlt-daemon-system \
"
RDEPENDS:${PN}-guest += "\
    dlt-daemon-system \
"
RDEPENDS:${PN}-hostdev += "\
    ${PN}-host \
    dlt-daemon-command \
"
RDEPENDS:${PN}-guestdev += "\
    ${PN}-guest\
"
