SUMMARY     = "Window Manager service binding for applications"
DESCRIPTION = "Window Manager is the service binding for controlling \
               rendering rights. Applications request to render itself, \
               then Window Manager checks the policy and notifies the \
               layout to the respective applications \
              "
HOMEPAGE    = "https://wiki.automotivelinux.org/windowmanager"
SECTION     = "graphics"
LICENSE     = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=2ee41112a44fe7014dce33e26468ba93"

DEPENDS = "af-binder json-c wayland wayland-ivi-extension wayland-native"

inherit cmake aglwgt

SRC_URI = "git://gerrit.automotivelinux.org/gerrit/apps/agl-service-windowmanager;protocol=https;branch=${AGL_BRANCH}"
SRCREV = "8342c421b5b5081ba74a3acbece962ab080b39ea"
PV = "1.0+git${SRCPV}"
S = "${WORKDIR}/git"

#If you would like to output log, uncomment out
EXTRA_OECMAKE_append_agl-devel = " -DENABLE_DEBUG_OUTPUT=ON "