#!/bin/bash
set -x

git restore bazel/rules/flutter/flutter/private/flutter_actions.bzl

# The problem is that the python script needs proper indentation and `{}` escaping.
# In `flutter_actions.bzl`, I added `{{}}` escaping but it didn't like it.
# Actually, the python block starts and ends inside a `format()` call. I should not inject anything there that uses `{}` without escaping it correctly.
# Where is `format()` called? At the end of `script_content`.

# Let's just create a new wrapper script that gets executed instead.

sed -i '/cd "$ORIGINAL_PWD"/a \
find "$WORKSPACE_DIR_ABS" -name pubspec.yaml -exec sed -i '\''s/resolution: *workspace//g'\'' {} + || true' bazel/rules/flutter/flutter/private/flutter_actions.bzl
