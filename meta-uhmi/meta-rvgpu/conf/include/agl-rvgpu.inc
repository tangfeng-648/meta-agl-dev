# As of now, we'd unconditionally append agl-rvgpu-proxy
# to all images belong to agl-rvgpu feature. In future
# it will be handled properly via multiconfig etc.

AGL_FEATURES:append = " agl-rvgpu-proxy"
