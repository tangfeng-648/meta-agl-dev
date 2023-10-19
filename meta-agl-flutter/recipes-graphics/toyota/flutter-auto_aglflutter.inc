FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += "file://0001-The-ivi-homescreen-is-using-eglGetPlatformDisplay-di.patch"

# NOTE: Cannot use += here due to ??= use in the recipe
PACKAGECONFIG:append = " client-agl"
