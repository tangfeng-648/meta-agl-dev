# fix bug for ptest with usrmerge.
# Delete it after yocto version up.
do_compile_ptest:append() {
    sed -i 's,/usr/usr,/usr,g' \
        ${B}/tests/test_one \
        ${B}/tests/test_script

}
