#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

guard_path="deploy/tests/no_chatwoot_residue_test.sh"
active_roots=(
  Cargo.toml
  README.md
  src
  deploy
  docs/business
  docs/technical/developer
  docs/technical/reports
)

if ! tracked_inventory="$(git ls-files -s -- "${active_roots[@]}")"; then
  echo "chat platform residue scanner failed" >&2
  exit 2
fi
tracked=()
symlinks=()
while IFS=$'\t' read -r metadata path; do
  [[ -z "$path" || "$path" == "$guard_path" ]] && continue
  tracked+=("$path")
  [[ "${metadata%% *}" == "120000" ]] && symlinks+=("$path")
done <<< "$tracked_inventory"
if ((${#tracked[@]} == 0)); then
  echo "chat platform residue scan failed: no tracked active files were discovered" >&2
  exit 2
fi

matches=""
if matches="$(git grep -n -i 'chatwoot' -- "${tracked[@]}")"; then
  :
elif [[ $? -ne 1 ]]; then
  echo "chat platform residue scanner failed" >&2
  exit 2
fi

symlink_matches=()
for path in "${symlinks[@]}"; do
  if ! target="$(readlink -- "$path")"; then
    echo "chat platform residue scanner failed" >&2
    exit 2
  fi
  if [[ "${target,,}" == *chatwoot* ]]; then
    symlink_matches+=("$path -> $target")
  fi
done

if [[ -n "$matches" ]] || ((${#symlink_matches[@]} != 0)); then
  echo "active Chatwoot residue remains:" >&2
  if [[ -n "$matches" ]]; then
    printf '%s\n' "$matches" >&2
  fi
  if ((${#symlink_matches[@]} != 0)); then
    printf '%s\n' "${symlink_matches[@]}" >&2
  fi
  exit 1
fi

historical=(
  docs/research/ohc_tool_integration_research_report.md
  docs/reports/tool_integration_research_report_q3.md
  docs/research/triage_report_bazel.md
)
for path in "${historical[@]}"; do
  git ls-files --error-unmatch "$path" >/dev/null
  rg -q '^> Superseded architecture: .*native omnichannel' "$path" || {
    echo "missing native-architecture superseded marker: $path" >&2
    exit 1
  }
done
