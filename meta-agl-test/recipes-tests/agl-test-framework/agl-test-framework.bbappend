#LTP
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/LTP/
    install -m 0644 ${WORKDIR}/git/tests/LTP/*py ${D}/usr/AGL/agl-test/tests/LTP/

    #cve
    install -d ${D}/usr/AGL/agl-test/tests/LTP/cve/
    install -m 0644 ${WORKDIR}/git/tests/LTP/cve/* ${D}/usr/AGL/agl-test/tests/LTP/cve/

    #math
    install -d ${D}/usr/AGL/agl-test/tests/LTP/math/
    install -m 0644 ${WORKDIR}/git/tests/LTP/math/* ${D}/usr/AGL/agl-test/tests/LTP/math/

    #posix_conformance_tests
    install -d ${D}/usr/AGL/agl-test/tests/LTP/posix_conformance_tests/
    install -m 0644 ${WORKDIR}/git/tests/LTP/posix_conformance_tests/* ${D}/usr/AGL/agl-test/tests/LTP/posix_conformance_tests/

    #syscalls
    install -d ${D}/usr/AGL/agl-test/tests/LTP/syscalls/resource
    install -m 0644 ${WORKDIR}/git/tests/LTP/syscalls/*py ${D}/usr/AGL/agl-test/tests/LTP/syscalls/
    install -m 0644 ${WORKDIR}/git/tests/LTP/syscalls/resource/* ${D}/usr/AGL/agl-test/tests/LTP/syscalls/resource/
}

#acl
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/acl/
    install -m 0644 ${WORKDIR}/git/tests/acl/* ${D}/usr/AGL/agl-test/tests/acl/
}

#aio_stress
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/aio_stress/
    install -m 0644 ${WORKDIR}/git/tests/aio_stress/* ${D}/usr/AGL/agl-test/tests/aio_stress/
}

#attr
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/attr/
    install -m 0644 ${WORKDIR}/git/tests/attr/* ${D}/usr/AGL/agl-test/tests/attr/
}

#babeltrace
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/babeltrace/
    install -m 0644 ${WORKDIR}/git/tests/babeltrace/* ${D}/usr/AGL/agl-test/tests/babeltrace/
}

#babeltrace2
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/babeltrace2/
    install -m 0644 ${WORKDIR}/git/tests/babeltrace2/* ${D}/usr/AGL/agl-test/tests/babeltrace2/
}

#bash
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/bash/
    install -m 0644 ${WORKDIR}/git/tests/bash/* ${D}/usr/AGL/agl-test/tests/bash/
}

#bc
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/bc/
    install -m 0644 ${WORKDIR}/git/tests/bc/* ${D}/usr/AGL/agl-test/tests/bc/
}

#bluez5
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/bluez5/
    install -m 0644 ${WORKDIR}/git/tests/bluez5/* ${D}/usr/AGL/agl-test/tests/bluez5/
}

#busybox
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/busybox/resource/
    install -m 0644 ${WORKDIR}/git/tests/busybox/*py ${D}/usr/AGL/agl-test/tests/busybox/
    install -m 0644 ${WORKDIR}/git/tests/busybox/resource/* ${D}/usr/AGL/agl-test/tests/busybox/resource/
}

#bzip2
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/bzip2/
    install -m 0644 ${WORKDIR}/git/tests/bzip2/* ${D}/usr/AGL/agl-test/tests/bzip2/
}

#coreutils
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/coreutils/
    install -m 0644 ${WORKDIR}/git/tests/coreutils/* ${D}/usr/AGL/agl-test/tests/coreutils/
}

#cpio
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/cpio/
    install -m 0644 ${WORKDIR}/git/tests/cpio/* ${D}/usr/AGL/agl-test/tests/cpio/
}

#crashme
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/crashme/
    install -m 0644 ${WORKDIR}/git/tests/crashme/* ${D}/usr/AGL/agl-test/tests/crashme/
}

#diffutils
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/diffutils/
    install -m 0644 ${WORKDIR}/git/tests/diffutils/* ${D}/usr/AGL/agl-test/tests/diffutils/
}

#e2fsprogs
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/e2fsprogs/
    install -m 0644 ${WORKDIR}/git/tests/e2fsprogs/* ${D}/usr/AGL/agl-test/tests/e2fsprogs/
}

#expat
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/expat/
    install -m 0644 ${WORKDIR}/git/tests/expat/* ${D}/usr/AGL/agl-test/tests/expat/
}

#gdbm
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/gdbm/
    install -m 0644 ${WORKDIR}/git/tests/gdbm/* ${D}/usr/AGL/agl-test/tests/gdbm/
}

#gdk_pixbuf
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/gdk_pixbuf/
    install -m 0644 ${WORKDIR}/git/tests/gdk_pixbuf/* ${D}/usr/AGL/agl-test/tests/gdk_pixbuf/
}

#glib2
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/glib2/
    install -m 0644 ${WORKDIR}/git/tests/glib2/* ${D}/usr/AGL/agl-test/tests/glib2/
}

#gstreamer
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/gstreamer/
    install -m 0644 ${WORKDIR}/git/tests/gstreamer/* ${D}/usr/AGL/agl-test/tests/gstreamer/
}

#json_glib
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/json_glib/
    install -m 0644 ${WORKDIR}/git/tests/json_glib/* ${D}/usr/AGL/agl-test/tests/json_glib/
}

#libpam
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/libpam/
    install -m 0644 ${WORKDIR}/git/tests/libpam/* ${D}/usr/AGL/agl-test/tests/libpam/
}

#libxml2
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/libxml2/
    install -m 0644 ${WORKDIR}/git/tests/libxml2/* ${D}/usr/AGL/agl-test/tests/libxml2/
}

#linus_stress
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/linus_stress/
    install -m 0644 ${WORKDIR}/git/tests/linus_stress/* ${D}/usr/AGL/agl-test/tests/linus_stress/
}

#lua
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/lua/
    install -m 0644 ${WORKDIR}/git/tests/lua/* ${D}/usr/AGL/agl-test/tests/lua/
}

#lz4
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/lz4/
    install -m 0644 ${WORKDIR}/git/tests/lz4/* ${D}/usr/AGL/agl-test/tests/lz4/
}

#openssl
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/openssl/
    install -m 0644 ${WORKDIR}/git/tests/openssl/* ${D}/usr/AGL/agl-test/tests/openssl/
}

#python3
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/python3/
    install -m 0644 ${WORKDIR}/git/tests/python3/* ${D}/usr/AGL/agl-test/tests/python3/
}

#stress_ng
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/stress_ng/
    install -m 0644 ${WORKDIR}/git/tests/stress_ng/* ${D}/usr/AGL/agl-test/tests/stress_ng/
}

#zlib
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/zlib/
    install -m 0644 ${WORKDIR}/git/tests/zlib/* ${D}/usr/AGL/agl-test/tests/zlib/
}
