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

mapfile -t tracked < <(git ls-files -- "${active_roots[@]}" | awk -v guard="$guard_path" '$0 != guard')
if ((${#tracked[@]} == 0)); then
  echo "chat platform residue scan failed: no tracked active files were discovered" >&2
  exit 2
fi

if matches="$(git grep -n -i 'chatwoot' -- "${tracked[@]}")"; then
  echo "active Chatwoot residue remains:" >&2
  printf '%s\n' "$matches" >&2
  exit 1
elif [[ $? -ne 1 ]]; then
  echo "chat platform residue scanner failed" >&2
  exit 2
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
