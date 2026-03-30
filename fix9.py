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

if target in content:
    with open(file_path, "w") as f:
        f.write(content.replace(target, replacement))
else:
    print("Target not found")
