DESCRIPTION = "Currently, only for testing vosk websocket server."
SUMMARY = "A simple demo consisting of a websocket PyQt client and UI made using QML."
HOMEPAGE = "https://github.com/amanarora9848/vosk-server-py-qt-demo"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://LICENSE;md5=b21e7c146caa10030fea1d5525982206"

SRC_URI = "git://github.com/amanarora9848/vosk-server-py-qt-demo;protocol=https;branch=main"

PV = "1.0+git${SRCPV}"
SRCREV = "877c4b19411a42c35bbd345004564449a24304d0"

S = "${WORKDIR}/git"

RDEPENDS:${PN} += " \
    python3-pyqt5 \
    python3-wavio \
    python3-websockets \
    python3-sounddevice \
    wayland \
"

do_configure () {
	:
}

do_compile () {
	:
}

do_install () {
	install -d ${D}${libdir}/vosk-server-demo-qml-pyqt
	cp ${S}/main.py ${D}${libdir}/vosk-server-demo-qml-pyqt/
    cp ${S}/main.pyproject ${D}${libdir}/vosk-server-demo-qml-pyqt/
    cp ${S}/main.pyproject.user ${D}${libdir}/vosk-server-demo-qml-pyqt/
    cp ${S}/record_voice_send.py ${D}${libdir}/vosk-server-demo-qml-pyqt/
    cp ${S}/sendwav.py ${D}${libdir}/vosk-server-demo-qml-pyqt/
    cp ${S}/voiceRecognition.qml ${D}${libdir}/vosk-server-demo-qml-pyqt/
    chmod a+x ${D}${libdir}/vosk-server-demo-qml-pyqt/main.py
}

FILES:${PN} += " /usr/lib/vosk-server-demo-qml-pyqt "
