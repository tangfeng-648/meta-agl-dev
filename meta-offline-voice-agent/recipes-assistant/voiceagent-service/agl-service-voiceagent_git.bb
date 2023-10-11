SUMMARY = "A gRPC-based voice agent service designed for Automotive Grade Linux (AGL)."
HOMEPAGE = "https://github.com/malik727/agl-service-voiceagent"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=4202492ed9afcab3aaecc4a9ec32adb2"

SRC_URI = " \
    git://github.com/malik727/agl-service-voiceagent;protocol=https;branch=main \
"

SRCREV = "${AUTOREV}"
S = "${WORKDIR}/git"

DEPENDS += " \
    python3 \
    python3-setuptools-native \
    python3-grpcio-tools-native \
    "

inherit setuptools3

do_compile:prepend() {
    # Generate proto files and move them to 'generated/' directory
    python3 -m grpc_tools.protoc -I${S}/agl_service_voiceagent/protos --python_out=${S}/agl_service_voiceagent/generated --grpc_python_out=${S}/agl_service_voiceagent/generated voice_agent.proto
}

RDEPENDS:${PN} += " \
    python3-numpy \
    python3-grpcio \
    python3-grpcio-tools \
    python3-pygobject \
    kuksa-client \
    python3-rasa \
    python3-snips-inference-agl \
    vosk \
    "
