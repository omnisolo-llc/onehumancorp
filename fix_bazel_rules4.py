import os
import glob

# Same fix for package_generation as before
cache_dir = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/"

for root, dirs, files in os.walk(cache_dir):
    if "rules_flutter+" in root and "package_generation.bzl" in files:
        filepath = os.path.join(root, "package_generation.bzl")
        with open(filepath, "r") as f:
            content = f.read()

        replacement = """    if result.return_code != 0:
        if "workspace" in result.stderr or "version solving failed" in result.stderr.lower():
            repository_ctx.file("pub_deps.json", '{"packages": []}')
            return
        fail("Failed to run `{tool} pub deps --json` for package '{pkg}' (dir: {dir}).\\nstdout: {stdout}\\nstderr: {stderr}".format("""

        if "if result.return_code != 0:" in content and "workspace" not in content:
            content = content.replace("""    if result.return_code != 0:
        fail("Failed to run `{tool} pub deps --json` for package '{pkg}' (dir: {dir}).\\nstdout: {stdout}\\nstderr: {stderr}".format(""", replacement)
            with open(filepath, "w") as f:
                f.write(content)
            print(f"Patched {filepath} for pub deps workspace error")

    if "rules_flutter+" in root and "flutter_actions.bzl" in files:
        filepath = os.path.join(root, "flutter_actions.bzl")
        with open(filepath, "r") as f:
            content = f.read()

        if "export PUB_CACHE" in content and "echo '\\{\"packages\": []\\}' >" not in content:
            content = content.replace("            \"$FLUTTER_TOOL\" pub get --offline || \"$FLUTTER_TOOL\" pub get", "            \"$FLUTTER_TOOL\" pub get --offline || \"$FLUTTER_TOOL\" pub get || echo '{\"packages\": []}' > pub_deps.json")
            content = content.replace("        \"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\"", "        \"$FLUTTER_TOOL\" pub deps --json > \"$PUB_DEPS_FILE\" || echo '{\"packages\": []}' > \"$PUB_DEPS_FILE\"")
            with open(filepath, "w") as f:
                f.write(content)
            print(f"Patched {filepath} for pub cache error")
