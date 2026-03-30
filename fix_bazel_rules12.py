import re
filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/package_generation.bzl"

with open(filepath, "r") as f:
    content = f.read()

content = content.replace("repository_ctx.file(pub_deps_file", "repository_ctx.file(\"pub_deps.json\"")

with open(filepath, "w") as f:
    f.write(content)
