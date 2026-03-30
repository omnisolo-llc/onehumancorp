#!/bin/bash
sed -i 's/@local_config_platform\/\/:constraints.bzl/@platforms\/\/host:constraints.bzl/g' rules/android_sdk_repository/helper.bzl
find rules/ -name "*.bzl" -type f -exec sed -i '1s/^/load("@rules_cc\/\/cc\/common:cc_info.bzl", "CcInfo")\n/' {} +
