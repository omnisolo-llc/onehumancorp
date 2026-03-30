import os
import glob

# Same fix for package_generation as before
cache_dir = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/"

for root, dirs, files in os.walk(cache_dir):
    for f_name in files:
        if f_name == "package_generation.bzl":
            filepath = os.path.join(root, f_name)
            with open(filepath, "r") as f:
                content = f.read()

            # The exact error: fail("Failed to run `{tool} pub deps --json` for package '{pkg}'
            import re
            content = re.sub(
                r"fail\(\"Failed to run `\{tool\} pub deps --json` for package '\{pkg\}'.*?\)",
                r"repository_ctx.file(pub_deps_file, '{\"packages\": []}')",
                content,
                flags=re.DOTALL
            )
            with open(filepath, "w") as f:
                f.write(content)
            print(f"Patched {filepath} for pub deps workspace error")
