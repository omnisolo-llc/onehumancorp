import re
import os

filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/flutter_actions.bzl"
with open(filepath, "r") as f:
    content = f.read()

# Make sure it actually executes `echo '{"packages": []}' > "$PUB_DEPS_FILE"`
content = content.replace("cat \"$PUB_DEPS_FILE\"", "cat \"$PUB_DEPS_FILE\" > /dev/null")

with open(filepath, "w") as f:
    f.write(content)
