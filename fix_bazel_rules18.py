import re
import os

filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/flutter_actions.bzl"
with open(filepath, "r") as f:
    content = f.read()

# Fix the JSON parser failure:
# The python script inside flutter_actions.bzl reads the pub_deps.json file.
# If it's empty, it fails. We output {"packages": []} not an empty file.
# Wait, the error says it's empty. Let's check how we wrote the echo.
# "echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\""
# Let's make sure it's valid JSON and the python script reads it.
content = content.replace("echo '{\"packages\": []}'", "echo '{\\\"packages\\\": []}'")

with open(filepath, "w") as f:
    f.write(content)
print(f"Patched {filepath} for pub deps JSON empty error")
