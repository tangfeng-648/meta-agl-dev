# Helper class for container manager config creation.
# Assumes that:
# - Recipe name is 'cm-config-' + <guest name>
# - Corresponding files {config,system.conf}.<guest name>.in are in
#   the file search path
# - That references to the DRM lease device name are parameterized
#   with @DRM_LEASE_DEVICE@ in the .in files

python __anonymous() {
    bpn = d.getVar('BPN')
    if not bpn.startswith('cm-config-'):
        bb.error('Recipe name does not start with \'cm-config-\'')
    config = bpn[10:]
    d.setVar('CM_CONFIG_NAME', config)
}

S = "${WORKDIR}"

inherit allarch

DRM_LEASE_DEVICE ??= "card0-HDMI-A-1"

do_configure[noexec] = "1"
do_compile[noexec] = "1"

do_install:append () {
    install -m 0755 -d ${D}/opt/container/guests/${CM_CONFIG_NAME}
    for f in system.conf.${CM_CONFIG_NAME}.in; do
        sed -e 's|@DRM_LEASE_DEVICE@|${DRM_LEASE_DEVICE}|g' \
            ${WORKDIR}/$f > ${D}/opt/container/guests/${CM_CONFIG_NAME}/${f%.${CM_CONFIG_NAME}.in}
    done
}

FILES:${PN}:append = "/opt/container/guests/${CM_CONFIG_NAME}"
