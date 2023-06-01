SUMMARY = "Baseline Flutter Image for Development"

LICENSE = "MIT"

require agl-image-flutter.inc

IMAGE_INSTALL:append = "\
    weston-ini-conf-landscape \
    \
    flutter-auto-with-logging \
    \
    flutter-engine-sdk-dev \
    \
    flutter-gallery \
    flutter-test-animated-background \
    flutter-test-texture-egl \
    \
    "

IMAGE_FEATURES:append = "\
    ssh-server-openssh \
    "
