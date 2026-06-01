#!/usr/bin/env bash
set -euo pipefail

find_next_dir() {
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    local candidate
    candidate="$(find "${TEST_SRCDIR}" -path '*/src/ui/next/package.json' -print -quit 2>/dev/null || true)"
    if [[ -n "${candidate}" ]]; then
      dirname "${candidate}"
      return 0
    fi
  fi

  if [[ -n "${BUILD_WORKSPACE_DIRECTORY:-}" && -f "${BUILD_WORKSPACE_DIRECTORY}/src/ui/next/package.json" ]]; then
    printf '%s\n' "${BUILD_WORKSPACE_DIRECTORY}/src/ui/next"
    return 0
  fi

  local dir="${PWD}"
  while [[ "${dir}" != "/" ]]; do
    if [[ -f "${dir}/src/ui/next/package.json" ]]; then
      printf '%s\n' "${dir}/src/ui/next"
      return 0
    fi
    dir="$(dirname "${dir}")"
  done

  return 1
}

next_dir="$(find_next_dir)"
work_dir="${TEST_TMPDIR:-/tmp}/next-vite-test"
rm -rf "${work_dir}"
mkdir -p "${work_dir}"

cp -L "${next_dir}/package.json" "${work_dir}/"
cp -L "${next_dir}/package-lock.json" "${work_dir}/"
cp -L "${next_dir}/tsconfig.json" "${work_dir}/"
cp -L "${next_dir}/next-env.d.ts" "${work_dir}/"
cp -L "${next_dir}/vitest.config.ts" "${work_dir}/"
cp -L "${next_dir}/vitest.setup.ts" "${work_dir}/"
cp -RL "${next_dir}/src" "${work_dir}/src"

cd "${work_dir}"

if [[ ! -x "node_modules/.bin/vitest" ]]; then
  npm install
fi

npx vitest run \
  src/app/api/chat/route.test.ts \
  src/app/api/v1/growth/storefront/embed/route.test.ts \
  src/app/changelog/page.test.tsx \
  src/app/api-docs/page.test.tsx
