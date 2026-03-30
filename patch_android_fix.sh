#!/bin/bash
find /home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_android+ -name "*.bzl" | xargs grep -l "CcInfo" | while read file; do
    if ! grep -q "@rules_cc//cc/common:cc_info.bzl" "$file"; then
        sed -i '1s/^/load("@rules_cc\/\/cc\/common:cc_info.bzl", "CcInfo")\n/' "$file"
    fi
done
