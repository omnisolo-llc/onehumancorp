#!/bin/bash
set -x

git restore pubspec.yaml
git restore srcs/app/pubspec.yaml
git restore bazel/rules/flutter/flutter/private/flutter_actions.bzl
git restore bazel/rules/flutter/flutter/private/package_generation.bzl

# ONLY patch package_generation.bzl
sed -i '/lower_stderr = stderr.lower()/a \
        if "workspace" in lower_stderr or "resolution \`workspace\`" in lower_stderr: \
            repository_ctx.report_progress( \
                "Skipping pub deps generation for {} due to workspace resolution error; falling back to pubspec.yaml".format(package_name), \
            ) \
            return False' bazel/rules/flutter/flutter/private/package_generation.bzl

# IF we need to remove resolution workspace, we do it safely inside `flutter_actions.bzl` without adding braces or semicolons that format() will break on.
# But wait, earlier I proved that just stripping `resolution: workspace` out of `srcs/app/pubspec.yaml` was enough, EXCEPT that caused a failure for `srcs/app/lib/models:pipeline_model` which said "srcs/app/pubspec.yaml is included in the workspace from ./pubspec.yaml, but does not have resolution: workspace".
# So Dart MUST see `resolution: workspace` on `srcs/app/pubspec.yaml`.

# The core problem is that `flutter pub deps` fails on `test_api` when running locally because it's part of a workspace. The `package_generation.bzl` patch handles that.
# Does `flutter pub get` fail in `flutter_actions.bzl` because of the workspace?
# Let's find out! We will just let `bazelisk test` run with ONLY the `package_generation.bzl` patch and see the actual error, because we lost the original error.
