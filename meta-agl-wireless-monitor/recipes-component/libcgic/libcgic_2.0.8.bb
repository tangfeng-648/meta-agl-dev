SUMMARY = "CGIC library"
HOMEPAGE = "https://github.com/boutell/cgic"
DESCRIPTION = "An ANSI C library for CGI Programming."

LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://license.txt;md5=1ef0cf0c9e6e61f326fd48746179cf38"

SRCREV = "a3490b7612f194f029b6e7a7fc602ad962bc33e2"

S = "${WORKDIR}/git"

SRC_URI = "git://github.com/boutell/cgic.git;protocol=https;branch=master \
           file://0001-modify-makefile-for-cross-build.patch \
"

EXTRA_OEMAKE = "LDFLAGS='${LDFLAGS}'"

do_install() {
    install -d ${D}/usr/bin/cgi-bin
    install -m 0755 ${S}/capture ${D}/usr/bin/cgi-bin
    install -m 0755 ${S}/cgictest.cgi ${D}/usr/bin/cgi-bin
}
