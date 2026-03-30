import sys

file_path = "bazel/rules/flutter/flutter/private/package_generation.bzl"
with open(file_path, "r") as f:
    content = f.read()

search_str = """        # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            return False"""

replace_str = """        # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ) or (
            "workspace" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            repository_ctx.file(pub_deps_rel, '{"packages": []}')
            return False"""

if search_str in content:
    content = content.replace(search_str, replace_str)
    with open(file_path, "w") as f:
        f.write(content)
    print("Patched package_generation.bzl successfully")
else:
    print("Could not find search string in package_generation.bzl")
