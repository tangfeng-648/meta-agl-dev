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

#aio_stress
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/aio_stress/
    install -m 0644 ${WORKDIR}/git/tests/aio_stress/* ${D}/usr/AGL/agl-test/tests/aio_stress/
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

#crashme
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/crashme/
    install -m 0644 ${WORKDIR}/git/tests/crashme/* ${D}/usr/AGL/agl-test/tests/crashme/
}

#expat
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/expat/
    install -m 0644 ${WORKDIR}/git/tests/expat/* ${D}/usr/AGL/agl-test/tests/expat/
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
