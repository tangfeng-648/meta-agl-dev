# Integration type support for lxc autostart based startup.

IMAGE_INSTALL += " \
    ${LXC_CONTAINER_CONFIGS} \
    lxc-autostart \
"

LXC_CONTAINER_CONFIGS ?= ""

python __anonymous() {
    for c in (d.getVar('CONTAINER_IMAGES') or "").split():
        (mc, image) = c.split(':')

        # Assume there is a X-lxc-config package for guest-image-X
        config = image
        if config.startswith('guest-image-'):
            config = config[len('guest-image-'):]
        d.appendVar('LXC_CONTAINER_CONFIGS', ' ' + 'lxc-config-' + config)
}
