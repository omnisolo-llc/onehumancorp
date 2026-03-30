import sys
import os

file_path = "bazel/rules/flutter/flutter/private/flutter_actions.bzl"
if os.path.exists(file_path):
    with open(file_path, "r") as f:
        content = f.read()

    search_str = """    if ctx.attr.pub_deps == None:
        fail("pub_deps output must be specified when generating dependencies.")

    # 1. Run flutter pub deps --json inside the workspace."""

    if search_str in content:
        # We don't necessarily need to patch flutter_actions.bzl if we only care about rules_flutter++pub+pub_test_api
        pass
else:
    print(f"{file_path} not found")
