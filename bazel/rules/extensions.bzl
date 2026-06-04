load("//bazel/rules:proto_deps.bzl", "proto_deps")

def _proto_deps_impl(module_ctx):
    proto_deps()

proto_deps_extension = module_extension(
    implementation = _proto_deps_impl,
)
