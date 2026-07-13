#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
checker="$script_dir/check_repo_hygiene.sh"
test_root=$(mktemp -d)
trap 'rm -rf -- "$test_root"' EXIT

fail() {
  printf 'repo hygiene test: %s\n' "$*" >&2
  exit 1
}

new_repo() {
  local repo
  repo=$(mktemp -d "$test_root/repo.XXXXXX")
  git -C "$repo" init -q
  printf '%s\n' "$repo"
}

assert_forbidden() {
  local label=$1
  local path=$2
  local repo output rendered line
  repo=$(new_repo)
  output="$repo/checker.output"

  mkdir -p -- "$repo/$(dirname -- "$path")"
  : > "$repo/$path"
  git -C "$repo" add -f -- "$path"

  if (cd -- "$repo" && "$checker") >"$output" 2>&1; then
    fail "$label: checker accepted forbidden basename"
  fi

  printf -v rendered '%q' "$path"
  grep -Fq -- "repo hygiene: forbidden runtime secret artifact is tracked: $rendered" "$output" ||
    fail "$label: checker did not identify the shell-escaped forbidden path"

  while IFS= read -r line; do
    [[ "$line" == 'repo hygiene: '* ]] ||
      fail "$label: diagnostic contains an unescaped line break"
  done < "$output"
}

assert_near_misses_allowed() {
  local repo output path
  repo=$(new_repo)
  output="$repo/checker.output"

  for path in \
    '.ohc_jwt_secret.backup' \
    'prefix.ohc_jwt_secret' \
    'nested/.ohc_jwt_secret.suffix'; do
    mkdir -p -- "$repo/$(dirname -- "$path")"
    : > "$repo/$path"
    git -C "$repo" add -f -- "$path"
  done

  (cd -- "$repo" && "$checker") >"$output" 2>&1 ||
    fail 'near-miss basenames were rejected'
  grep -Fxq -- 'repo hygiene: ok' "$output" ||
    fail 'near-miss repository did not report success'
}

assert_forbidden 'root basename' '.ohc_jwt_secret'
assert_forbidden 'nested basename' 'nested/.ohc_jwt_secret'
assert_forbidden 'space-containing path' 'directory with spaces/.ohc_jwt_secret'
assert_forbidden 'newline-containing directory' $'directory\nwith-control/.ohc_jwt_secret'
assert_near_misses_allowed

printf 'repo hygiene test: ok\n'
