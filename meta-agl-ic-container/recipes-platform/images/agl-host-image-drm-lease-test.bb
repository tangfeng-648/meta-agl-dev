SUMMARY = "DRM Lease LXC test host image"
LICENSE = "MIT"

require lxc-host-image-minimal.bb
require recipes-platform/images/agl-lxc-install-single-image.inc
require recipes-platform/images/agl-lxc-autostart.inc

CONTAINER_IMAGES ?= "agl-container-ivi:guest-image-drm-lease-test-ivi \
                     agl-container-cluster:guest-image-drm-lease-test-cluster \
                    "

IMAGE_INSTALL += " \
    kernel-modules \
"

# packages required for network bridge settings via lxc-net
IMAGE_INSTALL += " \
    lxc-networking \
    iptables-modules \
    dnsmasq \
    systemd-netif-config \
    kernel-module-xt-addrtype \
    kernel-module-xt-multiport \
"

# Under the this line, shall describe machine specific package.
IMAGE_INSTALL:append:rcar-gen3 = " kernel-module-gles gles-user-module-firmware"
