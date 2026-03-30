import re

with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "r") as f:
    content = f.read()

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

content = re.sub(r'        if \(\n            "path" in lower_stderr.*?\n            return False', replacement, content, flags=re.DOTALL)

with open("bazel/rules/flutter/flutter/private/package_generation.bzl", "w") as f:
    f.write(content)
