#!/usr/bin/env bash
set -euo pipefail

if [[ "${OMNISOLO_RUN_LIVE_HARNESS_E2E:-${OMNISOLO_LIVE_HARNESS_E2E:-0}}" != "1" ]]; then
  printf '%s\n' '{"schema":"omnisolo.live_harness_matrix.v1","status":"skipped","reason":"explicit live opt-in is absent","results":[]}'
  exit 0
fi

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

selected_harnesses=",${OMNISOLO_LIVE_HARNESSES-${harnesses[*]}},"
selected_harnesses="${selected_harnesses// /,}"

is_selected() {
  [[ "$selected_harnesses" == *",$1,"* ]]
}

IFS=',' read -r -a requested_harnesses <<< "$selected_harnesses"
selection_count=0
for requested in "${requested_harnesses[@]}"; do
  [[ -z "$requested" ]] && continue
  if [[ " ${harnesses[*]} " != *" $requested "* ]]; then
    printf 'unknown harness: %s\n' "$requested" >&2
    exit 1
  fi
  selection_count=$((selection_count + 1))
done
if (( selection_count == 0 )); then
  echo 'empty harness selection' >&2
  exit 1
fi
native_gate_complete=1
for harness in "${harnesses[@]:0:native_harness_count}"; do
  if ! is_selected "$harness"; then
    native_gate_complete=0
  fi
done

matrix_file="$(mktemp)"
models_file="$(mktemp)"
preflight_file="$(mktemp)"
attempt_file="$(mktemp)"
result_file="$(mktemp)"

plandex_services_started=0
service_state=""
service_image="${OMNISOLO_LOCAL_SERVICE_IMAGE:-omnisolo/harness-local-services:local}"
export OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN="$(python3 -c 'import secrets; print(secrets.token_hex(32))')"

