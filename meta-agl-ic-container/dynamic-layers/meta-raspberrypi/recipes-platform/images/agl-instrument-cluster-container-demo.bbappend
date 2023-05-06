WKS_FILES:remove = "agl-ic-container-noloader.wks agl-ic-container-noloader-demo.wks"
WKS_FILES:prepend = " \
    ${@bb.utils.contains('OUT_OF_TREE_CONTAINER_IMAGE_DEPLOY_DIR', 'non', 'agl-ic-container-bootpart-uuid.wks ', 'agl-ic-container-bootpart-uuid-demo.wks ', d)} \
"
