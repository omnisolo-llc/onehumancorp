#!/bin/bash
set -euo pipefail

discovery="$1"
work_dir="$(mktemp -d)"
trap 'rm -rf "$work_dir"' EXIT

mkdir -p "$work_dir/source" "$work_dir/runfiles/src/ui/next/e2e"
printf 'test("symlinked spec", () => {});\n' > "$work_dir/source/symlinked.spec.ts"
ln -s "$work_dir/source/symlinked.spec.ts" "$work_dir/runfiles/src/ui/next/e2e/symlinked.spec.ts"

mapfile -d '' specs < <("$discovery" "$work_dir/runfiles")
[[ ${#specs[@]} -eq 1 ]]
[[ "$(realpath "${specs[0]}")" == "$work_dir/source/symlinked.spec.ts" ]]
