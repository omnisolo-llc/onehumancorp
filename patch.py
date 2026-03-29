import os
import glob
for f in glob.glob("/home/jules/.cache/bazel/_bazel_jules/*/external/rules_android++android_sdk_repository_extension+androidsdk/*.bzl"):
    with open(f, 'r') as file:
        content = file.read()
    content = content.replace('HOST_CONSTRAINTS = []', '')
    content = content.replace('load("@rules_shell//shell:sh_binary.bzl", "sh_binary")', 'load("@rules_shell//shell:sh_binary.bzl", "sh_binary")\nHOST_CONSTRAINTS = []')
    with open(f, 'w') as file:
        file.write(content)
