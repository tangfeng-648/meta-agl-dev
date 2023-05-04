python __anonymous() {
    for c in (d.getVar('CONTAINER_IMAGES') or "").split():
        (mc, image) = c.split(':')
        dependency = 'mc::' + mc + ':' + image + ':do_image_complete'
        d.appendVarFlag('do_rootfs', 'mcdepends', ' ' + dependency)
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
