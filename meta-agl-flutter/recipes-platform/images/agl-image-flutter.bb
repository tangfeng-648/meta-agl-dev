require recipes-platform/images/agl-image-compositor.bb

SUMMARY = "Baseline Flutter Image for Development"
LICENSE = "MIT"

CLANGSDK = "1"

IMAGE_INSTALL += "\
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

IMAGE_FEATURES += "\
    ssh-server-openssh \
    "
