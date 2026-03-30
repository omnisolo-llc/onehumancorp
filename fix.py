bzl_file = "MODULE.bazel"

with open("MODULE.bazel", "r") as f:
    content = f.read()

# Make sure we don't duplicate
if 'module_name = "rules_android"' not in content:
    override = """
single_version_override(
    module_name = "rules_android",
    patch_cmds = [
        "sed -i 's/@local_config_platform\\\\/\\\\/:constraints.bzl/@platforms\\\\/\\\\/host:constraints.bzl/g' rules/android_sdk_repository/helper.bzl",
        "find . -type f -name '*.bzl' -exec sed -i -E 's/\\\\bCcInfo\\\\b//g' {} +",
        "find . -type f -name '*.bzl' -exec sed -i -E 's/\\\\bCcInfo,//g' {} +",
        "find . -type f -name '*.bzl' -exec sed -i -E 's/, *CcInfo//g' {} +"
    ],
)
"""
    with open(bzl_file, "a") as f:
        f.write(override)
