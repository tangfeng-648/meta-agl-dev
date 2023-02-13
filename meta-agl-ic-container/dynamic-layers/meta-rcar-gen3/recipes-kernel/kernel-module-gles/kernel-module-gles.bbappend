inherit guest-kernel-module

SYSROOT_PREPROCESS_FUNCS:aglcontainerguest = ""

RDEPENDS:${PN}:append = " gles-user-module-firmware"
