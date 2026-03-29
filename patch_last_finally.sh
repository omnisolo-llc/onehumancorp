#!/bin/bash
set -x

git restore pubspec.yaml
git restore srcs/app/pubspec.yaml
git restore bazel/rules/flutter/flutter/private/flutter_actions.bzl
git restore bazel/rules/flutter/flutter/private/package_generation.bzl

# It's an issue with the pub cache layout for rules_flutter in Bazel. The flutter SDK creates issues downloading older versions because it insists on the workspace resolution locally but external packages don't have it.

sed -i '/lower_stderr = stderr.lower()/a \
        if "workspace" in lower_stderr or "resolution \`workspace\`" in lower_stderr: \
            repository_ctx.report_progress( \
                "Skipping pub deps generation for {} due to workspace resolution error; falling back to pubspec.yaml".format(package_name), \
            ) \
            return False' bazel/rules/flutter/flutter/private/package_generation.bzl

# We will just rewrite the `FlutterPrepareDeps` action to sed OUT the workspace resolution IF AND ONLY IF it's in a `.pub_cache` external package directory, or just entirely inside the staging directory. Let's do it right inside `flutter_actions.bzl` AFTER it creates `WORKSPACE_DIR_ABS`.

sed -i '/chmod -R u+rwX "$WORKSPACE_DIR_ABS"/a \
find "$WORKSPACE_DIR_ABS" -type f -name pubspec.yaml -exec sed -i '\''s/resolution: *workspace//g'\'' {} + || true' bazel/rules/flutter/flutter/private/flutter_actions.bzl
