#aio-stress
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/aio-stress/
    install -m 0644 ${WORKDIR}/git/tests/aio-stress/* ${D}/usr/AGL/agl-test/tests/aio-stress/
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

#zlib
do_install:append() {
    install -d ${D}/usr/AGL/agl-test/tests/zlib/
    install -m 0644 ${WORKDIR}/git/tests/zlib/* ${D}/usr/AGL/agl-test/tests/zlib/
}
