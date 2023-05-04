# Integration type support for multi partition with host and guest image.

require recipes-platform/images/agl-lxc-multiconfig-build.inc

# Handle modification of IMAGE_LINK_NAME done by ULCB builds with Kingfisher support
MACHINE_SUFFIX = "${@bb.utils.contains('AGL_FEATURES', 'kingfisher', '-kf', '', d)}"

install_container_images() {
    for c in ${CONTAINER_IMAGES}; do
        config=${c%:*}
        image=${c#*:}
        name=${image#guest-image-}
        rm -f "${DEPLOY_DIR_IMAGE}/${image}.ext4"
        src="${TOPDIR}/tmp-${config}/deploy/images/${MACHINE}/${image}-${MACHINE}${MACHINE_SUFFIX}.ext4"
        bbnote "Installing ${src}"
        ln -s ${src} "${DEPLOY_DIR_IMAGE}/${image}.ext4"
    done
}

ROOTFS_POSTPROCESS_COMMAND += "install_container_images; "
