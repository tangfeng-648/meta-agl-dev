FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI += "file://0001-Add-optional-agl_shell-plugin.patch"

# For now disable gstreamer to avoid needing to enable "commercial"
# licenses for the stated ffmpeg dependency.
PACKAGECONFIG:remove = "gstreamer"

# Enable agl-shell wayland protocol plugin
PACKAGECONFIG[agl-shell] = "-DBUILD_PLUGIN_AGL_SHELL=ON, -DBUILD_PLUGIN_AGL_SHELL=OFF"
# NOTE: Cannot use += here due to ??= use in the recipe
PACKAGECONFIG:append = " agl-shell"

