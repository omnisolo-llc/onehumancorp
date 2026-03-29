#!/bin/bash
set -x

git restore bazel/rules/flutter/flutter/private/flutter_actions.bzl
git restore bazel/rules/flutter/flutter/private/package_generation.bzl

# The problem is `srcs/app/pubspec.yaml` is included in the workspace from `./pubspec.yaml`
# It says "but does not have `resolution: workspace`".
# And if we add it, `test_api` complains because we didn't patch `package_generation.bzl` to ignore `workspace` errors.

# Let's DO BOTH:
# 1. Restore `resolution: workspace` to our own pubspecs (or leave them alone if they have them)
git restore pubspec.yaml
git restore srcs/app/pubspec.yaml

# 2. Patch `package_generation.bzl` to ignore workspace errors during fetching external packages.
sed -i '/lower_stderr = stderr.lower()/a \
        if "workspace" in lower_stderr or "resolution \`workspace\`" in lower_stderr: \
            repository_ctx.report_progress( \
                "Skipping pub deps generation for {} due to workspace resolution error; falling back to pubspec.yaml".format(package_name), \
            ) \
            return False' bazel/rules/flutter/flutter/private/package_generation.bzl
