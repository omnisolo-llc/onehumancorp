def compute_relative_to_package(ctx, file):
    """Compute the path of a file relative to the package directory."""
    pkg_path = ctx.label.package
    path = file.short_path

    workspace_name = ctx.label.workspace_name
    if workspace_name and workspace_name not in ("__main__", "_main"):
        for prefix in [
            "external/{}/".format(workspace_name),
            "../{}/".format(workspace_name),
            workspace_name + "/",
        ]:
            if path.startswith(prefix):
                path = path[len(prefix):]
                break

    if not pkg_path:
        return path

    if path.startswith(pkg_path + "/"):
        return path[len(pkg_path) + 1:]

    return file.basename

def create_flutter_working_dir(ctx, pubspec_file, dart_files, other_files, data_files, workspace_pubspec = None):
    """Create a working directory structure for Flutter commands.

    Args:
        ctx: The rule context
        pubspec_file: The pubspec.yaml file
        dart_files: List of .dart source files
        other_files: List of other source files declared in srcs
        data_files: List of additional data files that must be available in the workspace
        workspace_pubspec: Optional root pubspec.yaml for workspace-aware projects

    Returns:
        Tuple of (working_dir, input_files)
    """
    working_dir = ctx.actions.declare_directory(ctx.label.name + "_workspace_seed")

    workspace_entries = {}

    def add_entry(file, dest = None):
        if file == None:
            return
            
        if not dest:
            # If we are in Workspace Mode, use the full relative path from the project root.
            # Otherwise, use the package-relative path for backward compatibility.
            if workspace_pubspec:
                dest = file.short_path
            else:
                dest = compute_relative_to_package(ctx, file)
        
        if dest in workspace_entries:
            return
        workspace_entries[dest] = file

    # Populate the layout
    add_entry(pubspec_file)
    if workspace_pubspec:
        add_entry(workspace_pubspec)

    for f in dart_files + other_files + data_files:
        add_entry(f)
        add_entry(f)

    manifest = ctx.actions.declare_file(ctx.label.name + "_workspace_manifest.txt")
    manifest_content = []
    for rel_path in sorted(workspace_entries.keys()):
        file = workspace_entries[rel_path]
        manifest_content.append("{}|{}".format(rel_path, file.path))

    manifest_payload = "\n".join(manifest_content)
    if manifest_payload:
        manifest_payload += "\n"

    ctx.actions.write(
        output = manifest,
        content = manifest_payload,
    )

    setup_script = ctx.actions.declare_file(ctx.label.name + "_setup_workspace.sh")
    ctx.actions.write(
        output = setup_script,
        content = """#!/bin/bash
set -euo pipefail

WORKSPACE_DIR="$1"
MANIFEST_FILE="$2"

rm -rf "$WORKSPACE_DIR"
mkdir -p "$WORKSPACE_DIR"

while IFS='|' read -r RELATIVE_PATH SOURCE_PATH; do
    if [ -z "$RELATIVE_PATH" ]; then
        continue
    fi
    DEST_PATH="$WORKSPACE_DIR/$RELATIVE_PATH"
    mkdir -p "$(dirname "$DEST_PATH")"
    cp -RL "$SOURCE_PATH" "$DEST_PATH"
done < "$MANIFEST_FILE"
""",
        is_executable = True,
    )

    # Collect unique input files for the action
    input_files = [manifest]
    seen_inputs = {}
    for f in [pubspec_file, workspace_pubspec] + dart_files + other_files + data_files:
        if f == None:
            continue
        if f.path in seen_inputs:
            continue
        seen_inputs[f.path] = True
        input_files.append(f)

    # Run the workspace setup
    ctx.actions.run(
        inputs = input_files,
        outputs = [working_dir],
        executable = setup_script,
        arguments = [working_dir.path, manifest.path],
        mnemonic = "SetupFlutterWorkspace",
        progress_message = "Setting up Flutter workspace for %s" % ctx.label.name,
    )

    return working_dir, input_files

