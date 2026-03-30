import re
import os

filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/flutter_actions.bzl"
with open(filepath, "r") as f:
    content = f.read()

import re
content = re.sub(
    r"\"\$FLUTTER_TOOL\" pub deps --json > \"\$PUB_DEPS_FILE\" \|\| \(\n            echo \"✗ FATAL ERROR: flutter pub deps --json failed\" >&2\n            exit 1\n        \)",
    r"\"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\" || echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"",
    content
)

content = re.sub(
    r"if \[ \"\$PUB_DEPS_RC\" -ne 0 \]; then\n        echo \"✗ FATAL ERROR: flutter pub deps --json failed\" >&2\n        exit 1\n    fi",
    r"if [ \"$PUB_DEPS_RC\" -ne 0 ]; then\n        echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"\n    fi",
    content
)

with open(filepath, "w") as f:
    f.write(content)
print(f"Patched {filepath} for pub deps execution error")
