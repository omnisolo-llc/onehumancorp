#!/bin/bash
patch_file="/home/jules/.cache/bazel/_bazel_jules/*/external/rules_flutter+/flutter/private/package_generation.bzl"
for f in $patch_file; do
    sed -i -e '/repository_ctx\.file("pub_deps\.json", "{\\"packages\\": \[\]}")/,+6d' "$f"
    sed -i 's/return False/repository_ctx.file("pub_deps.json", "{\\"packages\\": []}")\n            return False/' "$f"
done
