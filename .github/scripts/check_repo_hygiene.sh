#!/usr/bin/env bash
set -euo pipefail

if [ -n "${BUILD_WORKSPACE_DIRECTORY:-}" ]; then
  cd "$BUILD_WORKSPACE_DIRECTORY"
fi

fail=0

report() {
  printf 'repo hygiene: %s\n' "$*" >&2
  fail=1
}

while IFS= read -r -d '' path; do
  case "$path" in
    .empty_commit_trigger*|*/.empty_commit_trigger*|get_business_context_code|get_business_context_code.rs|*/get_business_context_code|*/get_business_context_code.rs|bazelisk-linux-amd64|*/bazelisk-linux-amd64)
      report "forbidden generated artifact is tracked: $path"
      ;;
    cleanup_padding.json|replies.json|*.png)
      if [[ "$path" != */* ]]; then
        report "root-level scratch artifact is tracked: $path"
      fi
      ;;
  esac

  case "$path" in
    *.rs|*.py|*.sh)
      if [[ "$path" != */* ]]; then
        report "root-level source/scratch file is tracked without an owner: $path"
      fi
      ;;
  esac

  if [ -f "$path" ] && command -v file >/dev/null 2>&1; then
    mime="$(file -b --mime-type -- "$path")"
    mode="$(git ls-files -s -- "$path" | awk '{print $1}')"
    case "$mime" in
      application/x-executable|application/x-pie-executable|application/x-mach-binary|application/x-dosexec|application/vnd.microsoft.portable-executable)
        report "compiled executable binary is tracked: $path ($mime)"
        ;;
      application/octet-stream)
        if [ "$mode" = "100755" ]; then
          report "executable binary-like artifact is tracked: $path ($mime)"
        fi
        ;;
    esac
  fi
done < <(git ls-files -z)

if [ "$fail" -ne 0 ]; then
  printf 'repo hygiene: move owned source into the source tree and build graph; keep local compiled outputs untracked.\n' >&2
  exit 1
fi

printf 'repo hygiene: ok\n'
