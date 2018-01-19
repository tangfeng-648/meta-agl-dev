SUMMARY     = "Sound Manager client library for applications"
DESCRIPTION = "Sound Manager client library for applications built with recipe"
HOMEPAGE    = "https://wiki.automotivelinux.org/soundmanager"
SECTION     = "multimedia"
LICENSE     = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=ae6497158920d9524cf208c09cc4c984"

DEPENDS = "af-binder json-c"
RDEPENDS_${PN} = "agl-service-audio-soundmanager"

inherit cmake

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/staging/libsoundmanager.git;protocol=https;branch=${AGL_BRANCH}"
SRCREV = "f4dd1b50ef871116e304153e8d9acc296dcdb232"
S = "${WORKDIR}/git"
