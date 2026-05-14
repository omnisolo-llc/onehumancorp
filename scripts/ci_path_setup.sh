#!/bin/bash
# This script is called during postinstall in CI to ensure that
# tools are available and to shim bazelisk to fix CI issues.

if [ -n "$GITHUB_PATH" ]; then
  BIN_DIR="$(pwd)/.ci_bin"
  mkdir -p "$BIN_DIR"

  cat <<'EOF' > "$BIN_DIR/bazelisk"
#!/bin/bash
# OHC CI Bazelisk Shim
cmd=$1
shift
args=()
has_dash_dash=false

case "$cmd" in
  build|test|run|query|cquery|aquery)
    args+=("$cmd")
    # Add resource constraints for the runner
    args+=("--local_resources=ram=1024")
    ;;
  *)
    args+=("$cmd")
    ;;
esac

for arg in "$@"; do
  if [[ "$arg" == "--" ]]; then
    has_dash_dash=true
    args+=("$arg")
    continue
  fi
  # Fix for: Negative target patterns can only appear after the end of options marker ('--')
  if [[ "$arg" == -//* ]] && [ "$has_dash_dash" = false ]; then
    args+=("--")
    has_dash_dash=true
  fi
  args+=("$arg")
done

# Try to find bazel in PATH
if command -v bazel >/dev/null 2>&1; then
  exec bazel "${args[@]}"
else
  # If bazel is not found, it might be that setup-bazel hasn't run yet or failed
  echo "Error: bazel not found in PATH" >&2
  exit 127
fi
EOF

  chmod +x "$BIN_DIR/bazelisk"
  echo "$BIN_DIR" >> "$GITHUB_PATH"
  echo "Added $BIN_DIR to GITHUB_PATH with bazelisk shim"
fi
