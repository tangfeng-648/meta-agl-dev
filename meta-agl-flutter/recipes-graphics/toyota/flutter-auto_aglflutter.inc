FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append:use-nxp-bsp = " file://0001-Disable-on_frame_base_surface-wl_surface_commit.patch"
SRC_URI:append: = " file://0001-display-Add-support-for-wl_output-version-4.patch \
		   file://0002-display-Add-support-for-agl_shell-version-8.patch"
