#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
checker="$script_dir/check_repo_hygiene.sh"
repo_root=$(cd -- "$script_dir/../.." && pwd)
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

assert_literal_bazel_header_forbidden() {
  local label=$1
  local path=$2
  local flag=$3
  local assignment_style=$4
  local repo output rendered synthetic_value
  repo=$(new_repo)
  output="$repo/checker.output"
  synthetic_value="unit-test-credential-placeholder"

  mkdir -p -- "$repo/$(dirname -- "$path")"
  case "$assignment_style" in
    equals)
      printf 'common --%s=x-build-service-api-key=%s\n' "$flag" "$synthetic_value" > "$repo/$path"
      ;;
    quoted-spaced)
      printf 'common --%s "x-build-service-api-key = %s"\n' "$flag" "$synthetic_value" > "$repo/$path"
      ;;
    *)
      fail "$label: unknown assignment style"
      ;;
  esac
  git -C "$repo" add -- "$path"

  if (cd -- "$repo" && "$checker") >"$output" 2>&1; then
    fail "$label: checker accepted a tracked literal Bazel credential header"
  fi

  printf -v rendered '%q' "$path"
  grep -Fq -- "repo hygiene: tracked Bazel API credential header has a literal value: $rendered" "$output" ||
    fail "$label: checker did not identify the shell-escaped offending path"
  if grep -Fq -- "$synthetic_value" "$output"; then
    fail "$label: diagnostic exposed the credential value"
  fi
}

assert_bazel_header_references_allowed() {
  local repo output
  repo=$(new_repo)
  output="$repo/checker.output"

  mkdir -p -- "$repo/.github/workflows"
  {
    printf '%s\n' 'common --remote_header=x-build-service-api-key=${{ secrets.BUILD_SERVICE_API_KEY }}'
    printf '%s\n' 'common --bes_header="x-build-service-api-key=${BUILD_SERVICE_API_KEY}"'
    printf '%s\n' 'common --remote_header=x-build-service-api-key=$BUILD_SERVICE_API_KEY'
    printf '%s\n' 'common --remote_header=x-build-service-auth=unit-test-near-miss'
  } > "$repo/.github/workflows/build.yml"
  git -C "$repo" add -- '.github/workflows/build.yml'

  (cd -- "$repo" && "$checker") >"$output" 2>&1 ||
    fail 'protected Bazel credential references or near misses were rejected'
  grep -Fxq -- 'repo hygiene: ok' "$output" ||
    fail 'protected Bazel credential reference repository did not report success'
}

assert_optional_local_bazelrc_contract() {
  local announce_pattern
  announce_pattern='^[[:space:]]*(common|always|build|test|run)(:[^[:space:]]+)?[[:space:]]+--announce_rc([[:space:]]|$|=[[:space:]]*(true|1|yes)([[:space:]]|$))'

  grep -Fxq -- '/.bazelrc.local' "$repo_root/.gitignore" ||
    fail 'the optional local Bazel rc is not narrowly ignored'
  grep -Fxq -- 'try-import %workspace%/.bazelrc.local' "$repo_root/.bazelrc" ||
    fail 'the tracked Bazel rc does not try-import the optional local rc'
  if grep -Eq -- "$announce_pattern" "$repo_root/.bazelrc"; then
    fail 'the tracked Bazel rc enables credential-bearing option announcements'
  fi
}

assert_announce_rc_pattern_contract() {
  local announce_pattern line
  announce_pattern='^[[:space:]]*(common|always|build|test|run)(:[^[:space:]]+)?[[:space:]]+--announce_rc([[:space:]]|$|=[[:space:]]*(true|1|yes)([[:space:]]|$))'

  for line in \
    'build --announce_rc' \
    'build:macos --announce_rc' \
    'common:remote --announce_rc=true' \
    'test:security --announce_rc = yes' \
    'run:local --announce_rc=1'; do
    printf '%s\n' "$line" | grep -Eq -- "$announce_pattern" ||
      fail "announce_rc enablement pattern missed: $line"
  done

  for line in \
    'build --noannounce_rc' \
    'build:macos --announce_rc=false' \
    'query:diagnostic --announce_rc' \
    'build --announce_rc_backup'; do
    if printf '%s\n' "$line" | grep -Eq -- "$announce_pattern"; then
      fail "announce_rc enablement pattern rejected a near miss: $line"
    fi
  done
}

assert_forbidden 'root basename' '.ohc_jwt_secret'
assert_forbidden 'nested basename' 'nested/.ohc_jwt_secret'
assert_forbidden 'space-containing path' 'directory with spaces/.ohc_jwt_secret'
assert_forbidden 'newline-containing directory' $'directory\nwith-control/.ohc_jwt_secret'
assert_near_misses_allowed
assert_literal_bazel_header_forbidden 'remote header equals' '.bazelrc' 'remote_header' 'equals'
assert_literal_bazel_header_forbidden 'BES header quoted and spaced' 'config with spaces/build.bazelrc' 'bes_header' 'quoted-spaced'
assert_literal_bazel_header_forbidden 'newline path diagnostic' $'config\nwith-control/build.bazelrc' 'remote_header' 'equals'
assert_bazel_header_references_allowed
assert_announce_rc_pattern_contract
assert_optional_local_bazelrc_contract

printf 'repo hygiene test: ok\n'
