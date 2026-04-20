#!/bin/bash
set -euo pipefail

# Hermetic domain model generator.
# Called by the Bazel proto_dart_library rule with:
#   $1 = input .proto file path
#   $2 = output .domain.dart file path

# Locate the hermetic Dart binary supplied by @flutter_sdk//:dart (via runfiles).
# Bazel places runfile symlinks under $RUNFILES_DIR or alongside the script.
if [[ -n "${RUNFILES_DIR:-}" ]]; then
  DART_SEARCH_ROOT="$RUNFILES_DIR"
elif [[ -n "${RUNFILES_MANIFEST_FILE:-}" ]]; then
  DART_SEARCH_ROOT="$(dirname "$RUNFILES_MANIFEST_FILE")"
else
  DART_SEARCH_ROOT="$(cd "$(dirname "$0")" && pwd)"
fi

# Try common runfile paths for the flutter_sdk dart binary
DART=""
for candidate in \
    "$DART_SEARCH_ROOT/rules_flutter/flutter/dart" \
    "$DART_SEARCH_ROOT/../flutter_sdk/dart" \
    "$(find "$DART_SEARCH_ROOT" -name dart -type f 2>/dev/null | grep -v '.dart' | head -n 1)"; do
  if [[ -x "$candidate" ]]; then
    DART="$candidate"
    break
  fi
done

# Fall back to PATH dart if hermetic binary not found (should not happen in Bazel)
if [[ -z "$DART" ]]; then
  DART="$(command -v dart 2>/dev/null || true)"
fi

if [[ -z "$DART" ]]; then
  echo "ERROR: Could not locate dart binary. Hermetic build required." >&2
  exit 1
fi

exec "$DART" "$(dirname "$0")/generate_domain_models.dart" "$@"