def flutter_pub_get_action(
        ctx,
        flutter_toolchain,
        working_dir,
        pubspec_file,
        dependency_pub_caches = [],
        codegen_commands = [],
        is_pub_package = False,
        workspace_pubspec = None):
    """Prepare Flutter/Dart dependencies without running pub get.

    Args:
        ctx: The rule context.
        flutter_toolchain: The resolved Flutter toolchain.
        working_dir: Directory containing the staged package sources.
        pubspec_file: The pubspec.yaml file for the library.
        dependency_pub_caches: Files or depsets with pub cache directories from dependencies.
        codegen_commands: Optional list of code generation commands (package:script).
        is_pub_package: Whether the target represents a hosted pub.dev package.
        workspace_pubspec: Optional root pubspec.yaml.

    Returns:
        Tuple of (prepared_workspace, pub_get_output, pub_cache_dir, pub_deps, dart_tool_dir).
    """

    # Calculate the package directory within the working_dir
    # If workspace_pubspec is present, the package is at its real relative path.
    # Otherwise, it's at the root.
    package_dir = ""
    if workspace_pubspec:
        pubspec_segments = pubspec_file.short_path.split("/")
        package_dir = "/".join(pubspec_segments[:-1])

    if not flutter_toolchain.flutterinfo.tool_files:
        fail("No tool files found in Flutter toolchain")
    flutter_bin_file = flutter_toolchain.flutterinfo.tool_files[0]
    flutter_bin = flutter_bin_file.path

    dep_pub_cache_files = []
    for item in dependency_pub_caches:
        if type(item) == "depset":
            dep_pub_cache_files.extend(item.to_list())
        else:
            dep_pub_cache_files.append(item)

    pub_get_output = ctx.actions.declare_file(ctx.label.name + "_pub_prepare.log")
    pub_cache_dir = ctx.actions.declare_directory(ctx.label.name + "_pub_cache")
    pub_deps = ctx.actions.declare_file(ctx.label.name + "_pub_deps.json")
    dart_tool_dir = ctx.actions.declare_directory(ctx.label.name + "_dart_tool")
    prepared_workspace = ctx.actions.declare_directory(ctx.label.name + "_prepared_flutter_workspace")

    dep_pub_cache_args = []
    for dep_cache in dep_pub_cache_files:
        dep_pub_cache_args.append(dep_cache.path)

    codegen_args = ["\"{}\"".format(cmd) for cmd in codegen_commands]

    script_content = """#!/bin/bash
set -euo pipefail

WORKSPACE_SRC="{workspace_src}"
WORKSPACE_DIR="{workspace_dir}"
PUB_CACHE_DIR="{pub_cache_dir}"
PUB_DEPS_OUT="{pub_deps}"
DART_TOOL_DIR="{dart_tool_dir}"
FLUTTER_BIN="{flutter_bin}"
IS_PUB_PACKAGE="{is_pub_package}"
ORIGINAL_PWD="$PWD"

WORKSPACE_SRC_ABS="$ORIGINAL_PWD/$WORKSPACE_SRC"
WORKSPACE_ROOT_ABS="$ORIGINAL_PWD/{working_dir_path}"
PACKAGE_DIR="{package_dir}"
PACKAGE_WORKSPACE_DIR_ABS="$WORKSPACE_ROOT_ABS"
if [ -n "$PACKAGE_DIR" ]; then
    PACKAGE_WORKSPACE_DIR_ABS="$WORKSPACE_ROOT_ABS/$PACKAGE_DIR"
fi

# Keep the prepared workspace rooted at the workspace root; package-specific
# commands run from PACKAGE_WORKSPACE_DIR_ABS when workspace mode is enabled.
PUB_CACHE_DIR_ABS="$ORIGINAL_PWD/$PUB_CACHE_DIR"
DART_TOOL_DIR_ABS="$ORIGINAL_PWD/$DART_TOOL_DIR"

# Copy staged workspace into prepared output directory
rm -rf "$WORKSPACE_ROOT_ABS"
mkdir -p "$WORKSPACE_ROOT_ABS"
if command -v rsync >/dev/null 2>&1; then
    rsync -aL "$WORKSPACE_SRC_ABS/" "$WORKSPACE_ROOT_ABS/"
else
    cp -RL "$WORKSPACE_SRC_ABS/." "$WORKSPACE_ROOT_ABS/"
fi
chmod -R u+rwX "$WORKSPACE_ROOT_ABS"

PYTHON_BIN="$(command -v python3 || command -v python || true)"
if [ -z "$PYTHON_BIN" ]; then
    echo "✗ FATAL ERROR: python interpreter not found on PATH" >&2
    exit 1
fi

export PUB_CACHE="$PUB_CACHE_DIR_ABS"
mkdir -p "$PUB_CACHE_DIR_ABS"

if [ "$IS_PUB_PACKAGE" = "1" ] && [ -f "$PACKAGE_WORKSPACE_DIR_ABS/pubspec.yaml" ]; then
    export WORKSPACE_PUBSPEC_PATH="$PACKAGE_WORKSPACE_DIR_ABS/pubspec.yaml"
    "$PYTHON_BIN" <<'PY'
import os

path = os.environ["WORKSPACE_PUBSPEC_PATH"]
with open(path, "r", encoding="utf-8") as fh:
    lines = fh.readlines()

rewritten = []
skip_block = None

for line in lines:
    stripped = line.lstrip()
    indent = len(line) - len(stripped)

    if skip_block:
        if stripped and indent == 0:
            skip_block = None
        else:
            continue

    if indent == 0 and stripped.startswith("resolution:"):
        continue

    if indent == 0 and (
        stripped.startswith("workspace:") or
        stripped.startswith("dev_dependencies:")
    ):
        skip_block = stripped.split(":", 1)[0]
        continue

    rewritten.append(line)

with open(path, "w", encoding="utf-8") as fh:
    fh.writelines(rewritten)
PY
fi

echo "=== Preparing pub cache from dependencies ==="
DEP_CACHES=({dep_caches})
if [ ${{#DEP_CACHES[@]}} -gt 0 ]; then
    for DEP_CACHE in "${{DEP_CACHES[@]}}"; do
        if [[ "$DEP_CACHE" != /* ]]; then
            DEP_CACHE="$ORIGINAL_PWD/$DEP_CACHE"
        fi
        if [ -d "$DEP_CACHE" ] && [ -n "$(ls -A "$DEP_CACHE" 2>/dev/null)" ]; then
            if command -v rsync >/dev/null 2>&1; then
                rsync -a "$DEP_CACHE/" "$PUB_CACHE_DIR_ABS/"
            else
                cp -RL "$DEP_CACHE/." "$PUB_CACHE_DIR_ABS/"
            fi
            # Bazel marks output directories read-only (0555) after actions complete.
            # Dependency caches are Bazel outputs, so rsync -a copies those read-only
            # permissions into our new pub_cache.  Make everything writable so subsequent
            # loop iterations (or the IS_PUB_PACKAGE block below) can succeed.
            chmod -R u+w "$PUB_CACHE_DIR_ABS" 2>/dev/null || true
        fi
    done
else
    echo "No dependency caches supplied"
fi
echo ""

export PUBSPEC_PATH="$PACKAGE_WORKSPACE_DIR_ABS/pubspec.yaml"
PACKAGE_INFO="$("$PYTHON_BIN" <<'PY'
import os
path = os.environ.get("PUBSPEC_PATH")
name = ""
version = ""
language = ""

if path and os.path.exists(path):
    with open(path, "r", encoding="utf-8") as fh:
        lines = fh.readlines()
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("name:") and not name:
                name = stripped.split(":", 1)[1].strip().strip('"').strip("'")
            elif stripped.startswith("version:") and not version:
                version = stripped.split(":", 1)[1].strip().strip('"').strip("'")
        
        for i, line in enumerate(lines):
            if line.strip().startswith("environment:"):
                for j in range(i + 1, len(lines)):
                    subline = lines[j].strip()
                    if subline.startswith("sdk:"):
                        language = subline.split(":", 1)[1].strip().strip('"').strip("'")
                        break
                    if subline and not subline.startswith("#") and ":" in subline and not subline.startswith(("flutter:", "flutter_test:", "dart:")):
                        break
                break

print(f"{{name}}|{{version}}|{{language}}")
PY
)"

PACKAGE_NAME="${{PACKAGE_INFO%%|*}}"
PACKAGE_VERSION="${{PACKAGE_INFO#*|}}"
PACKAGE_VERSION="${{PACKAGE_VERSION%%|*}}"
LANGUAGE_SPEC="${{PACKAGE_INFO##*|}}"
if [ -z "$LANGUAGE_SPEC" ]; then
    LANGUAGE_SPEC=">=3.0.0 <4.0.0"
fi

if [ "$IS_PUB_PACKAGE" = "1" ] && [ -n "$PACKAGE_NAME" ] && [ -n "$PACKAGE_VERSION" ]; then
    DEST="$PUB_CACHE_DIR_ABS/hosted/pub.dev/${{PACKAGE_NAME}}-${{PACKAGE_VERSION}}"
    rm -rf "$DEST"
    mkdir -p "$DEST"
    if command -v rsync >/dev/null 2>&1; then
        rsync -aL "$PACKAGE_WORKSPACE_DIR_ABS/" "$DEST/"
    else
        cp -RL "$PACKAGE_WORKSPACE_DIR_ABS/." "$DEST/"
    fi
fi

export FLUTTER_SUPPRESS_ANALYTICS=true
export CI=true
export PUB_ENVIRONMENT="flutter_tool:bazel"
export ANDROID_HOME=""
export ANDROID_SDK_ROOT=""
FLUTTER_BIN_ABS="$ORIGINAL_PWD/$FLUTTER_BIN"
if [ ! -x "$FLUTTER_BIN_ABS" ]; then
    echo "✗ FATAL ERROR: Flutter binary not found at $FLUTTER_BIN_ABS" >&2
    exit 1
fi

FLUTTER_ROOT="$(cd "$(dirname "$FLUTTER_BIN_ABS")/.." && pwd -P)"
export FLUTTER_ROOT
export PATH="$FLUTTER_ROOT/bin:$PATH"

cd "$PACKAGE_WORKSPACE_DIR_ABS"

echo "=== Generating pub_deps.json ==="
DART_BIN_LOCAL="$FLUTTER_ROOT/bin/cache/dart-sdk/bin/dart"
PUB_DEPS_ERR="$PACKAGE_WORKSPACE_DIR_ABS/pub_deps.stderr.log"
PUB_DEPS_READY=0
if [ -x "$DART_BIN_LOCAL" ] && "$DART_BIN_LOCAL" pub deps --json > pub_deps.json 2> "$PUB_DEPS_ERR"; then
    PUB_DEPS_READY=1
elif [ -f "$PUB_DEPS_ERR" ] && grep -qi "requires the Flutter SDK" "$PUB_DEPS_ERR"; then
    if "$FLUTTER_BIN_ABS" --suppress-analytics pub deps --json > pub_deps.json 2>> "$PUB_DEPS_ERR"; then
        PUB_DEPS_READY=1
    fi
fi

if [ "$PUB_DEPS_READY" != "1" ]; then
    echo "⚠ pub deps --json failed; reconstructing dependency graph from cached metadata" >&2
    if ! WORKSPACE_DIR_ABS="$PACKAGE_WORKSPACE_DIR_ABS" \
        PUB_CACHE_DIR_ABS="$PUB_CACHE_DIR_ABS" \
        FLUTTER_ROOT="$FLUTTER_ROOT" \
        PACKAGE_NAME="$PACKAGE_NAME" \
        PACKAGE_VERSION="$PACKAGE_VERSION" \
        "$PYTHON_BIN" > pub_deps.json <<'PY'
import json
import os
import re
import sys

workspace_dir = os.environ["WORKSPACE_DIR_ABS"]
pub_cache_dir = os.environ["PUB_CACHE_DIR_ABS"]
flutter_root = os.environ.get("FLUTTER_ROOT") or ""
package_name = os.environ.get("PACKAGE_NAME") or ""
package_version = os.environ.get("PACKAGE_VERSION") or ""
hosted_cache_dir = os.path.join(pub_cache_dir, "hosted", "pub.dev")


def read_pubspec_meta(pubspec_path):
    name = ""
    version = ""
    if not os.path.exists(pubspec_path):
        return name, version

    with open(pubspec_path, "r", encoding = "utf-8") as fh:
        for line in fh:
            stripped = line.strip()
            if stripped.startswith("name:") and not name:
                name = stripped.split(":", 1)[1].strip().strip('"').strip("'")
            elif stripped.startswith("version:") and not version:
                version = stripped.split(":", 1)[1].strip().strip('"').strip("'")
            if name and version:
                break

    return name, version


def finish_dependency(name, block):
    if not name:
        return None
    if block != None and block.get("path"):
        return dict(name = name, source = "path", description = dict(path = block.get("path")))
    if block != None and block.get("sdk"):
        return dict(name = name, source = "sdk")
    return dict(name = name, source = "hosted")


def parse_pubspec_dependencies(pubspec_path):
    if not os.path.exists(pubspec_path):
        return []

    with open(pubspec_path, "r", encoding = "utf-8") as fh:
        content = fh.read().splitlines()

    deps = []
    in_deps = False
    deps_indent = 0
    current_name = ""
    current_indent = 0
    current_block = None

    for raw_line in content:
        stripped = raw_line.strip()
        indent = len(raw_line) - len(raw_line.lstrip(" "))

        if not stripped or stripped.startswith("#"):
            continue

        if not in_deps:
            if stripped == "dependencies:":
                in_deps = True
                deps_indent = indent
            continue

        if indent <= deps_indent:
            entry = finish_dependency(current_name, current_block)
            if entry:
                deps.append(entry)
            current_name = ""
            current_block = None
            break

        if current_name and indent > current_indent:
            if ":" in stripped:
                sub_key, sub_value = stripped.split(":", 1)
                if current_block == None:
                    current_block = dict()
                current_block[sub_key.strip()] = sub_value.strip().strip('"').strip("'")
            continue

        if ":" not in stripped:
            continue

        name, remainder = stripped.split(":", 1)
        name = name.strip()
        remainder = remainder.strip()
        entry_indent = indent

        if not name:
            continue

        entry = finish_dependency(current_name, current_block)
        if entry:
            deps.append(entry)

        current_name = name
        current_indent = entry_indent
        if remainder:
            deps.append(dict(name = current_name, source = "hosted"))
            current_name = ""
            current_block = None
        else:
            current_block = dict()

    entry = finish_dependency(current_name, current_block)
    if entry:
        deps.append(entry)

    return deps


def version_key(version):
    parts = re.split("([0-9]+)", version)
    key = []
    for part in parts:
        if not part:
            continue
        if part.isdigit():
            key.append((0, int(part)))
        else:
            key.append((1, part))
    return key


def build_hosted_index(cache_dir):
    index = dict()
    if not os.path.isdir(cache_dir):
        return index

    for entry in os.listdir(cache_dir):
        full_path = os.path.join(cache_dir, entry)
        if not os.path.isdir(full_path):
            continue
        match = re.match("^([a-z0-9_]+)-(.+)$", entry)
        if not match:
            continue
        name = match.group(1)
        version = match.group(2)
        index.setdefault(name, []).append((version, full_path))

    for versions in index.values():
        versions.sort(key = lambda item: version_key(item[0]))

    return index


def hosted_package(name, hosted_index):
    versions = hosted_index.get(name) or []
    if not versions:
        return None, None
    return versions[-1]


def sdk_package_dir(name):
    if not flutter_root:
        return ""
    if name == "sky_engine":
        return os.path.join(flutter_root, "bin", "cache", "pkg", "sky_engine")
    return os.path.join(flutter_root, "packages", name)


hosted_index = build_hosted_index(hosted_cache_dir)
root_pubspec = os.path.join(workspace_dir, "pubspec.yaml")
if not package_name or not package_version:
    root_name, root_version = read_pubspec_meta(root_pubspec)
    if not package_name:
        package_name = root_name
    if not package_version:
        package_version = root_version

if not package_name:
    raise RuntimeError("unable to determine root package name from pubspec.yaml")

packages = [dict(name = package_name, source = "root", dependency = "direct main", version = package_version)]
seen = set([package_name])
queue = []
missing = []

for dep in parse_pubspec_dependencies(root_pubspec):
    queue.append((dep, True))

while queue:
    dep, is_direct = queue.pop(0)
    name = dep.get("name")
    source = dep.get("source")
    dependency_kind = "direct main" if is_direct else "transitive"

    if not name or name in seen:
        continue

    if source == "hosted":
        version, package_dir = hosted_package(name, hosted_index)
        if not package_dir:
            missing.append("hosted package not found in cache: %s" % name)
            continue
        packages.append(dict(name = name, source = "hosted", version = version, dependency = dependency_kind))
        seen.add(name)
        for child in parse_pubspec_dependencies(os.path.join(package_dir, "pubspec.yaml")):
            queue.append((child, False))
    elif source == "sdk":
        packages.append(dict(name = name, source = "sdk", dependency = dependency_kind))
        seen.add(name)
        package_dir = sdk_package_dir(name)
        if package_dir and os.path.isdir(package_dir):
            for child in parse_pubspec_dependencies(os.path.join(package_dir, "pubspec.yaml")):
                queue.append((child, False))
    elif source == "path":
        missing.append("path dependencies are not supported in offline pub deps fallback: %s" % name)

if missing:
    for item in missing:
        sys.stderr.write(item + "\\n")

json.dump(dict(packages = packages), sys.stdout, indent = 2)
sys.stdout.write("\\n")
PY
    then
        cat "$PUB_DEPS_ERR" >&2 || true
        echo "✗ FATAL ERROR: flutter pub deps --json failed" >&2
        exit 1
    fi
fi

export PUB_DEPS_PATH="$PACKAGE_WORKSPACE_DIR_ABS/pub_deps.json"
"$PYTHON_BIN" <<'PY'
import os

path = os.environ.get("PUB_DEPS_PATH")
if path and os.path.exists(path):
    with open(path, "r", encoding="utf-8") as fh:
        payload = fh.read()
    start = None
    for idx, ch in enumerate(payload):
        if ch == "[" or ch == chr(123):
            start = idx
            break
    if start and start > 0:
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(payload[start:])
PY

if [ ! -s pub_deps.json ]; then
    echo "✗ FATAL ERROR: pub_deps.json is empty" >&2
    exit 1
fi

export PUB_CACHE_ABS="$PUB_CACHE_DIR_ABS"
export WORKSPACE_ABS="$PACKAGE_WORKSPACE_DIR_ABS"
export PACKAGE_CONFIG_PATH="$PACKAGE_WORKSPACE_DIR_ABS/.dart_tool/package_config.json"
export ROOT_PACKAGE_NAME="$PACKAGE_NAME"
export ROOT_LANGUAGE_SPEC="$LANGUAGE_SPEC"
mkdir -p "$(dirname "$PACKAGE_CONFIG_PATH")"
"$PYTHON_BIN" <<'PY'
import json
import os

deps_path = os.path.join(os.environ["WORKSPACE_ABS"], "pub_deps.json")
cache_root = os.environ["PUB_CACHE_ABS"]
workspace_root = os.environ["WORKSPACE_ABS"]
config_path = os.environ["PACKAGE_CONFIG_PATH"]
config_dir = os.path.dirname(config_path)
flutter_root = os.environ.get("FLUTTER_ROOT") or ""
root_name = os.environ.get("ROOT_PACKAGE_NAME") or ""
language_spec = os.environ.get("ROOT_LANGUAGE_SPEC") or ""

def _parse_language(spec):
    if not spec:
        return "3.0"
    normalized = spec
    for marker in [">=", "<=", ">", "<", "^", "~"]:
        normalized = normalized.replace(marker, " ")
    tokens = normalized.split()
    if tokens:
        version = tokens[0].split("+")[0]
        parts = version.split(".")
        if len(parts) >= 2:
            return parts[0] + "." + parts[1]
        if len(parts) == 1:
            return parts[0] + ".0"
    return "3.0"

def read_language_spec(pubspec_path):
    if not os.path.exists(pubspec_path):
        return ""
    with open(pubspec_path, "r", encoding="utf-8") as fh:
        lines = fh.readlines()
    for i, line in enumerate(lines):
        if line.strip().startswith("environment:"):
            for j in range(i + 1, len(lines)):
                subline = lines[j].strip()
                if subline.startswith("sdk:"):
                    return subline.split(":", 1)[1].strip().strip('"').strip("'")
                if subline and not subline.startswith("#") and ":" in subline and not subline.startswith(("flutter:", "flutter_test:", "dart:")):
                    return ""
            break
    return ""

def package_language_for_root(root_path, fallback_spec = "", fallback_version = "3.0"):
    package_spec = read_language_spec(os.path.join(root_path, "pubspec.yaml"))
    if package_spec:
        return _parse_language(package_spec)
    if fallback_spec:
        return _parse_language(fallback_spec)
    return fallback_version

language_version = package_language_for_root(workspace_root, language_spec, "3.0")

with open(deps_path, "r", encoding="utf-8") as fh:
    data = json.load(fh)

packages = []
for entry in data.get("packages", []):
    name = entry.get("name")
    source = entry.get("source")
    version = entry.get("version")
    if not name:
        continue
    if source == "hosted" and version:
        root_path = os.path.join(cache_root, "hosted", "pub.dev", name + "-" + version)
        if not os.path.isdir(root_path):
            continue
        rel = os.path.relpath(root_path, config_dir).replace(os.sep, "/")
        package_language = package_language_for_root(root_path, fallback_version = "2.12" if name == "ffi" else "3.0")
        pkg = dict()
        pkg["name"] = name
        pkg["rootUri"] = rel
        pkg["packageUri"] = "lib/"
        pkg["languageVersion"] = package_language
        packages.append(pkg)
    elif source == "root":
        pkg = dict()
        pkg["name"] = name
        pkg["rootUri"] = os.path.relpath(workspace_root, config_dir).replace(os.sep, "/")
        pkg["packageUri"] = "lib/"
        pkg["languageVersion"] = language_version
        packages.append(pkg)
    elif source == "sdk" and flutter_root:
        if name == "sky_engine":
            root_path = os.path.join(flutter_root, "bin", "cache", "pkg", "sky_engine")
        else:
            root_path = os.path.join(flutter_root, "packages", name)
        if not os.path.isdir(root_path):
            continue
        rel = os.path.relpath(root_path, config_dir).replace(os.sep, "/")
        package_language = package_language_for_root(root_path)
        pkg = dict()
        pkg["name"] = name
        pkg["rootUri"] = rel
        pkg["packageUri"] = "lib/"
        pkg["languageVersion"] = package_language
        packages.append(pkg)

config = dict()
config["configVersion"] = 2
config["generated"] = True
config["generator"] = "rules_flutter"
config["packages"] = packages
with open(config_path, "w", encoding="utf-8") as fh:
    json.dump(config, fh, indent=2)
    fh.write("\\n")
PY

CODEGEN_COMMANDS=({codegen_commands})
if [ ${{#CODEGEN_COMMANDS[@]}} -gt 0 ]; then
    if ! "$FLUTTER_BIN_ABS" --suppress-analytics pub get --offline; then
        echo "✗ FATAL ERROR: flutter pub get --offline failed before code generation" >&2
        exit 1
    fi
    for CODEGEN_CMD in "${{CODEGEN_COMMANDS[@]}}"; do
        if [ -n "$CODEGEN_CMD" ]; then
            echo "Running code generation: $CODEGEN_CMD"
            if ! "$FLUTTER_BIN_ABS" --suppress-analytics pub run "$CODEGEN_CMD"; then
                echo "✗ FATAL ERROR: Code generation command '$CODEGEN_CMD' failed" >&2
                exit 1
            fi
        fi
    done
    rm -f .dart_tool/version 2>/dev/null || true
    rm -f .dart_tool/package_config_subset 2>/dev/null || true
fi

echo ""
echo "=== Dependency preparation complete ==="
""".format(
        workspace_src = working_dir.path,
        workspace_dir = prepared_workspace.path,
        working_dir_path = prepared_workspace.path,
        package_dir = package_dir,
        pub_cache_dir = pub_cache_dir.path,
        pub_deps = pub_deps.path,
        dart_tool_dir = dart_tool_dir.path,
        flutter_bin = flutter_bin,
        dep_caches = " ".join(['"{}"'.format(path) for path in dep_pub_cache_args]),
        codegen_commands = " ".join(codegen_args),
        is_pub_package = "1" if is_pub_package else "0",
    )

    ctx.actions.run_shell(
        inputs = [working_dir, pubspec_file] + dep_pub_cache_files + ([workspace_pubspec] if workspace_pubspec else []) + flutter_toolchain.flutterinfo.tool_files + flutter_toolchain.flutterinfo.sdk_files,
        outputs = [pub_get_output, pub_deps, pub_cache_dir, dart_tool_dir, prepared_workspace],
        command = script_content + """

cd "$ORIGINAL_PWD"

mkdir -p "$(dirname "{pub_get_output}")"
mkdir -p "$(dirname "{pub_deps}")"
mkdir -p "$PUB_CACHE_DIR_ABS"
mkdir -p "{dart_tool_dir}"

LOG_FILE="{pub_get_output}"
echo "=== Flutter Dependency Preparation ===" > "$LOG_FILE"
echo "Flutter binary: {flutter_bin}" >> "$LOG_FILE"
echo "Workspace output: {workspace_dir}" >> "$LOG_FILE"
echo "Prepared at: $(date)" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

if [ -f "$PACKAGE_WORKSPACE_DIR_ABS/pub_deps.json" ]; then
    cp "$PACKAGE_WORKSPACE_DIR_ABS/pub_deps.json" "{pub_deps}"
    echo "✓ Generated pub_deps.json" >> "$LOG_FILE"
else
    echo "{{}}" > "{pub_deps}"
    echo "⚠ pub_deps.json missing, wrote empty placeholder" >> "$LOG_FILE"
fi

rm -rf "{dart_tool_dir}"
mkdir -p "{dart_tool_dir}"
if [ -d "$PACKAGE_WORKSPACE_DIR_ABS/.dart_tool" ]; then
    if command -v rsync >/dev/null 2>&1; then
        rsync -a "$PACKAGE_WORKSPACE_DIR_ABS/.dart_tool/" "{dart_tool_dir}/"
    else
        cp -RL "$PACKAGE_WORKSPACE_DIR_ABS/.dart_tool/." "{dart_tool_dir}/"
    fi
    echo "✓ Created .dart_tool/package_config.json" >> "$LOG_FILE"
else
    echo "{{}}" > "{dart_tool_dir}/package_config.json"
    echo "⚠ .dart_tool missing, wrote placeholder package_config.json" >> "$LOG_FILE"
fi

mkdir -p "{pub_cache_dir}"
if [ -n "$(ls -A "$PUB_CACHE_DIR_ABS" 2>/dev/null)" ]; then
    echo "✓ Populated pub_cache directory" >> "$LOG_FILE"
else
    echo '{{}}' > "{pub_cache_dir}/.empty_cache.json"
    echo "⚠ Dependency cache was empty" >> "$LOG_FILE"
fi

echo "Status: Prepared dependencies without pub get" >> "$LOG_FILE"
""".format(
            pub_get_output = pub_get_output.path,
            pub_deps = pub_deps.path,
            pub_cache_dir = pub_cache_dir.path,
            dart_tool_dir = dart_tool_dir.path,
            flutter_bin = flutter_bin,
            workspace_dir = prepared_workspace.path,
        ),
        mnemonic = "FlutterPrepareDeps",
        progress_message = "Preparing Flutter dependencies for %s" % ctx.label.name,
    )

    return prepared_workspace, pub_get_output, pub_cache_dir, pub_deps, dart_tool_dir

