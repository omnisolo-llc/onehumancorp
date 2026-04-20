load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

_GOOGLE_PROTOBUF_DART_BUILD = "\n".join([
    'package(default_visibility = ["//visibility:public"])',
    "",
    "filegroup(",
    '    name = "protoc_plugin_srcs",',
    '    srcs = glob(["**"]),',
    ")",
    "",
    "filegroup(",
    '    name = "protoc_plugin_files",',
    '    srcs = glob(["**"]),',
    ")",
])

def proto_deps():
    http_archive(
        name = "google_protobuf_dart",
        sha256 = "cae253935cb6d372a3df34bbbe82c3044f067e088d5216ac1f645c36b596a6ad",
        strip_prefix = "protobuf.dart-protoc_plugin-v25.0.0/protoc_plugin",
        urls = ["https://github.com/google/protobuf.dart/archive/refs/tags/protoc_plugin-v25.0.0.tar.gz"],
        build_file_content = _GOOGLE_PROTOBUF_DART_BUILD,
    )
