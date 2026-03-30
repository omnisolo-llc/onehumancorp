import sys

file_path = "bazel/rules/flutter/flutter/private/flutter_actions.bzl"
with open(file_path, "r") as f:
    content = f.read()

target = """    if [ -f "$PUB_DEPS_ERR" ] && grep -qi "requires the Flutter SDK" "$PUB_DEPS_ERR"; then
        if ! "$FLUTTER_BIN_ABS" --suppress-analytics pub deps --json > pub_deps.json 2>> "$PUB_DEPS_ERR"; then
            echo '{"packages": []}' > pub_deps.json
        fi
    else
        echo '{"packages": []}' > pub_deps.json
    fi
fi

if ! [ -s pub_deps.json ]; then
    echo '{"packages": []}' > pub_deps.json
fi"""

replacement = """    if [ -f "$PUB_DEPS_ERR" ] && grep -qi "requires the Flutter SDK" "$PUB_DEPS_ERR"; then
        if ! "$FLUTTER_BIN_ABS" --suppress-analytics pub deps --json > pub_deps.json 2>> "$PUB_DEPS_ERR"; then
            echo '{"packages": []}' > pub_deps.json
        fi
    else
        echo '{"packages": []}' > pub_deps.json
    fi
fi

if ! [ -s pub_deps.json ]; then
    echo '{"packages": []}' > pub_deps.json
fi"""

target2 = """DART_BIN_LOCAL="$FLUTTER_ROOT/bin/cache/dart-sdk/bin/dart"
PUB_DEPS_ERR="$WORKSPACE_DIR_ABS/pub_deps.stderr.log"
if [ -x "$DART_BIN_LOCAL" ] && "$DART_BIN_LOCAL" pub deps --json > pub_deps.json 2> "$PUB_DEPS_ERR"; then
    :
else
    if [ -f "$PUB_DEPS_ERR" ] && grep -qi "requires the Flutter SDK" "$PUB_DEPS_ERR"; then
        if ! "$FLUTTER_BIN_ABS" --suppress-analytics pub deps --json > pub_deps.json 2>> "$PUB_DEPS_ERR"; then
            echo '{"packages": []}' > pub_deps.json
        fi
    else
        echo '{"packages": []}' > pub_deps.json
    fi
fi"""

replacement2 = """DART_BIN_LOCAL="$FLUTTER_ROOT/bin/cache/dart-sdk/bin/dart"
PUB_DEPS_ERR="$WORKSPACE_DIR_ABS/pub_deps.stderr.log"
"$DART_BIN_LOCAL" pub deps --json > pub_deps.json 2> "$PUB_DEPS_ERR" || true
if ! grep -q '"packages"' pub_deps.json; then
    "$FLUTTER_BIN_ABS" --suppress-analytics pub deps --json > pub_deps.json 2>> "$PUB_DEPS_ERR" || true
fi
if ! grep -q '"packages"' pub_deps.json; then
    echo '{"packages": []}' > pub_deps.json
fi"""

if target2 in content:
    with open(file_path, "w") as f:
        f.write(content.replace(target2, replacement2))
else:
    print("Target not found")
