SUMMARY = "vhost CAN backend device"
DESCRIPTION = "A vhost-user backend that emulates a VirtIO CAN device"
HOMEPAGE = "https://gerrit.automotivelinux.org"

FILESEXTRAPATHS:prepend := "${THISDIR}:" 
EXTRAPATHS:prepend := "${THISDIR}:" 

SRC_URI = " file://. "

LICENSE = "Apache-2.0"
LIC_FILES_CHKSUM = "\
    file://LICENSE;md5=d41d8cd98f00b204e9800998ecf8427e \
"

inherit cargo
inherit pkgconfig

include vhost-device-can-crates.inc
