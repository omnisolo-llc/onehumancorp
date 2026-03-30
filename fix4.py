import sys

file_path = "bazel/rules/flutter/flutter/private/package_generation.bzl"
with open(file_path, "r") as f:
    content = f.read()

target = """            repository_ctx.file(pub_deps_rel, '{"packages": []}')
            return False
            return False"""

replacement = """            repository_ctx.file(pub_deps_rel, '{"packages": []}')
            return False"""

if target in content:
    with open(file_path, "w") as f:
        f.write(content.replace(target, replacement))
else:
    print("Not found")
