#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

if [[ -z "${OPENAI_API_KEY:-}" ]]; then
  : "${SUB2API_API_KEY:?OPENAI_API_KEY or SUB2API_API_KEY is required}"
  export OPENAI_API_KEY="$SUB2API_API_KEY"
fi
unset SUB2API_API_KEY
export OPENAI_API_BASE_URL="${OPENAI_API_BASE_URL:-https://llmapi.omnisolo.co/v1}"
OPENAI_API_BASE_URL="${OPENAI_API_BASE_URL%/}"
export OPENAI_MODEL="${OPENAI_MODEL:-gpt-5.6-luna}"
export OPENAI_REASONING_EFFORT="${OPENAI_REASONING_EFFORT:-max}"
export OMNISOLO_LIVE_ATTEMPTS="${OMNISOLO_LIVE_ATTEMPTS:-2}"
export OMNISOLO_LIVE_RETRY_DELAY_SECS="${OMNISOLO_LIVE_RETRY_DELAY_SECS:-20}"

matrix_file="$(mktemp)"
models_file="$(mktemp)"
preflight_file="$(mktemp)"

harnesses=(omnisolo codex opencode deepseek pi kimi openhands openharness aider goose open-interpreter plandex)
native_harness_count=8
ports=(8090 8092 8094 8100 8102 8104 8106 8108 8110 8112 8114 8116)
images=(
  omnisolo/harness-worker:0.1.0
  omnisolo/harness-worker-codex:0.149.0
  omnisolo/harness-worker-opencode:1.18.15
  omnisolo/harness-worker-deepseek:0.1.1-rc.2
  omnisolo/harness-worker-pi:0.73.1
  omnisolo/harness-worker-kimi:1.49.0
  omnisolo/harness-worker-openhands:1.43.1
  omnisolo/harness-worker-openharness:0.6.0
  omnisolo/harness-worker-aider:0.86.0
  omnisolo/harness-worker-goose:1.33.1
  omnisolo/harness-worker-open-interpreter:0.4.2
  omnisolo/harness-worker-plandex:2.2.1
)
executables=("" codex opencode dsh-jsonrpc-agent pi omnisolo-kimi-acp openhands-agent-server python3 omnisolo-openai-shim omnisolo-openai-shim omnisolo-openai-shim omnisolo-openai-shim)
arguments=(
  '[]'
  '["app-server","--stdio"]'
  '[]'
  '["/etc/omnisolo/deepseek/cordis.yml"]'
  '["--mode","rpc","--no-session"]'
  '[]'
  '[]'
  '["/opt/omnisolo/openharness_bridge.py"]'
  '[]'
  '[]'
  '[]'
  '[]'
)
protocols=("" codex_app_server opencode_http deepseek_json_rpc pi_rpc kimi_acp openhands_http openharness_sdk openai_compatible_shim openai_compatible_shim openai_compatible_shim openai_compatible_shim)
integration_modes=(native native native native native native native native openai_compatible openai_compatible openai_compatible openai_compatible)

selected_harnesses=",${OMNISOLO_LIVE_HARNESSES:-${harnesses[*]}},"
selected_harnesses="${selected_harnesses// /,}"

is_selected() {
  [[ "$selected_harnesses" == *",$1,"* ]]
}

cleanup() {
  for harness in "${harnesses[@]}"; do
    docker rm --force "omnisolo-live-$harness" >/dev/null 2>&1 || true
  done
  rm -f "$matrix_file" "$models_file" "$preflight_file"
}
trap cleanup EXIT

models_status=$(curl --silent --show-error --max-time 45 --output "$models_file" \
  --write-out '%{http_code}' \
  --config /dev/stdin \
  "$OPENAI_API_BASE_URL/models" \
  <<<"header = \"Authorization: Bearer $OPENAI_API_KEY\"")
if [[ "$models_status" != "200" ]] \
  || ! jq -e --arg model "$OPENAI_MODEL" 'any(.data[]?; .id == $model)' "$models_file" >/dev/null; then
  jq -n \
    --arg status "$models_status" \
    --arg model "$OPENAI_MODEL" \
    '{schema:"omnisolo.live_harness_matrix.v1",status:"failed",failure_class:"model_catalog_preflight_failed",http_status:$status,model:$model,results:[]}'
  exit 1
fi

