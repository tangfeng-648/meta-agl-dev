require recipes-platform/images/agl-image-compositor.bb

SUMMARY = "Baseline Flutter Image for Development"
LICENSE = "MIT"

CLANGSDK = "1"

IMAGE_INSTALL += "\
    weston-ini-conf-landscape \
    \
    flutter-auto-verbose-logs \
    \
    flutter-engine-sdk-dev \
    \
    flutter-gallery \
    flutter-test-texture-egl \
    \
    "

IMAGE_FEATURES += "\
    ssh-server-openssh \
    "
