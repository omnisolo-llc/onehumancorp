with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "r") as f:
    content = f.read()
content = content.replace('"version solving failed" in lower_stderr', '"version solving failed" in lower_stderr or "workspace" in lower_stderr')
with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "w") as f:
    f.write(content)