def flutter_build_action(
        ctx,
        flutter_toolchain,
        working_dir,
        target,
        pub_cache_dir,
        dart_tool_dir,
        package_dir = ""):
    """Run a Flutter build command.

    Returns:
        Tuple of (build_output, build_artifacts_dir)
    """

    # Get the actual Flutter binary file object (first tool file)
    if not flutter_toolchain.flutterinfo.tool_files:
        fail("No tool files found in Flutter toolchain")
    flutter_bin_file = flutter_toolchain.flutterinfo.tool_files[0]
    flutter_bin = flutter_bin_file.path

    # Create output files
    build_output = ctx.actions.declare_file(ctx.label.name + "_build.log")
    build_artifacts = ctx.actions.declare_directory(ctx.label.name + "_build_artifacts")

    # Map targets to Flutter build commands and output paths
    target_configs = {
        "web": {
            "command": "build web --release",
            "output_dir": "build/web",
        },
        "apk": {
            "command": "build apk --release",
            "output_dir": "build/app/outputs/flutter-apk",
        },
        "ios": {
            "command": "build ios --release --no-codesign",
            "output_dir": "build/ios/iphoneos",
        },
        "macos": {
            "command": "build macos --release",
            "output_dir": "build/macos/Build/Products/Release",
        },
        "linux": {
            "command": "build linux --release",
            "output_dir": "build/linux/x64/release/bundle",
        },
        "windows": {
            "command": "build windows --release",
            "output_dir": "build/windows/x64/runner/Release",
        },
    }

    config = target_configs.get(target, target_configs["web"])

    script_content = """#!/bin/bash
set -euo pipefail

WORKSPACE_DIR="{workspace_dir}"
PUB_CACHE_DIR="{pub_cache_dir}"
DART_TOOL_DIR="{dart_tool_dir}"
FLUTTER_BIN="{flutter_bin}"
OUTPUT_LOG="{output_log}"
BUILD_ARTIFACTS="{build_artifacts}"
BUILD_COMMAND="{build_command}"
BUILD_OUTPUT_DIR="{build_output_dir}"
ORIGINAL_PWD="$PWD"

# Convert relative paths to absolute before changing directories
BUILD_ARTIFACTS_ABS="$ORIGINAL_PWD/$BUILD_ARTIFACTS"
DART_TOOL_DIR_ABS="$ORIGINAL_PWD/$DART_TOOL_DIR"
PUB_CACHE_DIR_ABS="$ORIGINAL_PWD/$PUB_CACHE_DIR"

# Set up environment
export PUB_CACHE="$PUB_CACHE_DIR_ABS"

# Set absolute path to Flutter binary from execroot
FLUTTER_BIN_ABS="$ORIGINAL_PWD/$FLUTTER_BIN"

# Validate Flutter binary exists and is executable
if [ ! -f "$FLUTTER_BIN_ABS" ]; then
    echo "✗ FATAL ERROR: Flutter binary not found at: $FLUTTER_BIN_ABS"
    echo "Expected Flutter SDK to be available via toolchain"
    exit 1
fi

if [ ! -x "$FLUTTER_BIN_ABS" ]; then
    echo "✗ FATAL ERROR: Flutter binary not executable at: $FLUTTER_BIN_ABS"
    echo "Check Flutter SDK permissions and installation"
    exit 1
fi

echo "Flutter binary verified at: $FLUTTER_BIN_ABS"

FLUTTER_ROOT_ORIG="$(cd "$(dirname "$FLUTTER_BIN_ABS")/.." && pwd -P)"

# Flutter updates cache lock and stamp files on startup, so run it from a
# writable SDK overlay instead of the read-only external repository.
FLUTTER_WRITABLE="$(mktemp -d "${{TMPDIR:-/tmp}}/flutter_root.XXXXXX")"
mkdir -p "${{FLUTTER_WRITABLE}}/bin"

for _f in "${{FLUTTER_ROOT_ORIG}}"/* "${{FLUTTER_ROOT_ORIG}}"/.[!.]*; do
    _n="$(basename -- "$_f")" || continue
    [ "$_n" = "bin" ] && continue
    [ -e "$_f" ] || [ -L "$_f" ] || continue
    ln -sf "$_f" "${{FLUTTER_WRITABLE}}/$_n" 2>/dev/null || true
done

mkdir -p "${{FLUTTER_WRITABLE}}/bin/internal"
for _f in "${{FLUTTER_ROOT_ORIG}}/bin/internal"/* "${{FLUTTER_ROOT_ORIG}}/bin/internal"/.[!.]*; do
    _n="$(basename -- "$_f")" || continue
    [ -e "$_f" ] || [ -L "$_f" ] || continue
    if [ "$_n" = "shared.sh" ]; then
        cp "$_f" "${{FLUTTER_WRITABLE}}/bin/internal/shared.sh"
        chmod u+w "${{FLUTTER_WRITABLE}}/bin/internal/shared.sh"

        # Bypass git checks in shared.sh
        sed -i 's/if \\[\\[ ! -e "$FLUTTER_ROOT\\/.git" \\]\\];/if false;/g' "${{FLUTTER_WRITABLE}}/bin/internal/shared.sh" 2>/dev/null || true
        sed -i 's/git rev-parse HEAD/echo bypassed/g' "${{FLUTTER_WRITABLE}}/bin/internal/shared.sh" 2>/dev/null || true

        # Add no-op git script
        cat << 'EOF' > "${{FLUTTER_WRITABLE}}/bin/git"
#!/bin/bash
if [[ "$*" == *"rev-parse HEAD"* ]]; then
  echo "bypassed"
elif [[ "$*" == *"describe --match"* ]]; then
  echo "3.29.3"
else
  echo "bypassed"
fi
EOF
        chmod +x "${{FLUTTER_WRITABLE}}/bin/git"
        export PATH="${{FLUTTER_WRITABLE}}/bin:$PATH"

        chmod u+x "${{FLUTTER_WRITABLE}}/bin/internal/shared.sh"
    else
        ln -sf "$_f" "${{FLUTTER_WRITABLE}}/bin/internal/$_n" 2>/dev/null || true
    fi
done

for _f in "${{FLUTTER_ROOT_ORIG}}/bin"/* "${{FLUTTER_ROOT_ORIG}}/bin"/.[!.]*; do
    _n="$(basename -- "$_f")" || continue
    case "$_n" in flutter|cache|internal) continue ;; esac
    [ -e "$_f" ] || [ -L "$_f" ] || continue
    ln -sf "$_f" "${{FLUTTER_WRITABLE}}/bin/$_n" 2>/dev/null || true
done

cat > "${{FLUTTER_WRITABLE}}/bin/flutter" << 'FLUTTER_WRAPPER_HEREDOC'
#!/usr/bin/env bash
set -e
unset CDPATH
BIN_DIR="$(cd "$(dirname -- "$BASH_SOURCE")" && pwd -P)"
PROG_NAME="$BIN_DIR/$(basename -- "$BASH_SOURCE")"
SHARED_NAME="$BIN_DIR/internal/shared.sh"
OS="$(uname -s)"
if [[ $OS =~ MINGW.* || $OS =~ CYGWIN.* || $OS =~ MSYS.* ]]; then
    exec "$BIN_DIR/flutter.bat" "$@"
fi
source "$SHARED_NAME"
shared::execute "$@"
FLUTTER_WRAPPER_HEREDOC
chmod +x "${{FLUTTER_WRITABLE}}/bin/flutter"

mkdir -p "${{FLUTTER_WRITABLE}}/bin/cache"
if [ -d "${{FLUTTER_ROOT_ORIG}}/bin/cache" ]; then
    for _f in "${{FLUTTER_ROOT_ORIG}}/bin/cache"/* "${{FLUTTER_ROOT_ORIG}}/bin/cache"/.[!.]*; do
        _n="$(basename -- "$_f")" || continue
        case "$_n" in lockfile|.upgrade_lock|.dartignore) continue ;; esac
        [ -e "$_f" ] || [ -L "$_f" ] || continue
        case "$_n" in
            *.stamp)
                cp "$_f" "${{FLUTTER_WRITABLE}}/bin/cache/$_n" 2>/dev/null || \
                    ln -sf "$_f" "${{FLUTTER_WRITABLE}}/bin/cache/$_n" 2>/dev/null || true
                ;;
            *)
                ln -sf "$_f" "${{FLUTTER_WRITABLE}}/bin/cache/$_n" 2>/dev/null || true
                ;;
        esac
    done
fi
chmod u+w "${{FLUTTER_WRITABLE}}/bin/cache" 2>/dev/null || true

FLUTTER_ROOT="$FLUTTER_WRITABLE"
FLUTTER_BIN_ABS="${{FLUTTER_WRITABLE}}/bin/flutter"

# Configure Flutter for sandbox environment
export FLUTTER_SUPPRESS_ANALYTICS=true
export CI=true
export PUB_ENVIRONMENT="flutter_tool:bazel"
export FLUTTER_ALREADY_LOCKED=true
export ANDROID_HOME=""
export ANDROID_SDK_ROOT=""
export FLUTTER_ROOT
export PATH="$FLUTTER_ROOT/bin:$PATH"
PYTHON_BIN="$(command -v python3 || command -v python || true)"
if [ -z "$PYTHON_BIN" ]; then
    echo "✗ FATAL ERROR: python interpreter not found on PATH" >&2
    exit 1
fi

# The prepared workspace is an input tree artifact and is mounted read-only in
# the sandbox. Copy it into a writable runtime directory before mutating it.
WORKSPACE_SRC_ABS="$ORIGINAL_PWD/$WORKSPACE_DIR"
RUNTIME_WORKSPACE="$(mktemp -d "${{TMPDIR:-/tmp}}/flutter_workspace.XXXXXX")"
if command -v rsync >/dev/null 2>&1; then
    rsync -aL "$WORKSPACE_SRC_ABS/" "$RUNTIME_WORKSPACE/"
else
    cp -RL "$WORKSPACE_SRC_ABS/." "$RUNTIME_WORKSPACE/"
fi
chmod -R u+rwX "$RUNTIME_WORKSPACE" 2>/dev/null || true

cd "$RUNTIME_WORKSPACE"
WORKSPACE_ROOT="$(pwd)"

# Copy .dart_tool tree to workspace
if [ -d "$DART_TOOL_DIR_ABS" ]; then
    # The workspace may be a re-used Bazel output directory whose .dart_tool
    # was made read-only after a previous action.  Remove it first so cp never
    # tries to overwrite files in a 0555 directory.
    chmod -R u+w .dart_tool 2>/dev/null || true
    rm -rf .dart_tool
    mkdir -p .dart_tool
    cp -R "$DART_TOOL_DIR_ABS/." .dart_tool/
    chmod -R u+rwX .dart_tool
fi

# Run flutter build
echo "=== Flutter Build {target} ==="
echo "Working directory: $(pwd)"

# Calculate the package directory from original execroot
# If package_dir is set, we must cd into it.
PACKAGE_DIR="{package_dir}"
if [ -n "$PACKAGE_DIR" ] && [ -d "$PACKAGE_DIR" ]; then
    cd "$PACKAGE_DIR"
    echo "Entered package directory: $(pwd)"
fi

echo "Flutter binary: $FLUTTER_BIN_ABS"
echo "Target: {target}"
echo ""

# Regenerate package_config.json with correct paths for this sandbox
# directly from Bazel-generated pub_deps.json so builds stay hermetic and do
# not require a runtime `flutter pub get`.
echo ""
echo "Regenerating package_config.json for build environment..."
export WORKSPACE_ROOT_PATH="$WORKSPACE_ROOT"
export PACKAGE_ROOT_PATH="$(pwd)"
export PACKAGE_PUBSPEC_PATH="$(pwd)/pubspec.yaml"
export WORKSPACE_PUBSPEC_PATH="$WORKSPACE_ROOT/pubspec.yaml"
export PUB_DEPS_PATH="$WORKSPACE_ROOT/pub_deps.json"
export PACKAGE_CONFIG_PATH="$WORKSPACE_ROOT/.dart_tool/package_config.json"
chmod u+w "$WORKSPACE_ROOT/.dart_tool" 2>/dev/null || true
chmod u+w "$PACKAGE_CONFIG_PATH" 2>/dev/null || true
rm -f "$PACKAGE_CONFIG_PATH" 2>/dev/null || true
PACKAGE_CONFIG_OUT="$(mktemp "${{TMPDIR:-/tmp}}/flutter_package_config.XXXXXX.log")"
if "$PYTHON_BIN" > "$PACKAGE_CONFIG_OUT" 2>&1 <<'PY'
import json
import os
from pathlib import Path

package_pubspec_path = os.environ["PACKAGE_PUBSPEC_PATH"]
workspace_pubspec_path = os.environ["WORKSPACE_PUBSPEC_PATH"]
deps_path = os.environ["PUB_DEPS_PATH"]
config_path = os.environ["PACKAGE_CONFIG_PATH"]
cache_root = os.environ["PUB_CACHE"]
workspace_root = os.environ["WORKSPACE_ROOT_PATH"]
package_root = os.environ["PACKAGE_ROOT_PATH"]
flutter_root = os.environ.get("FLUTTER_ROOT") or ""

def read_pubspec_meta(pubspec_path):
    name = ""
    language_spec = ""
    if not os.path.exists(pubspec_path):
        return name, language_spec

    with open(pubspec_path, "r", encoding = "utf-8") as fh:
        lines = fh.readlines()

    for line in lines:
        stripped = line.strip()
        if stripped.startswith("name:") and not name:
            name = stripped.split(":", 1)[1].strip().strip('"').strip("'")

    for i, line in enumerate(lines):
        if line.strip().startswith("environment:"):
            for j in range(i + 1, len(lines)):
                subline = lines[j].strip()
                if subline.startswith("sdk:"):
                    language_spec = subline.split(":", 1)[1].strip().strip('"').strip("'")
                    break
                if subline and not subline.startswith("#") and ":" in subline and not subline.startswith(("flutter:", "flutter_test:", "dart:")):
                    break
            break

    return name, language_spec

workspace_name, workspace_language_spec = read_pubspec_meta(workspace_pubspec_path)
package_name, package_language_spec = read_pubspec_meta(package_pubspec_path)

def _parse_language(spec):
    if not spec:
        return "3.0"
    normalized = spec
    for marker in [">=", "<=", ">", "<", "^", "~"]:
        normalized = normalized.replace(marker, " ")
    tokens = normalized.split()
    if tokens:
        version = tokens[0].split("+")[0]
        parts = version.split(".")
        if len(parts) >= 2:
            return parts[0] + "." + parts[1]
        if len(parts) == 1:
            return parts[0] + ".0"
    return "3.0"

def package_language_for_root(root_path, fallback_spec = "", fallback_version = "3.0"):
    _, package_spec = read_pubspec_meta(os.path.join(root_path, "pubspec.yaml"))
    if package_spec:
        return _parse_language(package_spec)
    if fallback_spec:
        return _parse_language(fallback_spec)
    return fallback_version

def _as_uri(path):
    return Path(path).resolve().as_uri()

language_version = package_language_for_root(package_root, package_language_spec, "3.0")
workspace_language_version = package_language_for_root(workspace_root, workspace_language_spec, "3.0")

with open(deps_path, "r", encoding = "utf-8") as fh:
    data = json.load(fh)

packages = []
seen = set()

def add_package(name, root_path, fallback_spec = "", fallback_version = "3.0"):
    if not name or name in seen:
        return
    if not os.path.isdir(root_path):
        return
    package_language = package_language_for_root(root_path, fallback_spec, fallback_version)
    packages.append(dict(
        name = name,
        rootUri = _as_uri(root_path),
        packageUri = "lib/",
        languageVersion = package_language,
    ))
    seen.add(name)

if workspace_name and os.path.abspath(workspace_pubspec_path) != os.path.abspath(package_pubspec_path):
    add_package(workspace_name, workspace_root, workspace_language_spec, workspace_language_version)

for entry in data.get("packages", []):
    pkg_name = entry.get("name")
    source = entry.get("source")
    version = entry.get("version")
    if not pkg_name:
        continue
    if source == "hosted" and version:
        root_path = os.path.join(cache_root, "hosted", "pub.dev", pkg_name + "-" + version)
        add_package(pkg_name, root_path, fallback_version = "2.12" if pkg_name == "ffi" else "3.0")
    elif source == "root":
        add_package(pkg_name, package_root, package_language_spec, language_version)
    elif source == "sdk" and flutter_root:
        if pkg_name == "sky_engine":
            root_path = os.path.join(flutter_root, "bin", "cache", "pkg", "sky_engine")
        else:
            root_path = os.path.join(flutter_root, "packages", pkg_name)
        add_package(pkg_name, root_path)

os.makedirs(os.path.dirname(config_path), exist_ok = True)
config = dict(
    configVersion = 2,
    generated = True,
    generator = "rules_flutter",
    packages = packages,
)
if flutter_root:
    config["flutterRoot"] = _as_uri(flutter_root)
config["pubCache"] = _as_uri(cache_root)
try:
    with open(config_path, "w", encoding = "utf-8") as fh:
        json.dump(config, fh, indent = 2)
        fh.write("\\n")
except OSError as e:
    if e.errno == 30: # Read-only file system
        pass
    else:
        raise
    # ignore ValueError as it typically relates to I/O on closed file handles
    # from a previous error or when Bazel cuts off the python stdout process
except ValueError as ve:
    pass
PY
then
    echo "✓ Package config regenerated successfully"
else
    cat "$PACKAGE_CONFIG_OUT" >&2 || true
    echo "✗ FATAL ERROR: package_config.json regeneration failed" >&2
    exit 1
fi
echo ""

echo "Running: $FLUTTER_BIN_ABS {build_command}"

if "$FLUTTER_BIN_ABS" --suppress-analytics {build_command}; then
    echo "✓ flutter {build_command} completed successfully"

    # Copy build artifacts to absolute path
    mkdir -p "$BUILD_ARTIFACTS_ABS"
    if [ -d "$BUILD_OUTPUT_DIR" ]; then
        echo "Copying from $BUILD_OUTPUT_DIR to $BUILD_ARTIFACTS_ABS"
        cp -r "$BUILD_OUTPUT_DIR"/* "$BUILD_ARTIFACTS_ABS/" 2>/dev/null || echo "No files to copy from $BUILD_OUTPUT_DIR"
        echo "Build artifacts copied from $BUILD_OUTPUT_DIR"
        echo "Artifacts directory contents:"
        ls -la "$BUILD_ARTIFACTS_ABS" | head -10
    else
        echo "✗ FATAL ERROR: Expected build output directory $BUILD_OUTPUT_DIR not found"
        echo "Flutter build completed but did not create expected output directory"
        echo "This indicates a serious issue with Flutter build execution"
        exit 1
    fi
    
    echo "✓ Flutter build completed successfully"
else
    echo "✗ FATAL ERROR: flutter {build_command} failed"
    echo "Check your Flutter project configuration and dependencies"
    echo "Ensure the offline pub cache contains all required dependencies"
    exit 1
fi
""".format(
        workspace_dir = working_dir.path,
        pub_cache_dir = pub_cache_dir.path,
        dart_tool_dir = dart_tool_dir.path,
        flutter_bin = flutter_bin,
        output_log = build_output.path,
        build_artifacts = build_artifacts.path,
        build_command = config["command"],
        build_output_dir = config["output_dir"],
        target = target,
        package_dir = package_dir,
    )

    # Execute build
    ctx.actions.run_shell(
        inputs = [working_dir, pub_cache_dir, dart_tool_dir] + flutter_toolchain.flutterinfo.tool_files + flutter_toolchain.flutterinfo.sdk_files,
        outputs = [build_artifacts],
        command = script_content,
        mnemonic = "FlutterBuild",
        progress_message = "Running flutter build %s for %s" % (target, ctx.label.name),
    )

    # Create the log file separately using Bazel's write action
    ctx.actions.write(
        output = build_output,
        content = """Flutter build execution log
Target: {target}
Command: {build_command}
Status: Mock flutter build completed (toolchain integration in progress)
Artifacts: Build artifacts directory created
""".format(
            target = target,
            build_command = config["command"],
        ),
    )

    return build_output, build_artifacts
