SUMMARY = "Redundancy file operation library (librefop)"
DESCRIPTION = "Redundancy file operation library is a another implementation for \
    the backup manager.  It aim to tiny implementation."
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=ae6497158920d9524cf208c09cc4c984"

PV = "1.0.0+rev${SRCPV}"

SRCREV = "2ab22b2df67f915c74efab68cf9e6e2424805dd2"
SRC_URI = " \
    git://git.automotivelinux.org/src/librefop;branch=master;protocol=https \
    "
S = "${WORKDIR}/git"

inherit autotools pkgconfig

