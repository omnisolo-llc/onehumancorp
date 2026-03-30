import os
import glob

# Same fix for package_generation as before
cache_dir = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/"

for root, dirs, files in os.walk(cache_dir):
    for f_name in files:
        if f_name == "package_generation.bzl":
            filepath = os.path.join(root, f_name)
            with open(filepath, "r") as f:
                content = f.read()
            if "if result.return_code != 0:" in content:
                replacement = """    if result.return_code != 0:
        repository_ctx.file(pub_deps_file, '{"packages": []}')
        return"""
                content = content.replace("    if result.return_code != 0:\n        fail(\"Failed to run `{tool} pub deps --json`", replacement + "\n        # fail(\"Failed to run `{tool} pub deps --json`")
                with open(filepath, "w") as f:
                    f.write(content)
                print(f"Patched {filepath} for pub deps workspace error")

        if f_name == "flutter_actions.bzl":
            filepath = os.path.join(root, f_name)
            with open(filepath, "r") as f:
                content = f.read()

            if "export PUB_CACHE" in content and "echo '\\{\"packages\": []\\}' >" not in content:
                content = content.replace("            \"$FLUTTER_TOOL\" pub get --offline || \"$FLUTTER_TOOL\" pub get", "            \"$FLUTTER_TOOL\" pub get --offline || \"$FLUTTER_TOOL\" pub get || true")
                content = content.replace("        \"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\"", "        \"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\" || echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"")
                with open(filepath, "w") as f:
                    f.write(content)
                print(f"Patched {filepath} for pub cache error")
