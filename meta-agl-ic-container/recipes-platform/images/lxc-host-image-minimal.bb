require recipes-platform/images/agl-image-boot.bb

SUMMARY = "A minimal container host image"

IMAGE_INSTALL += " \
    kernel-image \
    lxc \
    drm-lease-manager \
    packagegroup-agl-container-feature-logging-host \
"

CONTAINER_IMAGES ??= ""

NO_RECOMMENDATIONS = "1"

IMAGE_ROOTFS_EXTRA_SPACE:append = "${@bb.utils.contains("DISTRO_FEATURES", "systemd", " + 4096", "" ,d)}"
