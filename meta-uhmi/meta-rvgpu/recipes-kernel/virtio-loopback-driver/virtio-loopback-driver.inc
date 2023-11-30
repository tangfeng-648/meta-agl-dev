PV = "0.0+git${SRCPV}"
SRCREV = "070ba544722c78e3f6a0f43e4fe08a0214c66a49"
BRANCH ?= "main"
SRC_URI = " \
   git://github.com/unified-hmi/virtio-loopback-driver.git;protocol=https;branch=${BRANCH} \
"
