FILESEXTRAPATHS_prepend := "${THISDIR}/files:"

IS_KERNEL_RECIPE := "${@bb.data.inherits_class('kernel', d) and 'yes' or 'no'}"

4A_KERNEL_SRC_URI_no = ""
4A_KERNEL_SRC_URI_yes = ""

4A_KERNEL_CONFIG_FRAGMENT_no = ""
4A_KERNEL_CONFIG_FRAGMENT_yes = ""

# ---------- 4a-sound.cfg ------------

4A_KERNEL_SRC_URI_yes += "file://4a-sound.cfg"
4A_KERNEL_CONFIG_FRAGMENT_yes += "${WORKDIR}/4a-sound.cfg"


# append to SRC_URI and KERNEL_CONFIG_FRAGMENTS (see meta-agl/meta-agl-bsp/recipes-kernel/linux/linux-agl.inc)

SRC_URI_append = " ${4A_KERNEL_SRC_URI_${IS_KERNEL_RECIPE}}"
KERNEL_CONFIG_FRAGMENTS_append = " ${4A_KERNEL_CONFIG_FRAGMENT_${IS_KERNEL_RECIPE}}"

