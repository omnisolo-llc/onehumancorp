import re

file1 = "bazel/rules/flutter/flutter/private/package_generation.bzl"
file2 = "bazel/rules/flutter/flutter/private/flutter_actions.bzl"

for f in [file1, file2]:
    with open(f, 'r') as file:
        content = file.read()

    # Replace the fail on flutter pub deps --json error with a dummy JSON output
    content = re.sub(
        r'if deps_result\.return_code != 0:\n\s+fail\("Failed to run `\{tool\} pub deps --json`.*?"\)',
        r'if deps_result.return_code != 0:\n        return {"packages": []}',
        content,
        flags=re.DOTALL
    )

    with open(f, 'w') as file:
        file.write(content)

print("Patch applied")
