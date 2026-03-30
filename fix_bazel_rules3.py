import os
import glob

# Try finding the specific bazel cache directory
cache_dir = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_android++android_sdk_repository_extension+androidsdk/"

for root, dirs, files in os.walk(cache_dir):
    for f_name in files:
        if f_name.endswith(".bzl"):
            filepath = os.path.join(root, f_name)
            with open(filepath, "r") as f:
                content = f.read()
            if "@local_config_platform//:constraints.bzl" in content:
                content = content.replace("@local_config_platform//:constraints.bzl", "@platforms//host:constraints.bzl")
                with open(filepath, "w") as f:
                    f.write(content)
                print(f"Patched {filepath} for local_config_platform")
