FILESEXTRAPATHS:prepend := "${THISDIR}/files:"

#
# Patch set for 3.3.7 Engine
#
FLUTTER_ENGINE_PATCHES = "\
    file://0001-clang-toolchain.patch \
    file://0002-x64-sysroot-assert.patch \
    file://0003-remove-x11-dependency.patch \
    file://0004-prevent-redefinition-of-glib_autoptr_clear_AtkObject.patch \
    "
