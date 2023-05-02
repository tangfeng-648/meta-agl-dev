SUMMARY = "A minimal container host image"

require recipes-platform/images/agl-image-boot.inc

IMAGE_INSTALL += " \
    kernel-image \
    lxc \
    drm-lease-manager \
"

CONTAINER_IMAGES ??= ""

IMAGE_LINGUAS ?= " "

NO_RECOMMENDATIONS = "1"

IMAGE_ROOTFS_EXTRA_SPACE:append = "${@bb.utils.contains("DISTRO_FEATURES", "systemd", " + 4096", "" ,d)}"
