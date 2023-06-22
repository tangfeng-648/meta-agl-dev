# This recipe temporarily install mesa-18.2.0 as mesa-virtio to run remote-virtio-gpu.
# The mesa-virtio will be removed after remote-virtio-gpu supports upstream mesa.

SUMMARY = "Mesa library"
SECTION = "graphics"

LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://docs/license.html;md5=725f991a1cc322aa7a0cd3a2016621c4"

DEPENDS = "expat makedepend-native flex-native bison-native libxml2-native zlib chrpath-replacement-native"
DEPENDS:append = " libdrm wayland wayland-native wayland-protocols python3-native"

RDEPENDS:${PN}:append = " libgcc wayland libdrm glibc libstdc++ zlib expat"

LINUX_MAJOR = "${@(d.getVar('PREFERRED_VERSION_linux-yocto') or "x.y").split('.')[0]}"
LINUX_MINOR = "${@(d.getVar('PREFERRED_VERSION_linux-yocto') or "x.y").split('.')[1].split('%')[0]}"

SRC_URI = "https://mesa.freedesktop.org/archive/mesa-${PV}.tar.xz \
    file://0001-glBufferData-Update-resource-backing-memory.patch \
    file://0001-Use-wayland-scanner-in-the-path.patch \
    file://0002-mesa-virtio-Fix-missing-wayland-egl-backend-build-error.patch \
    file://0001-add-stride-status-to-virtgpu-3d-transfer-to-host-linux-${LINUX_MAJOR}-${LINUX_MINOR}.patch \
    file://0001-Enable-using-python3.patch \
"

SRC_URI[md5sum] = "88e1a7f31f259cec69bb76b3cb10c956"
SRC_URI[sha256sum] = "22452bdffff8e11bf4284278155a9f77cb28d6d73a12c507f1490732d0d9ddce"

S = "${WORKDIR}/mesa-${PV}"

inherit autotools pkgconfig gettext

EXTRA_OEMAKE += "WAYLAND_PROTOCOLS_DATADIR=${STAGING_DATADIR}/wayland-protocols"

EXTRA_OECONF = " \
    --prefix=/usr/lib/mesa-virtio \
    --exec_prefix=/usr/lib/mesa-virtio \
    --libdir=/usr/lib/mesa-virtio \
    --includedir=/usr/include/mesa-virtio \
    --sysconfdir=/etc/mesa-virtio \
    --datadir=/usr/share/mesa-virtio \
"

EXTRA_OECONF:append = " \
    --with-dri-drivers=swrast \
    --with-gallium-drivers=swrast,virgl \
    --with-platforms=drm,wayland \
    --disable-glx \
    --disable-dri3 \
"

EXCLUDE_FROM_SHLIBS = "1"

FILES:${PN} = " \
    /usr/lib/mesa-virtio/* \
    /etc/mesa-virtio/drirc \
    /usr/share/mesa-virtio/* \
"

FILES:${PN}-dev += " \
    /usr/lib/mesa-virtio/libglapi.so \
    /usr/lib/mesa-virtio/libEGL.so \
    /usr/lib/mesa-virtio/libgbm.so \
    /usr/lib/mesa-virtio/libGLESv2.so \
    /usr/lib/mesa-virtio/libGLESv1_CM.so \
"
