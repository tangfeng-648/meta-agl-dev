SUMMARY = "Baseline Flutter Image for Profiling"

LICENSE = "MIT"

require agl-image-flutter.inc

IMAGE_INSTALL:append = "\
    weston-ini-conf-landscape \
    \
    flutter-auto-runtimeprofile \
    \
    flutter-gallery-runtimeprofile \
    flutter-gallery-runtimeprofile-init \
    flutter-test-texture-egl-runtimeprofile \
    flutter-test-secure-storage-runtimeprofile \
    flutter-test-localization-runtimeprofile \
    "
