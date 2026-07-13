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
historical=(
  docs/research/ohc_tool_integration_research_report.md
  docs/reports/tool_integration_research_report_q3.md
  docs/research/triage_report_bazel.md
)
allowed_reference_paths=(
  .github/workflows/ci.yml
  "$guard_path"
  docs/superpowers/plans/2026-07-13-chatwoot-removal.md
  docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md
  docs/reports/production_agent_optimization_report.md
  "${historical[@]}"
)

is_allowed_reference() {
  local candidate="$1"
  local allowed
  for allowed in "${allowed_reference_paths[@]}"; do
    [[ "$candidate" == "$allowed" ]] && return 0
  done
  return 1
}

if mapfile -d '' -t tracked_records < <(git ls-files -s -z -- . 2>/dev/null); then
  inventory_pid=$!
else
  scanner_error "tracked inventory read failed"
fi
if ! wait "$inventory_pid"; then
  scanner_error "tracked inventory command failed"
fi
tracked=()
scan_pathspecs=()
symlinks=()
declare -A tracked_seen=()
append_tracked() {
  local candidate="$1"
  [[ -n "${tracked_seen["$candidate"]+present}" ]] && return 0
  tracked_seen["$candidate"]=1
  tracked+=("$candidate")
  scan_pathspecs+=(":(top,literal)$candidate")
}
for record in "${tracked_records[@]}"; do
  if [[ "$record" != *$'\t'* ]]; then
    scanner_error "malformed tracked inventory record"
  fi
  metadata="${record%%$'\t'*}"
  path="${record#*$'\t'}"
  [[ -z "$path" ]] && scanner_error "empty tracked inventory path"
  mode="${metadata%% *}"
  case "$mode" in
    100644|100755)
      if [[ -L "$path" || ! -f "$path" || ! -r "$path" ]]; then
        scanner_error "tracked regular input invalid" "$path"
      fi
      if ! is_allowed_reference "$path"; then
        append_tracked "$path"
      fi
      ;;
    120000)
      if [[ ! -L "$path" ]]; then
        scanner_error "tracked symlink input invalid" "$path"
      fi
      if is_allowed_reference "$path"; then
        scanner_error "allowed reference must be a regular file" "$path"
      fi
      append_tracked "$path"
      symlinks+=("$path")
      ;;
    *)
      scanner_error "unsupported tracked mode" "$mode" "$path"
      ;;
  esac
done
if ((${#tracked[@]} == 0)); then
  scanner_error "no tracked scan files were discovered"
fi

if ! physical_repo_root="$(realpath -e -- "$repo_root" 2>/dev/null)"; then
  scanner_error "repository root resolution failed" "$repo_root"
fi
symlink_match_paths=()
for path in "${symlinks[@]}"; do
  if mapfile -d '' -t link_targets < <(readlink -z -- "$path" 2>/dev/null); then
    readlink_pid=$!
  else
    scanner_error "tracked symlink result read failed" "$path"
  fi
  if ! wait "$readlink_pid" || ((${#link_targets[@]} != 1)); then
    scanner_error "tracked symlink read failed" "$path"
  fi
  target="${link_targets[0]}"
  if [[ "${target,,}" == *chatwoot* ]]; then
    symlink_match_paths+=("$path")
  fi
  if mapfile -d '' -t resolved_targets < <(realpath -m -z -- "$path" 2>/dev/null); then
    realpath_pid=$!
  else
    scanner_error "tracked symlink resolution result read failed" "$path"
  fi
  if ! wait "$realpath_pid" || ((${#resolved_targets[@]} != 1)); then
    scanner_error "tracked symlink resolution failed" "$path"
  fi
  resolved_target="${resolved_targets[0]}"
  case "$resolved_target" in
    "$physical_repo_root"/*)
      resolved_relative="${resolved_target#"$physical_repo_root"/}"
      ;;
    *)
      scanner_error "tracked symlink target escapes repository" "$path"
      ;;
  esac
  if mapfile -d '' -t target_records < <(git ls-files -s -z -- ":(top,literal)$resolved_relative" 2>/dev/null); then
    target_inventory_pid=$!
  else
    scanner_error "tracked symlink target inventory read failed" "$path"
  fi
  if ! wait "$target_inventory_pid"; then
    scanner_error "tracked symlink target inventory command failed" "$path"
  fi
  if ((${#target_records[@]} == 0)); then
    continue
  fi
  if ((${#target_records[@]} != 1)) || [[ "${target_records[0]}" != *$'\t'* ]]; then
    scanner_error "tracked symlink target inventory malformed" "$path"
  fi
  target_metadata="${target_records[0]%%$'\t'*}"
  target_path="${target_records[0]#*$'\t'}"
  if [[ "$target_path" != "$resolved_relative" ]]; then
    scanner_error "tracked symlink target inventory mismatch" "$path"
  fi
  target_mode="${target_metadata%% *}"
  case "$target_mode" in
    100644|100755)
      if [[ -L "$target_path" || ! -f "$target_path" || ! -r "$target_path" ]]; then
        scanner_error "tracked symlink target regular input invalid" "$path" "$target_path"
      fi
      append_tracked "$target_path"
      ;;
    120000)
      scanner_error "tracked symlink target did not resolve" "$path" "$target_path"
      ;;
    *)
      scanner_error "unsupported tracked symlink target mode" "$target_mode" "$path" "$target_path"
      ;;
  esac
done

if mapfile -d '' -t matching_paths < <(git grep -i -l -z 'chatwoot' -- "${scan_pathspecs[@]}" 2>/dev/null); then
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
  scanner_error "tracked residue command failed"
fi

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

historical_marker='> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.'
for path in "${historical[@]}"; do
  if ! git ls-files --error-unmatch ":(top,literal)$path" >/dev/null 2>&1; then
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
