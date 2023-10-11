SUMMARY = "Motion JPEG streamer but modified for AGL system."
HOMEPAGE = "http://sourceforge.net/projects/mjpg-streamer/"
DESCRIPTION = "Motion JPEG streamer is a video streamer specially designed for Web Cameras."

LICENSE = "GPL-2.0-only"
LIC_FILES_CHKSUM = "file://${S}/mjpg-streamer/LICENSE;md5=751419260aa954499f7abaabaa882bbe"

PN = "mjpg-streamer"

SRC_URI = "git://github.com/lucky33newman/Motion-JPEG-for-AGL.git;branch=main;protocol=https"

SRCREV = "09185f3d55b7c4de6d196d2ad5c5b4eadc4f340a"

S = "${WORKDIR}/git"

DEPENDS = "libjpeg-turbo"
RDEPENDS:${PN} += "libjpeg-turbo"

EXTRA_OEMAKE = "LDFLAGS='${LDFLAGS}'"

do_compile() {
    oe_runmake -C ${S}/mjpg-streamer
}

do_install() {
    install -d ${D}/usr/bin/
    install -m 0755 ${S}/mjpg-streamer/mjpg_streamer ${D}/usr/bin/
    install -d ${D}/usr/lib/mjpg-streamer
    install -m 0755 ${S}/mjpg-streamer/*.so ${D}/usr/lib/mjpg-streamer/
    install -d ${D}/etc/MJPG-streamer/www/
    install -m 0755 ${S}/mjpg-streamer/www/* ${D}/etc/MJPG-streamer/www/
}
