import sys

file_path = "bazel/rules/flutter/flutter/private/package_generation.bzl"
with open(file_path, "r") as f:
    content = f.read()

target = """        if (
            "path" in lower_stderr and (
                "could not find package" in lower_stderr or
                "which doesn't exist" in lower_stderr
            )
        ) or (
            "from sdk" in lower_stderr and (
                "doesn't match any versions" in lower_stderr or
                "doesn't exist" in lower_stderr
            )
        ) or (
            # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            return False"""

replacement = """        if (
            "path" in lower_stderr and (
                "could not find package" in lower_stderr or
                "which doesn't exist" in lower_stderr
            )
        ) or (
            "from sdk" in lower_stderr and (
                "doesn't match any versions" in lower_stderr or
                "doesn't exist" in lower_stderr
            )
        ) or (
            # Catch-all for SDK version-solving failures (e.g. _macros not in SDK).
            "version solving failed" in lower_stderr
        ) or (
            "workspace" in lower_stderr or
            "read-only file system" in lower_stderr
        ):
            repository_ctx.report_progress(
                "Skipping pub deps generation for {} due to unsupported dependency source; falling back to pubspec.yaml".format(package_name),
            )
            repository_ctx.file(pub_deps_rel, '{"packages": []}')
            return False"""

if target in content:
    with open(file_path, "w") as f:
        f.write(content.replace(target, replacement))
else:
    print("Target not found")
