require recipes-platform/images/agl-ivi-demo-platform.bb

IMAGE_INSTALL:append = "python3-vosk-api vosk vosk-server gcc make cmake autoconf automake gcc-c++ boost-dev"