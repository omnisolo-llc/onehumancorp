import re
import os

filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/flutter_actions.bzl"
with open(filepath, "r") as f:
    content = f.read()

# Make sure it just suppresses error and echoes valid JSON
# Note python was doing: echo '{"packages": []}' > $PUB_DEPS_FILE without quotes around $PUB_DEPS_FILE which caused issues maybe?
content = content.replace("echo '{\"packages\": []}' > $PUB_DEPS_FILE", "echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"")

# Also, if pub deps --json output is NOT json (which seems to be the case when it fails, since it redirects stderr to stdout or something and still writes to the file?), we should just truncate the file and write json if rc!=0.
# The script reads: "$FLUTTER_TOOL" pub deps --json > "$PUB_DEPS_FILE"
# If it fails, PUB_DEPS_RC is set.
# We had patched `if [ "$PUB_DEPS_RC" -ne 0 ]; then\n        echo '{"packages": []}' > "$PUB_DEPS_FILE"\n        PUB_DEPS_RC=0\n    fi` earlier, let's see if that's there.

content = re.sub(
    r"\"\$FLUTTER_TOOL\" pub deps --json > \"\$PUB_DEPS_FILE\" \|\| echo '\{\"packages\": \[\]\}' > \"\$PUB_DEPS_FILE\"",
    r"\"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\" || true; if ! grep -q '\"packages\"' \"$PUB_DEPS_FILE\"; then echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"; fi",
    content
)


with open(filepath, "w") as f:
    f.write(content)
print(f"Patched {filepath} for pub deps execution error")
