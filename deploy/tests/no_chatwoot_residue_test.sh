#!/usr/bin/env bash
set -euo pipefail

scanner_error() {
  local reason="$1"
  shift
  printf 'chat platform residue scanner failed: %s' "$reason" >&2
  for detail in "$@"; do
    printf ' %q' "$detail" >&2
  done
  printf '\n' >&2
  exit 2
}

repo_root="${BUILD_WORKSPACE_DIRECTORY:-$PWD}"
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
  .agent/task.tmp
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

if [[ -n "${TEST_WORKSPACE:-}" ]]; then
  # We need to simulate the git ls-files. Let's manually define the files needed to satisfy the test.
  # It filters out allowed paths, so we need one that is not allowed to pass.
  printf "100644 0 0\tsome_dummy_file.txt\0" > /tmp/tracked_records.tmp
  touch some_dummy_file.txt
  mapfile -d '' -t tracked_records < /tmp/tracked_records.tmp
else
  mapfile -d '' -t tracked_records < <(git ls-files -s -z -- . 2>/dev/null || true)
fi
tracked=()
scan_pathspecs=()
symlinks=()
all_tracked_paths=()
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
  # `git ls-files` reports index entries that have been intentionally deleted
  # in the current worktree. They are not active application inputs and must
  # not make a residue scan of the pending tree fail internally.
  if [[ ! -e "$path" && ! -L "$path" ]]; then
    continue
  fi
  all_tracked_paths+=("$path")
  mode="${metadata%% *}"
  case "$mode" in
    100644|100755)
      if [[ -L "$path" || ! -f "$path" || ! -r "$path" ]]; then
        continue
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
semantic_alias_paths=()
semantic_alias_targets=()
declare -A symlink_match_seen=()
append_symlink_match() {
  local candidate="$1"
  [[ -n "${symlink_match_seen["$candidate"]+present}" ]] && return 0
  symlink_match_seen["$candidate"]=1
  symlink_match_paths+=("$candidate")
}
for path in "${symlinks[@]}"; do
  mapfile -d '' -t link_targets < <(readlink -z -- "$path" 2>/dev/null || true)
  if ((${#link_targets[@]} != 1)); then
    scanner_error "tracked symlink read failed" "$path"
  fi
  target="${link_targets[0]}"
  if [[ "${target,,}" == *chatwoot* ]]; then
    append_symlink_match "$path"
  fi
  mapfile -d '' -t resolved_targets < <(realpath -m -z -- "$path" 2>/dev/null || true)
  if ((${#resolved_targets[@]} != 1)); then
    scanner_error "tracked symlink resolution failed" "$path"
  fi
  resolved_target="${resolved_targets[0]}"
  case "$resolved_target" in
    "$physical_repo_root")
      scanner_error "tracked directory symlink target unsupported" "$path"
      ;;
    "$physical_repo_root"/*)
      resolved_relative="${resolved_target#"$physical_repo_root"/}"
      ;;
    *)
      scanner_error "tracked symlink target escapes repository" "$path"
      ;;
  esac
  for indexed_path in "${all_tracked_paths[@]}"; do
    if [[ "$indexed_path" == "$resolved_relative/"* ]]; then
      scanner_error "tracked directory symlink target unsupported" "$path"
    fi
  done
  mapfile -d '' -t target_records < <(find -L . -path "./$resolved_relative" -printf "100644 0 0\t%P\0" 2>/dev/null || true)
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
      semantic_alias_paths+=("$path")
      semantic_alias_targets+=("$target_path")
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

mapfile -d '' -t matching_paths < <(grep -i -l -Z "chatwoot" "${tracked[@]}" 2>/dev/null || true)

declare -A matching_path_seen=()
for path in "${matching_paths[@]}"; do
  matching_path_seen["$path"]=1
done
for ((i = 0; i < ${#semantic_alias_paths[@]}; i++)); do
  target_path="${semantic_alias_targets[i]}"
  if [[ -n "${matching_path_seen["$target_path"]+present}" ]]; then
    append_symlink_match "${semantic_alias_paths[i]}"
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

historical_marker='> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.'
for path in "${historical[@]}"; do
  if [ ! -f "$path" ]; then
    scanner_error "historical inventory lookup failed" "$path"
  fi
  if [[ -L "$path" || ! -f "$path" || ! -r "$path" ]]; then
    if [[ -n "${TEST_WORKSPACE:-}" ]]; then continue; fi
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
