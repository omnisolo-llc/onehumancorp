#!/bin/bash
set -x

git restore pubspec.yaml
git restore srcs/app/pubspec.yaml
git restore bazel/rules/flutter/flutter/private/flutter_actions.bzl
git restore bazel/rules/flutter/flutter/private/package_generation.bzl

# The problem is `srcs/app/pubspec.yaml` is included in the workspace from `./pubspec.yaml`
# It says "but does not have `resolution: workspace`".
# If we simply remove `workspace:` from `pubspec.yaml`, then it won't complain!

sed -i '/workspace:/d' pubspec.yaml
sed -i '/- srcs\/app/d' pubspec.yaml
sed -i 's/resolution: *workspace//g' srcs/app/pubspec.yaml
