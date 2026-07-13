#!/usr/bin/env bash
set -euo pipefail

fail=0
if [ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]; then
  cd "$BUILD_WORKSPACE_DIRECTORY"
  script_dir="$BUILD_WORKSPACE_DIRECTORY/.github/scripts"
else
  script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
fi
credential_matches=$(mktemp)
trap 'rm -f -- "$credential_matches"' EXIT

report() {
  printf 'repo hygiene: %s\n' "$*" >&2
  fail=1
}

report_path() {
  local message=$1
  local path=$2
  local rendered
  printf -v rendered '%q' "$path"
  report "$message: $rendered"
}

if ! python3 "$script_dir/check_tracked_bazel_credentials.py" > "$credential_matches"; then
  report 'unable to scan tracked files for Bazel API credential headers'
else
  while IFS= read -r -d '' path; do
    report_path 'tracked Bazel API credential header has a literal value' "$path"
  done < "$credential_matches"
fi

while IFS= read -r -d '' path; do
  case "$path" in
    .ohc_jwt_secret|*/.ohc_jwt_secret)
      report_path 'forbidden runtime secret artifact is tracked' "$path"
      ;;
    .empty_commit_trigger*|*/.empty_commit_trigger*|get_business_context_code|get_business_context_code.rs|*/get_business_context_code|*/get_business_context_code.rs|bazelisk-linux-amd64|*/bazelisk-linux-amd64)
      report_path 'forbidden generated artifact is tracked' "$path"
      ;;
    cleanup_padding.json|replies.json|*.png)
      if [[ "$path" != */* ]]; then
        report_path 'root-level scratch artifact is tracked' "$path"
      fi
      ;;
  esac

  case "$path" in
    *.rs|*.py|*.sh)
      if [[ "$path" != */* ]]; then
        report_path 'root-level source/scratch file is tracked without an owner' "$path"
      fi
      ;;
  esac

  if [ -f "$path" ]; then
    if git diff --numstat --no-index /dev/null "$path" 2>/dev/null | grep -q "^-"; then
      case "$path" in
        *.png|*.jpg|*.jpeg|*.gif|*.ico|*.icns|*.woff|*.woff2|*.ttf)
          # Allowed binary files
          ;;
        *)
          report_path 'forbidden binary file is tracked' "$path"
          ;;
      esac
    fi
  fi
done < <(git ls-files -z)

if [ "$fail" -ne 0 ]; then
  printf 'repo hygiene: move owned source into the source tree and build graph; keep local compiled outputs untracked.\n' >&2
  exit 1
fi

printf 'repo hygiene: ok\n'
