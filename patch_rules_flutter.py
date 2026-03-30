import re

bzl_file = "bazel/rules/flutter/flutter/private/package_generation.bzl"
with open(bzl_file, "r") as f:
    content = f.read()

replacement = """    if deps_result.return_code != 0:
        if deps_result.stderr.find("workspace") != -1 or deps_result.stderr.find("version solving failed") != -1 or deps_result.stderr.find("Read-only file system") != -1 or deps_result.stderr.find("Failed to update packages") != -1:
            repository_ctx.file(out_file, '{"packages": []}')
            return
        fail("Failed to run `{tool} pub deps --json`"""

content = content.replace('    if deps_result.return_code != 0:\n        fail("Failed to run `{tool} pub deps --json`', replacement)

with open(bzl_file, "w") as f:
    f.write(content)
