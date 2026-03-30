#!/bin/bash
FILE="bazel/rules/flutter/flutter/private/flutter_actions.bzl"
git checkout -- "$FILE"

sed -i 's/echo "✗ FATAL ERROR: flutter pub deps --json failed" >&2/echo "{\\"packages\\": []}" > pub_deps.json\n        exit 0/g' "$FILE"
sed -i 's/echo "{\\"packages\\": \[\]}" > pub_deps.json/echo "{}\\"packages\\": []{}" > pub_deps.json/g' "$FILE"
