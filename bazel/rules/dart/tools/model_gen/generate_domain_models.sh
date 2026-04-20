#!/bin/bash
set -euo pipefail

# Hermetic domain model generator.
# Called by the Bazel proto_dart_library rule with:
#   $1 = hermetic dart binary path
#   $2 = input .proto file path
#   $3 = output .domain.dart file path

if [[ $# -ge 3 && -x "$1" && ! -d "$1" ]]; then
  DART="$1"
  shift
else
  DART=""
fi

# Locate the hermetic Dart binary supplied by @flutter_sdk//:dart (via runfiles).
# Bazel places runfile symlinks under $RUNFILES_DIR or alongside the script.
if [[ -n "${RUNFILES_DIR:-}" ]]; then
  DART_SEARCH_ROOT="$RUNFILES_DIR"
elif [[ -n "${RUNFILES_MANIFEST_FILE:-}" ]]; then
  DART_SEARCH_ROOT="$(dirname "$RUNFILES_MANIFEST_FILE")"
else
  DART_SEARCH_ROOT="$(cd "$(dirname "$0")" && pwd)"
fi

if [[ -z "$DART" ]]; then
  # Try common runfile paths for the flutter_sdk dart binary.
  for candidate in \
      "$DART_SEARCH_ROOT/rules_flutter++flutter+flutter_sdk/bin/dart" \
      "$DART_SEARCH_ROOT/rules_flutter++flutter+flutter_sdk/flutter/bin/cache/dart-sdk/bin/dart" \
      "$DART_SEARCH_ROOT/_main/external/rules_flutter++flutter+flutter_sdk/bin/dart" \
      "$DART_SEARCH_ROOT/_main/external/rules_flutter++flutter+flutter_sdk/flutter/bin/cache/dart-sdk/bin/dart" \
      "$DART_SEARCH_ROOT/flutter_sdk/bin/dart" \
      "$DART_SEARCH_ROOT/flutter_sdk/flutter/bin/cache/dart-sdk/bin/dart" \
      "$(find "$DART_SEARCH_ROOT" \( -name dart -o -name dart.exe \) ! -type d 2>/dev/null | grep -v '\.dart$' | head -n 1)"; do
    if [[ -x "$candidate" && ! -d "$candidate" ]]; then
      DART="$candidate"
      break
    fi
  done
fi

# Fall back to PATH dart if hermetic binary not found (should not happen in Bazel)
if [[ -z "$DART" ]]; then
  DART="$(command -v dart 2>/dev/null || true)"
fi

if [[ -z "$DART" ]]; then
  echo "ERROR: Could not locate dart binary. Hermetic build required." >&2
  exit 1
fi

exec "$DART" "$(dirname "$0")/generate_domain_models.dart" "$@"
