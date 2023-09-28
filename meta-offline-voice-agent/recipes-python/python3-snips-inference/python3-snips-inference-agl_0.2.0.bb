SUMMARY = "Inference only module of Snips NLU library"
HOMEPAGE = "https://github.com/malik727/snips-inference-agl"
LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "file://LICENSE;md5=ecab3ce0771d366e0f8e4ca058eb48e7"

SRC_URI += " \
    git://github.com/malik727/snips-inference-agl.git;protocol=https;branch=main \
    "

PV = "0.2.0+git${SRCPV}"
S = "${WORKDIR}/git"
SRCREV = "6d45b8d814b1937251524dd8a1e8cac0d8cab5de"

DEPENDS += " \
    python3-setuptools-native \
    python3-numpy-native \
    python3-numpy \
    python3-num2words \
    python3-pyaml \
    python3-six \
    python3-requests \
    python3-deprecation \
    python3-future \
    python3-scipy \
    python3-scikit-learn \
    python3-threadpoolctl \
    python3-sklearn-crfsuite \
    python3-tqdm \
    python3-packaging \
    python3-snips-nlu-utils \
    python3-snips-nlu-parsers \
    "

inherit setuptools3

RDEPENDS:${PN} += " \
    python3-numpy \
    python3-num2words \
    python3-pyaml \
    python3-six \
    python3-requests \
    python3-deprecation \
    python3-future \
    python3-scipy \
    python3-scikit-learn \
    python3-threadpoolctl \
    python3-sklearn-crfsuite \
    python3-tqdm \
    python3-packaging \
    python3-snips-nlu-utils \
    python3-snips-nlu-parsers \
    "
