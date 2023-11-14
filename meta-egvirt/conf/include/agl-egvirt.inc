# gstreamer1.0-plugins-bad contains:
# * videoparsersbad plugin which contains h264parse element needed to demonstrate H.264 hardware decoding
# * waylandsink plugin
MACHINE_EXTRA_RDEPENDS:append = " \
    kernel-module-virtio-video \
    gstreamer1.0-plugins-bad \
"

MACHINE_EXTRA_RDEPENDS:append = " \
    kernel-module-virtio-can \
"

# add driver for supporting vhost devices loopback (virtio-loopback)
MACHINE_EXTRA_RDEPENDS:append = " \
    virtio-loopback-driver \
"

IMAGE_INSTALL:append = " vhost-device-i2c vhost-device-rng vhost-device-vsock vhost-device-gpio "