cleanup() {
  if [[ "$plandex_services_started" == "1" ]]; then
    docker rm --force omnisolo-live-plandex-server omnisolo-live-plandex-db >/dev/null 2>&1 || true
  fi
  for harness in "${harnesses[@]}"; do
    if ! is_selected "$harness"; then continue; fi
    docker rm --force "omnisolo-live-$harness" "omnisolo-live-services-$harness" >/dev/null 2>&1 || true
  done
  if [[ -n "$service_state" ]]; then rm -rf -- "$service_state"; fi
  rm -f "$matrix_file" "$models_file" "$preflight_file" "$attempt_file" "$result_file"
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

service_state="$(mktemp -d)"
python3 scripts/live-harness-services.py "$service_state" "${harnesses[@]:0:native_harness_count}"
if [[ "${OMNISOLO_LIVE_BUILD_IMAGES:-1}" == "1" ]]; then
  docker buildx build --load --file deploy/docker/Dockerfile.harness-worker \
    --target local-service-daemon --tag "$service_image" . >&2
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
      . >&2
  done
fi

plandex_server_image="${OMNISOLO_PLANDEX_SERVER_IMAGE:-omnisolo/plandex-server:2.2.1}"
if is_selected plandex && [[ "${OMNISOLO_LIVE_BUILD_IMAGES:-1}" == "1" ]]; then
  docker buildx build --load --file deploy/docker/Dockerfile.plandex-server \
    --tag "$plandex_server_image" . >&2
fi

start_plandex_services() {
  if [[ "$plandex_services_started" == "0" ]]; then
    plandex_services_started=1
    plandex_db_password="$(python3 -c 'import secrets; print(secrets.token_hex(32))')"
    docker rm --force omnisolo-live-plandex-db >/dev/null 2>&1 || true
    POSTGRES_PASSWORD="$plandex_db_password" docker run --detach \
      --name omnisolo-live-plandex-db --env POSTGRES_PASSWORD \
      --env POSTGRES_USER=plandex --env POSTGRES_DB=plandex \
      --tmpfs /var/lib/postgresql/data \
      pgvector/pgvector:pg15@sha256:18d16372b8406bb38a9f94cbff15d125c463d71fde2770aa8b5c64bfcc1578ee >&2 || return 1
  fi
  for probe in $(seq 1 60); do
    if docker exec omnisolo-live-plandex-db pg_isready -U plandex -d plandex >/dev/null 2>&1; then
      break
    fi
    if [[ "$probe" == "60" ]]; then return 1; fi
    sleep 1
  done
  plandex_db_host="$(docker inspect --format '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' omnisolo-live-plandex-db)"
  docker rm --force omnisolo-live-plandex-server >/dev/null 2>&1 || true
  DB_PASSWORD="$plandex_db_password" docker run --detach --name omnisolo-live-plandex-server \
    --network container:omnisolo-live-plandex --read-only \
    --tmpfs /tmp:mode=1777 --tmpfs /var/lib/plandex:mode=0700,uid=1000,gid=1000 \
    --cap-drop ALL --security-opt no-new-privileges:true \
    --env "DB_HOST=$plandex_db_host" --env DB_PORT=5432 --env DB_NAME=plandex \
    --env DB_USER=plandex --env DB_PASSWORD "$plandex_server_image" >&2 || return 1
  for probe in $(seq 1 60); do
    if docker exec omnisolo-live-plandex-server python3 -c \
      'import socket; socket.create_connection(("127.0.0.1",8099),2).close()' >/dev/null 2>&1; then
      return 0
    fi
    if [[ "$probe" == "60" ]]; then return 1; fi
    sleep 1
  done
}

unset OMNISOLO_LIVE_PREVIOUS_KEY OMNISOLO_LIVE_PREVIOUS_VALUE
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
  if (( index >= native_harness_count && (native_gate_failed != 0 || native_gate_complete == 0) )); then
    failed_harnesses+=("$harness")
    gate_failure="native_gate_failed"
    if (( native_gate_complete == 0 )); then gate_failure="native_gate_incomplete"; fi
    jq -nc \
      --arg harness_id "$harness" \
      --arg failure_class "$gate_failure" \
      --arg native_protocol "$native_protocol" \
      --arg integration_mode "${integration_modes[$index]}" \
      --arg model "$OPENAI_MODEL" \
      --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
      '{harness_id:$harness_id,native_protocol:$native_protocol,integration_mode:$integration_mode,status:"failed",failure_class:$failure_class,provider_turn:"not_run",shared_service_probe:"not_run",reasoning_translation:"not_run",cleanup:"not_run",model:$model,reasoning_effort:$reasoning_effort}' \
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
    --env OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS=600
  )
  if (( index < native_harness_count )); then
    run_args+=(--network "container:omnisolo-live-services-$harness"
      --env OMNISOLO_LOCAL_SERVICE_ENDPOINT=http://127.0.0.1:8095
      --env OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN)
  else
    run_args+=(--publish "127.0.0.1:${ports[$index]}:8090")
  fi
  if [[ -n "${executables[$index]}" ]]; then
    run_args+=(
      --env "OMNISOLO_HARNESS_EXECUTABLE=${executables[$index]}"
      --env "OMNISOLO_HARNESS_ARGS_JSON=${arguments[$index]}"
      --env "OMNISOLO_HARNESS_PROTOCOL=${protocols[$index]}"
    )
  fi
  if [[ "$harness" == "plandex" ]]; then
    run_args+=(--env PLANDEX_ENV=development --env PLANDEX_API_HOST=http://127.0.0.1:8099)
  fi
  start_worker() {
    docker rm --force "$container" >/dev/null 2>&1 || true
    if [[ "$harness" == "plandex" ]]; then
      docker rm --force omnisolo-live-plandex-server >/dev/null 2>&1 || true
    fi
    if (( index < native_harness_count )); then
      docker rm --force "omnisolo-live-services-$harness" >/dev/null 2>&1 || true
      docker run --detach --name "omnisolo-live-services-$harness" \
        --publish "127.0.0.1:${ports[$index]}:8090" --read-only \
        --tmpfs /tmp:mode=1777 --tmpfs /home/omnisolo:mode=0700,uid=1000,gid=1000 \
        --cap-drop ALL --security-opt no-new-privileges:true \
        --volume "$service_state:/services" \
        --env OMNISOLO_LOCAL_SERVICE_CONFIG_FILE=/services/config.json \
        --env OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN "$service_image" >&2 || return 1
      for probe in $(seq 1 60); do
        if docker exec "omnisolo-live-services-$harness" node -e \
          'fetch("http://127.0.0.1:8095/v1/configuration",{headers:{Authorization:"Bearer "+process.env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN}}).then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))' >/dev/null 2>&1; then break; fi
        if [[ "$probe" == "60" ]]; then return 1; fi
        sleep 1
      done
    fi
    docker run "${run_args[@]}" "${images[$index]}" >&2 || return 1
    if [[ "$harness" == "plandex" ]]; then
      start_plandex_services
    fi
  }
  worker_started=1
  if ! start_worker; then
    worker_started=0
    if (( index < native_harness_count )); then
      docker logs --tail 200 "omnisolo-live-services-$harness" 2>&1 | jq -Rr 'split(env.OPENAI_API_KEY) | join("[REDACTED]") | split(env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN) | join("[REDACTED]")' >&2 || true
    fi
  fi
  succeeded=0
  started_at=$(date +%s)
  for attempt in $(seq 1 "$OMNISOLO_LIVE_ATTEMPTS"); do
    if [[ "$worker_started" == "0" ]]; then break; fi
    if (( attempt > 1 )) || [[ "$(docker inspect --format '{{.State.Running}}' "$container" 2>/dev/null || true)" != "true" ]]; then
      echo "[$harness] creating a fresh worker before retry $attempt" >&2
      start_worker
    fi
    echo "[$harness] live verification attempt $attempt/$OMNISOLO_LIVE_ATTEMPTS" >&2
    if (( index < native_harness_count )); then
      export OMNISOLO_LIVE_WRITER_SESSION="$(jq -r --arg h "$harness" '.[$h].writer' "$service_state/sessions.json")"
      export OMNISOLO_LIVE_READER_SESSION="$(jq -r --arg h "$harness" '.[$h].reader' "$service_state/sessions.json")"
      export OMNISOLO_LIVE_SERVICE_CONTAINER="omnisolo-live-services-$harness"
    else
      unset OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN
    fi
    if OMNISOLO_LIVE_HARNESS_E2E=1 \
      OMNISOLO_LIVE_HARNESS_ID="$harness" \
      OMNISOLO_LIVE_HARNESS_ENDPOINT="http://127.0.0.1:${ports[$index]}" \
      cargo test -p omnisolo_harness_worker --test live_harness_matrix \
        live_harness_worker_uses_the_real_provider -- --ignored --exact --nocapture >"$attempt_file" 2>&1; then
      if jq -Rsc --arg harness "$harness" --arg model "$OPENAI_MODEL" --arg mode "${integration_modes[$index]}" '
        [split("\n")[] | select(startswith("OMNISOLO_LIVE_RESULT=")) |
          ltrimstr("OMNISOLO_LIVE_RESULT=") | fromjson] |
        select(length == 1) | .[0] |
        select(.schema == "omnisolo.live_harness_result.v1" and
          .status == "passed" and .harness_id == $harness and .model == $model and
          .native_session_deleted == true and .evidence.usage_observed == true and
          .evidence.terminal_success_observed == true and .evidence.provider_marker_observed == true and
          ($mode != "native" or (.evidence.shared_local_services.status == "operations_verified" and
            .evidence.shared_local_services.writer_verified == true and
            .evidence.shared_local_services.reader_verified == true and
            ((env.OMNISOLO_LIVE_PREVIOUS_KEY // "") == "" or
              .evidence.shared_local_services.cross_harness_read_verified == true) and
            (.evidence.shared_local_services.writer_key | type == "string" and length > 0) and
            (.evidence.shared_local_services.writer_value | type == "string" and length > 0))))
      ' "$attempt_file" >"$result_file" && [[ -s "$result_file" ]]; then
        succeeded=1
        break
      fi
      echo "[$harness] test exited successfully without valid live evidence" >&2
    fi
    jq -Rr 'split(env.OPENAI_API_KEY) | join("[REDACTED]") | if (env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN // "") != "" then split(env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN) | join("[REDACTED]") else . end' "$attempt_file" >&2
    docker logs --tail 200 "$container" 2>&1 | jq -Rr 'split(env.OPENAI_API_KEY) | join("[REDACTED]") | if (env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN // "") != "" then split(env.OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN) | join("[REDACTED]") else . end' >&2 || true
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
    if (( index < native_harness_count )); then
      export OMNISOLO_LIVE_PREVIOUS_KEY="$(jq -r '.evidence.shared_local_services.writer_key // empty' "$result_file")"
      export OMNISOLO_LIVE_PREVIOUS_VALUE="$(jq -r '.evidence.shared_local_services.writer_value // empty' "$result_file")"
    fi
    jq -c \
      --arg native_protocol "$native_protocol" \
      --arg integration_mode "${integration_modes[$index]}" \
      --argjson duration_seconds "$duration_seconds" \
      '. + {native_protocol:$native_protocol,integration_mode:$integration_mode,duration_seconds:$duration_seconds}' \
      "$result_file" >>"$matrix_file"
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
  if (( index < native_harness_count )); then docker stop "omnisolo-live-services-$harness" >/dev/null || true; fi
done

matrix_status="passed"
if [[ "${#failed_harnesses[@]}" -ne 0 ]]; then
  matrix_status="failed"
fi
jq -s \
  --arg status "$matrix_status" \
  --arg native_gate "$(if (( native_gate_failed != 0 )); then printf failed; elif (( native_gate_complete == 0 )); then printf not_run; else printf passed; fi)" \
  --arg model "$OPENAI_MODEL" \
  --arg reasoning_effort "$OPENAI_REASONING_EFFORT" \
  '{schema:"omnisolo.live_harness_matrix.v1",status:$status,native_gate:$native_gate,model:$model,reasoning_effort:$reasoning_effort,results:.}' \
  "$matrix_file"
if [[ "$matrix_status" == "failed" ]]; then
  printf 'live harness verification failed: %s\n' "${failed_harnesses[*]}" >&2
  exit 1
fi
