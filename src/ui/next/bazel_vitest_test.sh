#!/usr/bin/env bash
set -euo pipefail

if [[ -z "${TEST_SRCDIR:-}" || -z "${TEST_WORKSPACE:-}" || -z "${TEST_TMPDIR:-}" ]]; then
  echo "next_vitest must run under Bazel with TEST_SRCDIR, TEST_WORKSPACE, and TEST_TMPDIR set" >&2
  exit 1
fi

runfiles_root="${TEST_SRCDIR}/${TEST_WORKSPACE}"
next_dir="${runfiles_root}/src/ui/next"
node_modules="${next_dir}/node_modules"

if [[ ! -d "$next_dir" ]]; then
  echo "missing Next app runfiles at $next_dir" >&2
  exit 1
fi

if [[ ! -f "$node_modules/vitest/vitest.mjs" ]]; then
  echo "missing Bazel-provided Vitest dependency at $node_modules/vitest/vitest.mjs" >&2
  exit 1
fi

node_bin="$(find "$TEST_SRCDIR" -path '*/bin/node' | head -n 1)"
if [[ -z "$node_bin" || ! -x "$node_bin" ]]; then
  echo "missing Bazel-provided Node.js binary" >&2
  exit 1
fi

cd "$next_dir"

work_dir="$TEST_TMPDIR/work"
rm -rf "$work_dir"
mkdir -p "$work_dir"

cp -RL src "$work_dir/src"

for file in next-env.d.ts package.json package-lock.json tsconfig.json vitest.setup.ts vitest.config.ts; do
  if [[ -f "$file" ]]; then
    cp -L "$file" "$work_dir/$file"
  else
    echo "missing $file" >&2
    exit 1
  fi
done
ln -s "$node_modules" "$work_dir/node_modules"

cd "$work_dir"

export NODE_PATH="$PWD/node_modules"
export NODE_OPTIONS="--experimental-vm-modules"

"$node_bin" node_modules/vitest/vitest.mjs run --passWithNoTests
