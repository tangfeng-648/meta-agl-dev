DESCRIPTION = "AGL simple IVI demo container LXC config"
LICENSE = "BSD-3-Clause"
LIC_FILES_CHKSUM = "file://${COMMON_LICENSE_DIR}/BSD-3-Clause;md5=550794465ba0ec5312d6919e203a55f9"

inherit lxc-config

require multi-display.inc

LXC_AUTO_START ??= "${@bb.utils.contains("HAS_MULTI_DISPLAY", "1", "1", "0" ,d)}"