preflight_status=$(curl --silent --show-error --max-time 90 --output "$preflight_file" \
  --write-out '%{http_code}' \
  --header 'Content-Type: application/json' \
  --config /dev/stdin \
  "$OPENAI_API_BASE_URL/responses" \
  --data "$(jq -nc --arg model "$OPENAI_MODEL" --arg effort "$OPENAI_REASONING_EFFORT" '{model:$model,input:"Reply with exactly OMNISOLO_PROVIDER_PREFLIGHT_OK",reasoning:{effort:$effort},max_output_tokens:32}')" \
  <<<"header = \"Authorization: Bearer $OPENAI_API_KEY\"")
if [[ "$preflight_status" != "200" ]] \
  || ! jq -e '.. | strings | select(. == "OMNISOLO_PROVIDER_PREFLIGHT_OK")' "$preflight_file" >/dev/null; then
  jq -n \
    --arg status "$preflight_status" \
    --arg model "$OPENAI_MODEL" \
    --arg effort "$OPENAI_REASONING_EFFORT" \
    --arg error_type "$(jq -r '.error.type // .type // empty' "$preflight_file" 2>/dev/null)" \
    '{schema:"omnisolo.live_harness_matrix.v1",status:"failed",failure_class:"provider_turn_preflight_failed",http_status:$status,error_type:$error_type,model:$model,reasoning_effort:$effort,results:[]}'
  exit 1
fi

if [[ "${OMNISOLO_LIVE_BUILD_IMAGES:-1}" == "1" ]]; then
  for index in "${!harnesses[@]}"; do
    if ! is_selected "${harnesses[$index]}"; then
      continue
    fi
    docker buildx build --load \
      --progress "${OMNISOLO_DOCKER_PROGRESS:-plain}" \
      --file deploy/docker/Dockerfile.harness-worker \
      --target "${harnesses[$index]}" \
      --tag "${images[$index]}" \
      .
  done
fi

