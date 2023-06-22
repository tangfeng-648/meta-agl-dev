PV = "0.0+git${SRCPV}"
SRCREV = "4b1f508003a7bf36256d5cc81986ef733e4cc79b"
BRANCH ?= "main"
SRC_URI = " \
    git://github.com/unified-hmi/remote-virtio-gpu.git;protocol=https;branch=${BRANCH} \
"
