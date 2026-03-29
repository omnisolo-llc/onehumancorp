#!/bin/bash
set -x

# The error now:
# "Because ffi depends on dart_flutter_team_lints >=3.5.2 which requires SDK version ^3.8.0, version solving failed. The current Dart SDK version is 3.7.2."
# The `ffi` package resolving requires dart ^3.8.0 which means we are using a version of `ffi` that is too new for Flutter 3.29.3.

# We must pin `ffi` or remove it if possible. Let's pin it.
# In `bazel/rules/flutter/MODULE.bazel`:

sed -i '/pub.package(/a \
    name = "pub_ffi",\n    package = "ffi",\n    version = "2.1.2",\n)' bazel/rules/flutter/MODULE.bazel
sed -i 's/pub.package(/pub.package(/g' bazel/rules/flutter/MODULE.bazel # just formatting
# Wait, let's just use bazel mod tidy or append it cleanly:

cat << 'INNER_EOF' >> bazel/rules/flutter/MODULE.bazel
pub.package(
    name = "pub_ffi",
    package = "ffi",
    version = "2.1.2",
)
use_repo(pub, "pub_ffi")
INNER_EOF
