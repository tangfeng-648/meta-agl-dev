SUMMARY = "Boa Web Server"
HOMEPAGE = "http://www.boa.org/"
DESCRIPTION = "Boa web server is a tiny web server."

LICENSE = "GPLv3"
LIC_FILES_CHKSUM = "file://Gnu_License;md5=94d55d512a9ba36caa9b7df079bae19f"

DEPENDS = "bison-native flex-native"

SRC_URI = "http://www.boa.org/boa-0.94.13.tar.gz \
           file://0001-boa-fix-compat.h.patch \
           file://0001-boa-fix-configure-avoid-run-test-on-cross-platform.patch \
           file://0001-boa-fix-icky-kernel-bug.patch \
           file://0001-boa-modify-conf-file-matching-AGL-system.patch \
"

SRC_URI[sha256sum] = "e00bb50eb859c736f2afc913976e82e8fc68a1fbe34fa294e014aa95f4d87366"

do_configure() {
    cd ${S}/src
    ./configure
    sed -i '/^CC =/ s/$/& -fcommon/' Makefile
}

do_compile() {
    cd ${S}/src
    oe_runmake
}

do_install() {
    install -d ${D}/usr/bin/
    install -m 0755 ${S}/src/boa ${D}/usr/bin/
    install -m 0755 ${S}/src/boa_indexer ${D}/usr/bin/
    install -d ${D}/etc/boa/
    install -m 0755 ${S}/boa.conf ${D}/etc/boa/
}

RDEPENDS:${PN} += "mime-support"
