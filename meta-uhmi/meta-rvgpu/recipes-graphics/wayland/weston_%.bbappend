require ${@bb.utils.contains('AGL_FEATURES', 'agl-rvgpu-proxy', 'weston_rvgpuproxy.inc', '', d)}
