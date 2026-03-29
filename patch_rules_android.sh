#!/bin/bash
for f in /home/jules/.cache/bazel/_bazel_jules/*/external/rules_android++android_sdk_repository_extension+androidsdk/helper.bzl; do
    sed -i 's/local_config_platform/platforms/' "$f"
done
