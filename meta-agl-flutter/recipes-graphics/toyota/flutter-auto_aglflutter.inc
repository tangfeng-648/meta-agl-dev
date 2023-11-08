FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

SRC_URI:append:use-nxp-bsp = " file://0001-Disable-on_frame_base_surface-wl_surface_commit.patch"
