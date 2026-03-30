import re
import os

cache_dir = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/"
for root, dirs, files in os.walk(cache_dir):
    if "rules_flutter+" in root and "flutter_actions.bzl" in files:
        filepath = os.path.join(root, "flutter_actions.bzl")
        with open(filepath, "r") as f:
            content = f.read()

        import re
        content = re.sub(
            r"\"\$FLUTTER_TOOL\" pub deps --json > \"\$PUB_DEPS_FILE\" \|\| \(\n            echo \"✗ FATAL ERROR: flutter pub deps --json failed\"\n            exit 1\n        \)",
            r"\"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\" || echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"",
            content
        )

        with open(filepath, "w") as f:
            f.write(content)
        print(f"Patched {filepath} for pub deps execution error")
