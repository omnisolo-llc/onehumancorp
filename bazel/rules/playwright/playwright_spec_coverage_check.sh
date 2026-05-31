#!/usr/bin/env bash
set -euo pipefail

mode=""
all_specs=()
ci_specs=()

for arg in "$@"; do
  case "$arg" in
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

if (( ${#all_specs[@]} == 0 )); then
  echo "Playwright Bazel coverage check failed: no specs were discovered."
  exit 1
fi

all_unique="$(printf '%s\n' "${all_specs[@]}" | sort -u)"
ci_unique="$(printf '%s\n' "${ci_specs[@]}" | sort -u)"

if [[ "$(printf '%s\n' "$all_unique" | wc -l)" != "$(printf '%s\n' "$ci_unique" | wc -l)" ]]; then
  echo "Playwright Bazel coverage check failed: discovered ${#all_specs[@]} specs, but the Bazel aggregate runs ${#ci_specs[@]}."
  comm -23 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique") | sed 's/^/missing from aggregate: /'
  comm -13 <(printf '%s\n' "$all_unique") <(printf '%s\n' "$ci_unique") | sed 's/^/not in spec glob: /'
  exit 1
fi

declare -A ci_spec_set=()
for spec in "${ci_specs[@]}"; do
  ci_spec_set["$spec"]=1
done

for spec in "${all_specs[@]}"; do
  if [[ -z "${ci_spec_set[$spec]:-}" ]]; then
    echo "Playwright Bazel coverage check failed: '$spec' is not included in //src/e2e:playwright."
    exit 1
  fi
done

echo "Bazel aggregate includes all ${#all_specs[@]} Playwright specs from src/e2e/*.spec.ts."
