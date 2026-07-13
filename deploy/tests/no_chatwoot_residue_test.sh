#!/usr/bin/env bash
set -euo pipefail

scanner_error() {
  printf 'chat platform residue scanner failed' >&2
  for detail in "$@"; do
    printf ' %q' "$detail" >&2
  done
  printf '\n' >&2
  exit 2
}

if ! repo_root="$(git rev-parse --show-toplevel 2>/dev/null)"; then
  scanner_error "repository root unavailable"
fi
if ! cd "$repo_root" 2>/dev/null; then
  scanner_error "repository root inaccessible" "$repo_root"
fi

guard_path="deploy/tests/no_chatwoot_residue_test.sh"
active_roots=(
  Cargo.toml
  README.md
  src
  deploy
  docs/business
  'docs/research/[customer_success]_invisible_ambassador_agent.md'
  docs/technical/developer
  docs/technical/reports
)

if mapfile -d '' -t tracked_records < <(git ls-files -s -z -- "${active_roots[@]}" 2>/dev/null); then
  inventory_pid=$!
else
  scanner_error "active inventory read failed"
fi
if ! wait "$inventory_pid"; then
  scanner_error "active inventory command failed"
fi
tracked=()
symlinks=()
for record in "${tracked_records[@]}"; do
  if [[ "$record" != *$'\t'* ]]; then
    scanner_error "malformed active inventory record"
  fi
  metadata="${record%%$'\t'*}"
  path="${record#*$'\t'}"
  [[ -z "$path" || "$path" == "$guard_path" ]] && continue
  mode="${metadata%% *}"
  case "$mode" in
    100644|100755)
      if [[ -L "$path" || ! -f "$path" || ! -r "$path" ]]; then
        scanner_error "tracked active regular input invalid" "$path"
      fi
      tracked+=("$path")
      ;;
    120000)
      if [[ ! -L "$path" ]]; then
        scanner_error "tracked active symlink input invalid" "$path"
      fi
      tracked+=("$path")
      symlinks+=("$path")
      ;;
    *)
      scanner_error "unsupported tracked active mode" "$mode" "$path"
      ;;
  esac
done
if ((${#tracked[@]} == 0)); then
  scanner_error "no tracked active files were discovered"
fi

if mapfile -d '' -t matching_paths < <(git grep -i -l -z 'chatwoot' -- "${tracked[@]}" 2>/dev/null); then
  grep_pid=$!
else
  scanner_error "active residue result read failed"
fi
if wait "$grep_pid"; then
  grep_status=0
else
  grep_status=$?
fi
if [[ "$grep_status" -ne 0 && "$grep_status" -ne 1 ]]; then
  scanner_error "active residue command failed"
fi

symlink_match_paths=()
for path in "${symlinks[@]}"; do
  if ! target="$(readlink -- "$path" 2>/dev/null)"; then
    scanner_error "tracked active symlink read failed" "$path"
  fi
  if [[ "${target,,}" == *chatwoot* ]]; then
    symlink_match_paths+=("$path")
  fi
done

if ((${#matching_paths[@]} != 0 || ${#symlink_match_paths[@]} != 0)); then
  echo "active Chatwoot residue remains:" >&2
  for path in "${matching_paths[@]}"; do
    printf 'active file: %q\n' "$path" >&2
  done
  for path in "${symlink_match_paths[@]}"; do
    printf 'active symlink: %q\n' "$path" >&2
  done
  exit 1
fi

historical=(
  docs/research/ohc_tool_integration_research_report.md
  docs/reports/tool_integration_research_report_q3.md
  docs/research/triage_report_bazel.md
)
historical_marker='> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.'
for path in "${historical[@]}"; do
  if ! git ls-files --error-unmatch "$path" >/dev/null 2>&1; then
    scanner_error "historical inventory lookup failed" "$path"
  fi
  if [[ -L "$path" || ! -f "$path" || ! -r "$path" ]]; then
    scanner_error "tracked historical regular input invalid" "$path"
  fi
  historical_lines=()
  if ! mapfile -t -n 2 historical_lines < "$path"; then
    scanner_error "historical marker read failed" "$path"
  fi
  if [[ "${historical_lines[1]-}" != "$historical_marker" ]]; then
    printf 'missing or misplaced native-architecture superseded marker: %q\n' "$path" >&2
    exit 1
  fi
done
