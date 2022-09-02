# AGL test support layer

This yocto layer adds the feature 'agl-test'

'agl-test' is used to compile the test sets and test framework into the
image, make it more convenient to test target.

For running test using 'agl-test', refer to:
    https://git.automotivelinux.org/src/agl-test-framework/tree/README

Now it's only support the target agl-demo-platform(ivi)

## Setup

Enable the  `agl-test` AGL feature when setting up your build environment
with aglsetup.sh.

This will add the `packagegroup-agl-extend-test` packagegroup to the image,
For specific packages to be added, please refer to the file:
    ./recipes-platform/packagegroup/packagegroup-agl-extend-test.bb
