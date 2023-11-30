PV = "0.0+git${SRCPV}"
SRCREV = "cd79224421fb2e3a0109c8e765264f5f39af9ef3"
BRANCH ?= "main"
SRC_URI = " \
    git://github.com/unified-hmi/remote-virtio-gpu.git;protocol=https;branch=${BRANCH} \
"
