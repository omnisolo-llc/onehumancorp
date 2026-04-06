#!/usr/bin/env bash
set -euo pipefail

# Test to verify standalone_ohc.sh respects OHC_STANDALONE when cleaning up tmp files.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WRAPPER_SCRIPT="${SCRIPT_DIR}/standalone_ohc.sh"

if [[ ! -f "${WRAPPER_SCRIPT}" ]]; then
  echo "ERROR: standalone_ohc.sh not found at ${WRAPPER_SCRIPT}" >&2
  exit 1
fi

TMP_WORKSPACE="$(mktemp -d)"
export HOME="${TMP_WORKSPACE}"
export OHC_LISTEN_ADDR="40123"

# Mock the underlying ohc server binary so it doesn't actually run anything blocking
mkdir -p "${TMP_WORKSPACE}/bin"
cat >"${TMP_WORKSPACE}/bin/ohc-server" <<'EOF'
#!/usr/bin/env bash
echo "mock server starting"
# Start a dummy server so wait_for_port succeeds
# We use python to create a simple listener that will respond immediately
python3 -c "import socket; s = socket.socket(); s.bind(('127.0.0.1', int('${PORT}'))); s.listen(1); print('listening'); c,a=s.accept(); print('accepted'); c.close()" &
MOCK_PID=$!
sleep 10
kill $MOCK_PID 2>/dev/null || true
echo "mock server exiting"
EOF
chmod +x "${TMP_WORKSPACE}/bin/ohc-server"
export PATH="${TMP_WORKSPACE}/bin:${PATH}"

export RUNFILES_DIR="${TMP_WORKSPACE}"
export TEST_WORKSPACE="mono"
mkdir -p "${TMP_WORKSPACE}/mono/srcs/server"
cp "${TMP_WORKSPACE}/bin/ohc-server" "${TMP_WORKSPACE}/mono/srcs/server/ohc"

cleanup() {
    rm -rf "${TMP_WORKSPACE}"
}
trap cleanup EXIT

STATE_DIR="${TMP_WORKSPACE}/.openclaw"
mkdir -p "${STATE_DIR}"

run_test_cleanup_standalone() {
  echo "Running test: Cleanup with OHC_STANDALONE=true"
  touch "${STATE_DIR}/test1.tmp"
  touch "${STATE_DIR}/keep1.log"

  export OHC_STANDALONE="true"
  "${WRAPPER_SCRIPT}" start --daemon

  sleep 1 # Wait for daemon to start
  "${WRAPPER_SCRIPT}" stop

  if [[ -f "${STATE_DIR}/test1.tmp" ]]; then
    echo "ERROR: tmp file was not cleaned up when OHC_STANDALONE=true" >&2
    exit 1
  fi

  if [[ ! -f "${STATE_DIR}/keep1.log" ]]; then
    echo "ERROR: non-tmp file was unexpectedly deleted" >&2
    exit 1
  fi
  echo "PASS: test_cleanup_standalone"
}

run_test_cleanup_cloud() {
  echo "Running test: Cleanup with OHC_STANDALONE=false"
  touch "${STATE_DIR}/test2.tmp"
  touch "${STATE_DIR}/keep2.log"

  export OHC_STANDALONE="false"
  "${WRAPPER_SCRIPT}" start --daemon

  sleep 1 # Wait for daemon to start
  "${WRAPPER_SCRIPT}" stop

  if [[ ! -f "${STATE_DIR}/test2.tmp" ]]; then
    echo "ERROR: tmp file was unexpectedly cleaned up when OHC_STANDALONE=false" >&2
    exit 1
  fi

  if [[ ! -f "${STATE_DIR}/keep2.log" ]]; then
    echo "ERROR: non-tmp file was unexpectedly deleted" >&2
    exit 1
  fi
  echo "PASS: test_cleanup_cloud"
}

run_test_cleanup_standalone
run_test_cleanup_cloud

echo "All tests passed!"
