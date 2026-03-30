import os

def process_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()

        changed = False
        if 'load("@rules_cc//cc:defs.bzl", "CcInfo")' not in content and 'load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")' not in content and 'CcInfo' in content:
            content = 'load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")\n' + content
            changed = True

        if changed:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Patched {filepath}")
    except Exception as e:
        print(f"Failed to process {filepath}: {e}")

for root, dirs, files in os.walk('/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_android+/'):
    for file in files:
        if file.endswith('.bzl'):
            process_file(os.path.join(root, file))
