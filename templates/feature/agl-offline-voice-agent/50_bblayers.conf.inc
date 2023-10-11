# agl-offline-voice-agent

AGL_META_PYTHON = "${METADIR}/external/meta-openembedded/meta-python"

BBLAYERS =+ " \
    ${METADIR}/meta-agl-devel/meta-offline-voice-agent \
    ${METADIR}/external/meta-tensorflow \
"
