SUMMARY = "AGL Voice Assistant"
DESCRIPTION = "Offline voice assistant app designed for Automotive Grade Linux (AGL)."
HOMEPAGE = "https://github.com/malik727/agl-flutter-voiceassistant"
BUGTRACKER = "https://github.com/malik727/agl-flutter-voiceassistant/issues"
SECTION = "graphics"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=4202492ed9afcab3aaecc4a9ec32adb2"

SRC_URI = "git://github.com/malik727/agl-flutter-voiceassistant.git;protocol=https;branch=main \
    "
    
SRCREV = "${AUTOREV}"
S = "${WORKDIR}/git"

inherit agl-app flutter-app

# flutter-app
#############
PUBSPEC_APPNAME = "agl_flutter_voiceassistant"
FLUTTER_APPLICATION_INSTALL_PREFIX = "/flutter"
FLUTTER_BUILD_ARGS = "bundle -v"

# agl-app
#########
AGL_APP_TEMPLATE = "agl-app-flutter"
AGL_APP_ID = "agl_flutter_voiceassistant"
AGL_APP_NAME = "AGL Voice Assistant"