# Integration type support for install guest root filesystem into host image.

# Handle modification of IMAGE_LINK_NAME done by ULCB builds with Kingfisher support
MACHINE_SUFFIX = "${@bb.utils.contains('AGL_FEATURES', 'kingfisher', '-kf', '', d)}"

python __anonymous() {
    for c in (d.getVar('CONTAINER_IMAGES') or "").split():
        (mc, image) = c.split(':')
        dependency = 'mc::' + mc + ':' + image + ':do_image_complete'
        d.appendVarFlag('do_rootfs', 'mcdepends', ' ' + dependency)
}

install_container_images() {
    for c in ${CONTAINER_IMAGES}; do
        config=${c%:*}
        image=${c#*:}
        name=${image#guest-image-}
        rm -rf  ${IMAGE_ROOTFS}/var/lib/machines/${name}
        install -m 0755 -d ${IMAGE_ROOTFS}/var/lib/machines/${name}
        src="${TOPDIR}/tmp-${config}/deploy/images/${MACHINE}/${image}-${MACHINE}${MACHINE_SUFFIX}.tar.bz2"
        bbnote "Installing ${src}"
        tar -C ${IMAGE_ROOTFS}/var/lib/machines/${name} -xf ${src}
    done
}

#
# Force rebuild of rootfs on every build to work around mcdepends issue
#
# NOTE:
# This is currently required as bitbake fails to trigger do_rootfs
# sometimes even when the guest image has in fact rebuilt.  This is
# being investigated with upstream.
#
do_rootfs[nostamp] = "1"

ROOTFS_POSTPROCESS_COMMAND += "install_container_images; "
