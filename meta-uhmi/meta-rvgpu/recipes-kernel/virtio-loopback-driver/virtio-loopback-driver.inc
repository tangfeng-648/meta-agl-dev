PV = "0.0+git${SRCPV}"
SRCREV = "db18183e68033fd4a6798e8998a5d9a3c6410cfc"
BRANCH ?= "main"
SRC_URI = " \
   git://github.com/unified-hmi/virtio-loopback-driver.git;protocol=https;branch=${BRANCH} \
"
