import re
import os

filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/flutter_actions.bzl"
with open(filepath, "r") as f:
    content = f.read()

# Python string reading empty JSON because we wrote it to PUB_DEPS_FILE which might not be used directly?
# The python script does `json.load(fp)`.
# Let's ensure the fallback works.
# Wait, the python script says: "File "<stdin>", line 22, in <module>"
# The python script is embedded inside `flutter_actions.bzl` itself!

content = re.sub(
    r"echo '\{\"packages\": \[\]\}' > \"\$PUB_DEPS_FILE\"",
    r"echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"; cat \"$PUB_DEPS_FILE\"",
    content, flags=re.DOTALL
)

with open(filepath, "w") as f:
    f.write(content)
print(f"Patched {filepath} for pub deps execution error")
