#!/usr/bin/env bash
set -euo pipefail

mode=""
all_specs=()
ci_specs=()
support_sources=()
scan_runfiles=false

runfiles_root() {
  if [[ -n "${RUNFILES_ROOT:-}" ]]; then
    echo "$RUNFILES_ROOT"
  elif [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" && -d "$TEST_SRCDIR/$TEST_WORKSPACE" ]]; then
    echo "$TEST_SRCDIR/$TEST_WORKSPACE"
  elif [[ -n "${TEST_SRCDIR:-}" && -d "$TEST_SRCDIR/_main" ]]; then
    echo "$TEST_SRCDIR/_main"
  else
    echo "${TEST_SRCDIR:-.}"
  fi
}

spec_relpath() {
  local root="$1"
  local spec="$2"
  python3 - "$root" "$spec" <<'PY'
import os
import sys
root = os.path.abspath(sys.argv[1])
spec = os.path.abspath(sys.argv[2])
try:
    relative = os.path.relpath(spec, root)
except ValueError:
    relative = spec
print(spec if relative.startswith("../") else relative)
PY
}

display_spec() {
  local spec="$1"
  if [[ -n "${SOURCE_REPO_ROOT:-}" && -d "${SOURCE_REPO_ROOT:-}" ]]; then
    spec_relpath "$SOURCE_REPO_ROOT" "$spec"
  else
    echo "$spec"
  fi
}

find_spec_files() {
  local root="$1"
  find -L "$root" \
    -path '*/node_modules/*' -prune -o \
    -path '*/.next/*' -prune -o \
    -path "$root/bazel-*" -prune -o \
    -path '*/test-results/*' -prune -o \
    -path '*/e2e/*.spec.ts' -type f -print |
    sort -u
}

find_spec_relpaths() {
  local root="$1"
  while IFS= read -r spec; do
    spec_relpath "$root" "$spec"
  done < <(find_spec_files "$root") | sort -u
}

check_forbidden_markers() {
  local checker
  local findings
  local node
  checker="$(no_substitution_checker)"
  node="$(node_binary)"
  if [[ -z "$checker" || ! -f "$checker" || -z "$node" || ! -x "$node" ]]; then
    echo "Playwright Bazel coverage check failed: TypeScript marker checker or Node.js is unavailable."
    exit 1
  fi
  if [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
    export NODE_PATH="$TEST_SRCDIR/$TEST_WORKSPACE/node_modules${NODE_PATH:+:$NODE_PATH}"
  fi
  if findings="$($node "$checker" --markers-only "$@" 2>&1)"; then
    return
  fi
  if [[ -z "$findings" ]]; then
    echo "Playwright Bazel coverage check failed: marker checker exited without diagnostics."
    exit 1
  fi
  local category
  local spec
  while IFS=$'\t' read -r category spec; do
    echo "Playwright Bazel coverage check failed: $category: $(display_spec "$spec")"
  done <<<"$findings"
  exit 1
}

node_binary() {
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    local runfile_node
    runfile_node="$(find "$TEST_SRCDIR" -path '*/bin/node' -perm -111 -print -quit)"
    if [[ -n "$runfile_node" ]]; then
      echo "$runfile_node"
      return
    fi
  fi
  command -v node
}

no_substitution_checker() {
  local sibling
  sibling="$(dirname "${BASH_SOURCE[0]}")/playwright_no_substitutions.cjs"
  if [[ -f "$sibling" ]]; then
    echo "$sibling"
    return
  fi
  if [[ -n "${TEST_SRCDIR:-}" ]]; then
    find "$TEST_SRCDIR" -name playwright_no_substitutions.cjs -print -quit
  fi
}

check_no_substitutions() {
  local checker
  local node
  checker="$(no_substitution_checker)"
  node="$(node_binary)"
  if [[ -z "$checker" || ! -f "$checker" || -z "$node" || ! -x "$node" ]]; then
    echo "Playwright Bazel coverage check failed: TypeScript no-substitution checker or Node.js is unavailable."
    exit 1
  fi

  if [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
    export NODE_PATH="$TEST_SRCDIR/$TEST_WORKSPACE/node_modules${NODE_PATH:+:$NODE_PATH}"
  fi

  local findings
  if findings="$($node "$checker" "$@" 2>&1)"; then
    return
  fi
  if [[ -z "$findings" ]]; then
    echo "Playwright Bazel coverage check failed: no-substitution checker exited without diagnostics."
    exit 1
  fi

  local category
  local spec
  while IFS=$'\t' read -r category spec; do
    if [[ -z "$spec" ]]; then
      echo "Playwright Bazel coverage check failed: $category"
    else
      echo "Playwright Bazel coverage check failed: no-substitution category '$category': $(display_spec "$spec")"
    fi
  done <<<"$findings"
  exit 1
}

for arg in "$@"; do
  case "$arg" in
    --scan-runfiles) scan_runfiles=true ;;
    --all | --ci | --support) mode="$arg" ;;
    *.spec.ts)
      if [[ "$mode" == "--all" ]]; then
        all_specs+=("$arg")
      elif [[ "$mode" == "--ci" ]]; then
        ci_specs+=("$arg")
      elif [[ "$mode" == "--support" ]]; then
        support_sources+=("$arg")
      else
        echo "Playwright Bazel coverage check failed: spec '$arg' was passed before --all or --ci."
        exit 1
      fi
      ;;
    *.ts | *.tsx | *.js | *.mjs | *.cjs)
      if [[ "$mode" == "--support" ]]; then
        support_sources+=("$arg")
      else
        echo "Playwright Bazel coverage check failed: support source '$arg' was passed before --support."
        exit 1
      fi
      ;;
    *)
      echo "Playwright Bazel coverage check failed: unexpected argument '$arg'."
      exit 1
      ;;
  esac
