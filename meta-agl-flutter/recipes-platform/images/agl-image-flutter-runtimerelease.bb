SUMMARY = "Baseline Flutter Image for Release"

LICENSE = "MIT"

require agl-image-flutter.inc

IMAGE_INSTALL:append = "\
    weston-ini-conf-landscape \
    \
    flutter-auto-runtimerelease \
    \
    flutter-gallery-runtimerelease \
    flutter-gallery-runtimerelease-init \
    flutter-test-texture-egl-runtimerelease \
    flutter-test-secure-storage-runtimerelease \
    flutter-test-localization-runtimerelease \
    "
