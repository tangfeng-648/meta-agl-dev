# Integration type support for multi partition with host and guest image.

require recipes-platform/images/agl-lxc-multiconfig-build.inc

# Install out of tree build image support
OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR ??= "non"

OUT_OF_TREE_CONTAINER_IMAGES ??= " \
    agl-ivi-demo-platform \
    agl-ivi-demo-platform-flutter \
    agl-ivi-demo-platform-html5 \
"

OUT_OF_TREE_CONTAINER_CONFIGS ??= " \
    cm-config-agl-qt-ivi-demo \
    cm-config-agl-flutter-ivi-demo \
    cm-config-agl-html5-ivi-demo \
"

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

    if ${@bb.utils.contains('OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR', 'non', 'false', 'true', d)}; then
        for image in ${OUT_OF_TREE_CONTAINER_IMAGES}; do
            rm -f "${DEPLOY_DIR_IMAGE}/${image}.ext4"
            src="${OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR}/images/${MACHINE}/${image}-${MACHINE}${MACHINE_SUFFIX}.ext4"
            bbnote "Installing ${src}"
            ln -s ${src} "${DEPLOY_DIR_IMAGE}/${image}.ext4"
        done
    fi
}

ROOTFS_POSTPROCESS_COMMAND += "install_container_images; "

IMAGE_INSTALL:append = " \
    ${@bb.utils.contains('OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR', 'non', '', '${OUT_OF_TREE_CONTAINER_CONFIGS}', d)} \
"

WKS_FILES:prepend = " \
    ${@bb.utils.contains('OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR', 'non', 'agl-ic-container-noloader.wks ', 'agl-ic-container-noloader-demo.wks ', d)} \
"
