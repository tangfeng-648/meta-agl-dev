# fix bug for ptest with usrmerge.
# Delete it after yocto version up.
do_compile_ptest:append() {
    sed -i 's,/usr/usr,/usr,g' \
        ${B}/tests/test_one \
        ${B}/tests/test_script

}

# fix bug for ptest with second running.
# Delete it after yocto version up.
do_install_ptest:append() {
    install -d ${D}${PTEST_PATH}/data
    install -m 0644 ${B}/tests/test_data.tmp ${D}${PTEST_PATH}/data/
    echo 'cp ../data/test_data.tmp ./' >> ${D}${PTEST_PATH}/run-ptest
}
