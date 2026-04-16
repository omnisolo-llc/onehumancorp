#!/usr/bin/env bash
set -euo pipefail

resolve_script_dir() {
  local source="${BASH_SOURCE[0]}"

  while [[ -L "${source}" ]]; do
    local dir
    dir="$(cd -P -- "$(dirname -- "${source}")" && pwd)"
    source="$(readlink -- "${source}")"
    if [[ "${source}" != /* ]]; then
      source="${dir}/${source}"
    fi
  done

  cd -P -- "$(dirname -- "${source}")" && pwd
}

find_runfiles_root() {
  local candidate

  for candidate in \
    "${RUNFILES_DIR:-}" \
    "${BASH_SOURCE[0]}.runfiles" \
    "$0.runfiles" \
    "${TEST_SRCDIR:-}"; do
    if [[ -n "${candidate}" && -d "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done

  return 1
}

find_server_bin() {
  local script_dir="$1"
  local script_path
  local runfiles_root
  local candidate

  script_path="$(readlink -f -- "${BASH_SOURCE[0]}")"
  runfiles_root="$(find_runfiles_root || true)"

  for candidate in \
    "${script_dir}/ohc-server" \
    "${script_dir}/ohc" \
    "${runfiles_root}/_main/srcs/server/ohc" \
    "${runfiles_root}/_main/srcs/server/ohc_/ohc" \
    "${runfiles_root}/__main__/srcs/server/ohc" \
    "${runfiles_root}/__main__/srcs/server/ohc_/ohc" \
    "${runfiles_root}/${TEST_WORKSPACE:-mono}/srcs/server/ohc" \
    "${runfiles_root}/${TEST_WORKSPACE:-mono}/srcs/server/ohc_/ohc"; do
    if [[ -n "${candidate}" && -x "${candidate}" ]]; then
      if [[ "$(readlink -f -- "${candidate}")" != "${script_path}" ]]; then
        printf '%s\n' "${candidate}"
        return 0
      fi
    fi
  done

  return 1
}

config_string() {
  local key="$1"

  if [[ ! -f "${CONFIG_FILE}" ]]; then
    return 0
  fi

  grep -o "\"${key}\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" "${CONFIG_FILE}" 2>/dev/null \
    | head -n 1 \
    | sed -E 's/.*:[[:space:]]*"([^"]*)"/\1/' || true
}

resolve_port() {
  local configured="${PORT:-${LISTEN_ADDR}}"

  if [[ -z "${configured}" ]]; then
    configured="18789"
  fi

  if [[ "${configured}" == *:* ]]; then
    configured="${configured##*:}"
  fi

  if [[ ! "${configured}" =~ ^[0-9]+$ ]]; then
    configured="18789"
  fi

  printf '%s\n' "${configured}"
}

is_pid_running() {
  if [[ ! -f "${PID_FILE}" ]]; then
    return 1
  fi

  local pid
  pid="$(cat "${PID_FILE}")"
  [[ -n "${pid}" ]] && kill -0 "${pid}" 2>/dev/null
}

wait_for_port() {
  local port="$1"
  local attempt

  for attempt in $(seq 1 40); do
    if (echo >"/dev/tcp/127.0.0.1/${port}") >/dev/null 2>&1; then
      return 0
    fi
    sleep 0.25
  done

  return 1
}

print_doctor() {
  local port="$1"
  local status="stopped"

  if is_pid_running; then
    status="running"
  fi

  echo "standalone wrapper: ${SCRIPT_PATH}"
  echo "server binary: ${SERVER_BIN}"
  echo "state dir: ${STATE_DIR}"
  echo "listen port: ${port}"
  echo "status: ${status}"
  if wait_for_port "${port}"; then
    echo "health: reachable"
  else
    echo "health: unreachable"
  fi
}


cleanup_tmp_files() {
  if [[ "${OHC_STANDALONE:-true}" == "true" ]]; then
    find "${STATE_DIR}" -name "*.tmp" -type f -mtime +1 -delete 2>/dev/null || true
  fi
}

start_daemon() {
  local port="$1"

  mkdir -p "${STATE_DIR}"
  chmod 700 "${STATE_DIR}"

  if is_pid_running; then
    echo "ohc already running on port ${port}"
    return 0
  fi

  rm -f "${PID_FILE}" "${LOG_FILE}"
  cleanup_tmp_files
  touch "${LOG_FILE}" "${PID_FILE}"
  chmod 0600 "${LOG_FILE}" "${PID_FILE}"

  env \
    HOME="${HOME}" \
    PORT="${port}" \
    GRPC_PORT="${GRPC_PORT:-0}" \
    GOMEMLIMIT="${GOMEMLIMIT:-128MiB}" \
    GOGC="${GOGC:-30}" \
    OHC_STANDALONE="true" \
    OHC_SQLITE_KEY="${OHC_SQLITE_KEY:-standalone_ephemeral_key}" \
    nohup "${SERVER_BIN}" >"${LOG_FILE}" 2>&1 &
  local pid=$!
  echo "${pid}" >"${PID_FILE}"

  if ! wait_for_port "${port}"; then
    echo "failed to start ohc on port ${port}; see ${LOG_FILE}" >&2
    return 1
  fi

  echo "ohc started on port ${port}"
}

stop_daemon() {
  if ! is_pid_running; then
    rm -f "${PID_FILE}" "${LOG_FILE}"
    cleanup_tmp_files
    echo "ohc is not running"
    return 0
  fi

  local pid
  local attempt
  pid="$(cat "${PID_FILE}")"

  kill "${pid}" 2>/dev/null || true
  pkill -P "${pid}" 2>/dev/null || true
  for attempt in $(seq 1 20); do
    if ! kill -0 "${pid}" 2>/dev/null; then
      rm -f "${PID_FILE}" "${LOG_FILE}"
      cleanup_tmp_files
      echo "ohc stopped"
      return 0
    fi
    sleep 0.25
  done

  pkill -9 -P "${pid}" 2>/dev/null || true
  kill -9 "${pid}" 2>/dev/null || true
  rm -f "${PID_FILE}" "${LOG_FILE}"
  cleanup_tmp_files
  echo "ohc stopped"
}

SCRIPT_DIR="$(resolve_script_dir)"
SCRIPT_PATH="$(readlink -f -- "${BASH_SOURCE[0]}")"
STATE_DIR="${HOME}/.openclaw"
CONFIG_FILE="${STATE_DIR}/openclaw.json"
PID_FILE="${STATE_DIR}/ohc.pid"
LOG_FILE="${STATE_DIR}/ohc.log"
LISTEN_ADDR="${OHC_LISTEN_ADDR:-$(config_string listen_addr)}"
PORT_VALUE="$(resolve_port)"
SERVER_BIN="$(find_server_bin "${SCRIPT_DIR}" || true)"

if [[ -z "${SERVER_BIN}" ]]; then
  echo "failed to locate the underlying ohc server binary" >&2
  exit 1
fi

case "${1:-}" in
  start)
    if [[ "${2:-}" != "--daemon" ]]; then
      echo "usage: ohc start --daemon" >&2
      exit 1
    fi
    start_daemon "${PORT_VALUE}"
    ;;
  stop)
    stop_daemon
    ;;
  doctor)
    print_doctor "${PORT_VALUE}"
    ;;
  "")
    exec "${SERVER_BIN}"
    ;;
  *)
    exec "${SERVER_BIN}" "$@"
    ;;
esac