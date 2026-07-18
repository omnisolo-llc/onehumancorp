#!/bin/bash
set -euo pipefail

if [[ $# -ne 1 || ! -d "$1" ]]; then
  echo "usage: $0 RUNFILES_WORKSPACE_ROOT" >&2
  exit 2
fi

workspace_root="$1"
discovery_roots=()
for relative_root in src/e2e src/ui/next/e2e src/ui/next/src/e2e; do
  candidate="$workspace_root/$relative_root"
  if [[ -d "$candidate" ]]; then
    discovery_roots+=("$candidate")
  fi
done

if (( ${#discovery_roots[@]} == 0 )); then
  exit 0
fi

# Bazel runfiles expose source files as symlinks. Follow them while keeping the
# search confined to the known E2E roots, then sort deterministically.
find -L "${discovery_roots[@]}" -name '*.spec.ts' -type f -print0 | sort -z
