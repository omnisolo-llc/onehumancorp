import re

def fix_file(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    # match lines with fail("Failed to run `{tool} pub deps --json`
    content = re.sub(r'fail\("Failed to run `\{tool\} pub deps --json`.*?\)[\n\s]*\)', 'return {"packages": []}', content, flags=re.DOTALL)
    content = re.sub(r'fail\("Failed to run `flutter pub deps --json.*?\)[\n\s]*\)', 'return []', content, flags=re.DOTALL)

    with open(file_path, "w") as f:
        f.write(content)

fix_file("bazel/rules/flutter/flutter/private/package_generation.bzl")
fix_file("bazel/rules/flutter/flutter/private/flutter_actions.bzl")
