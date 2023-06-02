SUMMARY = "AGL Flutter environment file"
LICENSE = "MIT"
LIC_FILES_CHKSUM = "file://${COREBASE}/meta/COPYING.MIT;md5=3da9cfbcb788c80a0384361b4de20420"

inherit allarch

AGL_FLUTTER_DEFAULT_RUNTIME ?= "release"

do_compile[noexec] = "1"

do_install() {
    install -d ${D}${sysconfdir}/default
    echo "FLUTTER_VERSION=${FLUTTER_SDK_TAG}" > ${D}${sysconfdir}/default/flutter
    echo "FLUTTER_RUNTIME=${AGL_FLUTTER_DEFAULT_RUNTIME}" >> ${D}${sysconfdir}/default/flutter
}
