import re

with open("MODULE.bazel", "r") as f:
    content = f.read()

content = re.sub(r'single_version_override\(\s*module_name\s*=\s*"rules_android"[^\)]+\)\n?', '', content, flags=re.MULTILINE)

content += """
single_version_override(
    module_name = "rules_android",
    patch_cmds = [
        "sed -i '1i load(\\"@rules_cc//cc/common:cc_info.bzl\\", \\"CcInfo\\")' rules/android_local_test/attrs.bzl",
        "sed -i '1i load(\\"@rules_cc//cc/common:cc_info.bzl\\", \\"CcInfo\\")' rules/android_library/impl.bzl",
        "sed -i '1i load(\\"@rules_cc//cc/common:cc_info.bzl\\", \\"CcInfo\\")' rules/android_library/attrs.bzl",
        "sed -i '1i load(\\"@rules_cc//cc/common:cc_info.bzl\\", \\"CcInfo\\")' rules/native_deps.bzl",
        "sed -i '1i load(\\"@rules_cc//cc/common:cc_info.bzl\\", \\"CcInfo\\")' rules/android_binary/attrs.bzl",
    ],
)
"""

with open("MODULE.bazel", "w") as f:
    f.write(content)
