#!/bin/bash
git checkout MODULE.bazel
cat << 'INNER_EOF' >> MODULE.bazel

bazel_dep(name = "rules_android", version = "0.1.1")
single_version_override(
    module_name = "rules_android",
    patch_cmds = [
        "sed -i 's|@local_config_platform//:constraints.bzl|@platforms//host:constraints.bzl|g' rules/android_sdk_repository/helper.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/android_local_test/attrs.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/android_library/impl.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/native_deps.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/android_binary/attrs.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/android_binary/impl.bzl",
        "sed -i '1s|^|load(\"@rules_cc//cc/common:cc_info.bzl\", \"CcInfo\")\\n|' rules/android_library/attrs.bzl",
    ],
)
INNER_EOF
