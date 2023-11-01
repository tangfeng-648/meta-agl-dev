SUMMARY = "Dataset and a pre-trained model for the AGL voice assistant RASA based NLU intent engine."
HOMEPAGE = "https://github.com/malik727/rasa-model-agl"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=2e01060a5557fe6a4b57f5ea6dc048d9"

SRC_URI = " \
    git://gerrit.automotivelinux.org/gerrit/src/rasa-model-agl;protocol=https;branch=${AGL_BRANCH} \
"

SRCREV = "${AUTOREV}"
S = "${WORKDIR}/git"

do_install() {
    install -d ${D}/usr/share/nlu/rasa
    cp -R ${WORKDIR}/git/* ${D}/usr/share/nlu/rasa/
}

FILES:${PN} += " /usr/share/nlu/rasa"
