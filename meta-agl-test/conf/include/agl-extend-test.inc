# Set default mode to "standard"
AGL_TEST_MODE ?= "standard"

IMAGE_FEATURES += "agl-test"
FEATURE_PACKAGES_agl-test = "packagegroup-agl-extend-test-${AGL_TEST_MODE}"