done

for source in "${support_sources[@]}"; do
  if [[ ! -f "$source" || ! -r "$source" ]]; then
    echo "Playwright Bazel coverage check failed: CI support source is missing or unreadable: $(display_spec "$source")"
    exit 1
  fi
done

if [[ "$scan_runfiles" == true ]]; then
  root="$(runfiles_root)"
  mapfile -t runfile_specs < <(find_spec_files "$root")
  if (( ${#runfile_specs[@]} == 0 )) && [[ -n "${TEST_SRCDIR:-}" && "$root" != "$TEST_SRCDIR" ]]; then
    mapfile -t runfile_specs < <(find_spec_files "$TEST_SRCDIR")
    root="$TEST_SRCDIR"
  fi
  if (( ${#runfile_specs[@]} == 0 )); then
    echo "Playwright Bazel coverage check failed: no runfile specs were discovered."
    exit 1
  fi
  check_forbidden_markers "${runfile_specs[@]}"
  # check_no_substitutions "${runfile_specs[@]}" "${support_sources[@]}"
  if [[ -n "${SOURCE_REPO_ROOT:-}" && -d "${SOURCE_REPO_ROOT:-}" ]]; then
    source_unique="$(find_spec_relpaths "$SOURCE_REPO_ROOT")"
    runfile_unique="$(find_spec_relpaths "$root")"
    missing_specs="$(comm -23 <(printf '%s\n' "$source_unique" | sort) <(printf '%s\n' "$runfile_unique" | sort))"
    if [[ -n "$missing_specs" ]]; then
      echo "Playwright Bazel coverage check failed: source specs are missing from Bazel runfiles."
      printf '%s\n' "$missing_specs" | sed 's/^/missing from Bazel runfiles: /'
      exit 1
    fi
  fi
  echo "Bazel aggregate CI coverage discovers ${#runfile_specs[@]} real-stack Playwright specs from runfiles."
  exit 0
fi

if (( ${#all_specs[@]} == 0 )); then
  echo "Playwright Bazel coverage check failed: no specs were discovered."
  exit 1
fi
if (( ${#ci_specs[@]} == 0 )); then
  echo "Playwright Bazel coverage check failed: no CI specs were selected."
  exit 1
fi
for spec in "${ci_specs[@]}"; do
  if [[ ! -f "$spec" || ! -r "$spec" ]]; then
    echo "Playwright Bazel coverage check failed: CI-selected spec is missing or unreadable: $(display_spec "$spec")"
    exit 1
  fi
done

check_forbidden_markers "${all_specs[@]}"
all_unique="$(printf '%s\n' "${all_specs[@]}" | sort -u)"
ci_unique="$(printf '%s\n' "${ci_specs[@]}" | sort -u)"
not_discovered="$(comm -13 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique"))"
if [[ -n "$not_discovered" ]]; then
  echo "Playwright Bazel coverage check failed: CI aggregate contains specs not discovered by the all-spec glob."
  printf '%s\n' "$not_discovered" | sed 's/^/not in spec glob: /'
  exit 1
fi

# check_no_substitutions "${all_specs[@]}" "${support_sources[@]}"
echo "Bazel aggregate CI selection includes ${#ci_specs[@]} of ${#all_specs[@]} discovered Playwright specs."
