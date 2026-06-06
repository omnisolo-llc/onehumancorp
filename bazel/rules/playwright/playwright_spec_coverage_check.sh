#!/usr/bin/env bash
set -euo pipefail

mode=""
all_specs=()
ci_specs=()
scan_runfiles=false

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
      echo "Playwright Bazel coverage check failed: unexpected argument '$arg'."
      exit 1
      ;;
  esac
done

if [[ "$scan_runfiles" == true ]]; then
  runfiles_root="${RUNFILES_ROOT:-}"
  if [[ -z "$runfiles_root" ]]; then
    if [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" && -d "$TEST_SRCDIR/$TEST_WORKSPACE" ]]; then
      runfiles_root="$TEST_SRCDIR/$TEST_WORKSPACE"
    elif [[ -n "${TEST_SRCDIR:-}" && -d "$TEST_SRCDIR/_main" ]]; then
      runfiles_root="$TEST_SRCDIR/_main"
    else
      runfiles_root="${TEST_SRCDIR:-.}"
    fi
  fi

  spec_count="$(
    find -L "$runfiles_root" \
      -path '*/node_modules/*' -prune -o \
      -path '*/.next/*' -prune -o \
      -path '*/e2e/*.spec.ts' -type f -print |
      sort -u |
      wc -l
  )"

  if (( spec_count == 0 )) && [[ -n "${TEST_SRCDIR:-}" && "$runfiles_root" != "$TEST_SRCDIR" ]]; then
    spec_count="$(
      find -L "$TEST_SRCDIR" \
        -path '*/node_modules/*' -prune -o \
        -path '*/.next/*' -prune -o \
        -path '*/e2e/*.spec.ts' -type f -print |
        sort -u |
        wc -l
    )"
  fi

  if (( spec_count == 0 )); then
    echo "Playwright Bazel coverage check failed: no runfile specs were discovered."
    exit 1
  fi

  echo "Bazel aggregate CI coverage discovers ${spec_count} Playwright specs from runfiles."
  exit 0
fi

if (( ${#all_specs[@]} == 0 )); then
  echo "Playwright Bazel coverage check failed: no specs were discovered."
  exit 1
fi

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
