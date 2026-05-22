#!/usr/bin/env bash
set -euo pipefail

cd "${BUILD_WORKSPACE_DIRECTORY:-$(pwd)}"

targets=("$@")
if [[ ${#targets[@]} -eq 0 ]]; then
  targets=(//...)
fi

exec npx bazelisk test \
  --@rules_rust//rust/settings:extra_rustc_flag=-Dwarnings \
  "${targets[@]}"