failed_harnesses=()
native_gate_failed=0
for index in "${!harnesses[@]}"; do
  harness="${harnesses[$index]}"
  if ! is_selected "$harness"; then
    continue
  fi
  native_protocol="${protocols[$index]}"
  if [[ -z "$native_protocol" ]]; then
    native_protocol="in_process"
  fi
  if (( index >= native_harness_count && native_gate_failed != 0 )); then
    failed_harnesses+=("$harness")
    jq -nc \
      --arg harness_id "$harness" \
      --arg native_protocol "$native_protocol" \
      --arg integration_mode "${integration_modes[$index]}" \
      --arg model "$OPENAI_MODEL" \
      --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
      '{harness_id:$harness_id,native_protocol:$native_protocol,integration_mode:$integration_mode,status:"failed",failure_class:"native_gate_failed",provider_turn:"not_run",shared_service_probe:"not_run",reasoning_translation:"not_run",cleanup:"not_run",model:$model,reasoning_effort:$reasoning_effort}' \
      >>"$matrix_file"
    continue
  fi
  container="omnisolo-live-$harness"
  tmp_mount="/tmp:mode=1777"
  if [[ "$harness" == "openhands" ]]; then
    # The official packaged launcher extracts native libraries under /tmp.
    tmp_mount="/tmp:exec,mode=1777"
  fi
  run_args=(
    --detach
    --name "$container"
    --publish "127.0.0.1:${ports[$index]}:8090"
    --read-only
    --tmpfs "$tmp_mount"
    --tmpfs /workspace:mode=0700,uid=1000,gid=1000
    --tmpfs /home/omnisolo:mode=0700,uid=1000,gid=1000
    --cap-drop ALL
    --security-opt no-new-privileges:true
    --env "OMNISOLO_HARNESS_WORKER_ID=harness-$harness"
    --env "OMNISOLO_HARNESS_ID=$harness"
    --env "OMNISOLO_HARNESS_POOL_ID=$harness"
    --env OPENAI_API_KEY
    --env OPENAI_API_BASE_URL
    --env OPENAI_MODEL
    --env OPENAI_REASONING_EFFORT
  )
  if [[ -n "${executables[$index]}" ]]; then
    run_args+=(
      --env "OMNISOLO_HARNESS_EXECUTABLE=${executables[$index]}"
      --env "OMNISOLO_HARNESS_ARGS_JSON=${arguments[$index]}"
      --env "OMNISOLO_HARNESS_PROTOCOL=${protocols[$index]}"
      --env OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS=90
    )
  fi
  start_worker() {
    docker rm --force "$container" >/dev/null 2>&1 || true
    docker run "${run_args[@]}" "${images[$index]}"
  }
  start_worker
  succeeded=0
  started_at=$(date +%s)
  for attempt in $(seq 1 "$OMNISOLO_LIVE_ATTEMPTS"); do
    if [[ "$(docker inspect --format '{{.State.Running}}' "$container" 2>/dev/null || true)" != "true" ]]; then
      echo "[$harness] worker exited; recreating it before retry $attempt" >&2
      start_worker
    fi
    echo "[$harness] live verification attempt $attempt/$OMNISOLO_LIVE_ATTEMPTS"
    if OMNISOLO_LIVE_HARNESS_E2E=1 \
      OMNISOLO_LIVE_HARNESS_ID="$harness" \
      OMNISOLO_LIVE_HARNESS_ENDPOINT="http://127.0.0.1:${ports[$index]}" \
      cargo test -p omnisolo_harness_worker --test live_harness_matrix \
        live_harness_worker_uses_the_real_provider -- --ignored --exact --nocapture; then
      succeeded=1
      break
    fi
    docker logs --tail 200 "$container" >&2 || true
    docker inspect --format \
      'container={{.Name}} status={{.State.Status}} exit={{.State.ExitCode}} oom={{.State.OOMKilled}} error={{.State.Error}}' \
      "$container" >&2 || true
    if [[ "$attempt" -lt "$OMNISOLO_LIVE_ATTEMPTS" ]]; then
      sleep "$OMNISOLO_LIVE_RETRY_DELAY_SECS"
    fi
  done
  if [[ "$succeeded" != "1" ]]; then
    failed_harnesses+=("$harness")
    if (( index < native_harness_count )); then
      native_gate_failed=1
    fi
  fi
  duration_seconds=$(($(date +%s) - started_at))
  if [[ "$succeeded" == "1" ]]; then
    jq -nc \
      --arg harness_id "$harness" \
      --arg native_protocol "$native_protocol" \
      --arg integration_mode "${integration_modes[$index]}" \
      --arg model "$OPENAI_MODEL" \
      --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
      --arg reasoning_translation "$OPENAI_REASONING_EFFORT" \
      --argjson duration_seconds "$duration_seconds" \
      '{harness_id:$harness_id,native_protocol:$native_protocol,integration_mode:$integration_mode,status:"passed",provider_turn:"passed",shared_service_probe:"passed",reasoning_translation:$reasoning_translation,cleanup:"passed",model:$model,reasoning_effort:$reasoning_effort,duration_seconds:$duration_seconds}' \
      >>"$matrix_file"
  else
    jq -nc \
      --arg harness_id "$harness" \
      --arg native_protocol "$native_protocol" \
      --arg integration_mode "${integration_modes[$index]}" \
      --arg model "$OPENAI_MODEL" \
      --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
      --argjson duration_seconds "$duration_seconds" \
      '{harness_id:$harness_id,native_protocol:$native_protocol,integration_mode:$integration_mode,status:"failed",failure_class:"harness_execution_failed",provider_turn:"not_verified",shared_service_probe:"not_verified",reasoning_translation:"not_verified",cleanup:"not_verified",model:$model,reasoning_effort:$reasoning_effort,duration_seconds:$duration_seconds}' \
      >>"$matrix_file"
  fi
  docker stop "$container" >/dev/null || true
done

matrix_status="passed"
if [[ "${#failed_harnesses[@]}" -ne 0 ]]; then
  matrix_status="failed"
fi
jq -s \
  --arg status "$matrix_status" \
  --arg native_gate "$( (( native_gate_failed == 0 )) && printf passed || printf failed )" \
  --arg model "$OPENAI_MODEL" \
  --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
  '{schema:"omnisolo.live_harness_matrix.v1",status:$status,native_gate:$native_gate,model:$model,reasoning_effort:$reasoning_effort,results:.}' \
  "$matrix_file"
if [[ "$matrix_status" == "failed" ]]; then
  printf 'live harness verification failed: %s\n' "${failed_harnesses[*]}" >&2
  exit 1
fi
