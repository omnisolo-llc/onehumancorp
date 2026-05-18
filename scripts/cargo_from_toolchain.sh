#!/usr/bin/env bash
set -euo pipefail

cargo_runfile="$1"
rustc_runfile="$2"
shift 2

resolve_runfile() {
  local runfile="$1"
  local root

  if [[ "${runfile}" = /* && -e "${runfile}" ]]; then
    printf '%s\n' "${runfile}"
    return 0
  fi

  for root in "${RUNFILES_DIR:-}" "${0}.runfiles" "$(dirname "$0").runfiles"; do
    [[ -n "${root}" ]] || continue
    if [[ -e "${root}/${runfile}" ]]; then
      printf '%s\n' "${root}/${runfile}"
      return 0
    fi
    if [[ -e "${root}/_main/${runfile}" ]]; then
      printf '%s\n' "${root}/_main/${runfile}"
      return 0
    fi
  done

  if [[ -e "${runfile}" ]]; then
    printf '%s\n' "${runfile}"
    return 0
  fi

  return 1
}

cargo="$(resolve_runfile "${cargo_runfile}")"
rustc="$(resolve_runfile "${rustc_runfile}")"
toolchain_bin="$(dirname "${cargo}")"

export PATH="${toolchain_bin}:${PATH}"
export RUSTC="${rustc}"

if [[ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]]; then
  cd "${BUILD_WORKSPACE_DIRECTORY}"
fi

exec "${cargo}" "$@"
