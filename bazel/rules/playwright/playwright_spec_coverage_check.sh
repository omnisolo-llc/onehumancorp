#!/usr/bin/env bash
set -euo pipefail

mode=""
all_specs=()
ci_specs=()
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
    rel = os.path.relpath(spec, root)
except ValueError:
    rel = spec
if rel.startswith("../"):
    rel = spec
print(rel)
PY
}

display_spec() {
  local spec="$1"
  if [[ -n "${SOURCE_REPO_ROOT:-}" && -d "${SOURCE_REPO_ROOT:-}" && -e "$spec" ]]; then
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
  local spec
  local failed=false
  for spec in "$@"; do
    if [[ ! -f "$spec" ]]; then
      continue
    fi
    if grep -Eq '(^|[^A-Za-z0-9_$])(test|describe)\.(skip|fixme)[[:space:]]*\(' "$spec"; then
      echo "Playwright Bazel coverage check failed: skipped Playwright tests are forbidden: $(display_spec "$spec")"
      failed=true
    fi
    if grep -Eq '(^|[^A-Za-z0-9_$])(test|describe)\.only[[:space:]]*\(' "$spec"; then
      echo "Playwright Bazel coverage check failed: focused Playwright tests are forbidden: $(display_spec "$spec")"
      failed=true
    fi
    if grep -Eq '\.(skip|fixme)[[:space:]]*\(' "$spec"; then
      echo "Playwright Bazel coverage check failed: runtime Playwright skips are forbidden: $(display_spec "$spec")"
      failed=true
    fi
  done
  if [[ "$failed" == true ]]; then
    exit 1
  fi
}

for arg in "$@"; do
  case "$arg" in
    --scan-runfiles)
      scan_runfiles=true
      ;;
    --all | --ci)
      mode="$arg"
      ;;
    *.spec.ts)
      if [[ "$mode" == "--all" ]]; then
        all_specs+=("$arg")
      elif [[ "$mode" == "--ci" ]]; then
        ci_specs+=("$arg")
      else
        echo "Playwright Bazel coverage check failed: spec '$arg' was passed before --all or --ci."
        exit 1
      fi
      ;;
    *)
      if [[ "$arg" == -* ]]; then
        # Ignore arguments that start with a dash, they are for playwright_test.sh
        continue
      else
        echo "Playwright Bazel coverage check failed: unexpected argument '$arg'."
        exit 1
      fi
      ;;
  esac
done

if [[ "$scan_runfiles" == true ]]; then
  runfiles_root="$(runfiles_root)"
  mapfile -t runfile_specs < <(find_spec_files "$runfiles_root")

  if (( ${#runfile_specs[@]} == 0 )) && [[ -n "${TEST_SRCDIR:-}" && "$runfiles_root" != "$TEST_SRCDIR" ]]; then
    mapfile -t runfile_specs < <(find_spec_files "$TEST_SRCDIR")
    runfiles_root="$TEST_SRCDIR"
  fi

  if (( ${#runfile_specs[@]} == 0 )); then
    echo "Playwright Bazel coverage check failed: no runfile specs were discovered."
    exit 1
  fi

  check_forbidden_markers "${runfile_specs[@]}"

  if [[ -n "${SOURCE_REPO_ROOT:-}" && -d "${SOURCE_REPO_ROOT:-}" ]]; then
    source_unique="$(find_spec_relpaths "$SOURCE_REPO_ROOT")"
    runfile_unique="$(find_spec_relpaths "$runfiles_root")"
    missing_specs="$(comm -23 <(printf '%s\n' "$source_unique") <(printf '%s\n' "$runfile_unique"))"
    if [[ -n "$missing_specs" ]]; then
      echo "Playwright Bazel coverage check failed: source specs are missing from Bazel runfiles."
      printf '%s\n' "$missing_specs" | sed 's/^/missing from Bazel runfiles: /'
      exit 1
    fi
  fi

  echo "Bazel aggregate CI coverage discovers ${#runfile_specs[@]} Playwright specs from runfiles."
  exit 0
fi

if (( ${#all_specs[@]} == 0 )); then
  echo "Playwright Bazel coverage check failed: no specs were discovered."
  exit 1
fi

check_forbidden_markers "${all_specs[@]}"

all_unique="$(printf '%s\n' "${all_specs[@]}" | sort -u)"
ci_unique="$(printf '%s\n' "${ci_specs[@]}" | sort -u)"

if [[ -n "$(comm -13 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique"))" ]]; then
  echo "Playwright Bazel coverage check failed: CI aggregate contains specs not discovered by the all-spec glob."
  comm -13 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique") | sed 's/^/not in spec glob: /'
  exit 1
fi

declare -A all_spec_set=()
for spec in "${all_specs[@]}"; do
  all_spec_set["$spec"]=1
done

for spec in "${ci_specs[@]}"; do
  if [[ -z "${all_spec_set[$spec]:-}" ]]; then
    echo "Playwright Bazel coverage check failed: '$spec' was not discovered by the all-spec glob."
    exit 1
  fi
done

missing_specs="$(comm -23 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique"))"
if [[ -n "$missing_specs" ]]; then
  echo "Playwright Bazel coverage check failed: CI aggregate does not include every discovered spec."
  printf '%s\n' "$missing_specs" | sed 's/^/missing from CI aggregate: /'
  exit 1
fi

echo "Bazel aggregate CI coverage includes all ${#all_specs[@]} discovered Playwright specs."
