# Container Manager config for cluster demo on R-Car Gen3
FILESEXTRAPATHS:prepend := "${THISDIR}/cm-config-cluster-demo:"

require conf/include/drm-lease-multi-display.inc

# If you want to change display assign in your board, please change this line in your recipe.
DRM_LEASE_DEVICE ?= "${@bb.utils.contains("HAS_MULTI_DISPLAY", "1", "card0-HDMI-A-2", "card0-HDMI-A-1" ,d)}"
