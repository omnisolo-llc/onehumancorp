#!/bin/bash
# OHC CI Bazelisk Shim Setup
# This script ensures bazelisk is on the PATH and shims it to fix CI issues.

set -e

if [ -n "$GITHUB_PATH" ]; then
  BIN_DIR="$(pwd)/.ci_bin"
  mkdir -p "$BIN_DIR"

  echo "Creating bazelisk shim at $BIN_DIR/bazelisk" >&2

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
    # Add resource constraints to prevent runner communication loss (OOM/CPU)
    args+=("--local_resources=ram=1024")
    args+=("--local_cpu_resources=2")
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
  # This fixes the "//... -//src/e2e/..." issue in CI.
  if [[ "$arg" == -//* ]] && [ "$has_dash_dash" = false ]; then
    args+=("--")
    has_dash_dash=true
  fi
  args+=("$arg")
done

# Try to find bazel in PATH (provided by setup-bazel)
# If not found, try to use the one installed by npm if available
BAZEL_BIN=$(command -v bazel || echo "$(pwd)/node_modules/.bin/bazel")

if [ -x "$BAZEL_BIN" ] || command -v bazel >/dev/null 2>&1; then
  exec "$BAZEL_BIN" "${args[@]}"
else
  # Fallback to just 'bazel' and hope for the best
  exec bazel "${args[@]}"
fi
EOF

  chmod +x "$BIN_DIR/bazelisk"
  echo "$BIN_DIR" >> "$GITHUB_PATH"
  echo "Added $BIN_DIR to GITHUB_PATH" >&2
fi
