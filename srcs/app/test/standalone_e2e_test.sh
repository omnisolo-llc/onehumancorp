#!/usr/bin/env bash
set -euo pipefail

WORKSPACE="${TEST_WORKSPACE:-mono}"
RUNFILES="${TEST_SRCDIR:-$PWD}"
TMPDIR="${TEST_TMPDIR:-/tmp/ohc_standalone_e2e}"

find_desktop_launcher() {
  local candidate

  for candidate in \
    "${RUNFILES}/${WORKSPACE}/desktop" \
    "${RUNFILES}/_main/desktop" \
    "${RUNFILES}/__main__/desktop"; do
    if [[ -x "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done

  find "${RUNFILES}" -maxdepth 4 -type f -name desktop -perm -u+x | head -n 1
}

desktop_launcher="$(find_desktop_launcher)"
if [[ -z "${desktop_launcher}" || ! -x "${desktop_launcher}" ]]; then
  echo "ERROR: desktop launcher not found in runfiles" >&2
  exit 1
fi

fake_bin_dir="${TMPDIR}/fake-bin"
fake_flutter="${fake_bin_dir}/flutter"
mkdir -p "${fake_bin_dir}" "${TMPDIR}/home"

cat >"${fake_flutter}" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ "$1" != "run" || "$2" != "-d" || "$3" != "linux" ]]; then
  echo "unexpected flutter invocation: $*" >&2
  exit 1
fi

if [[ "$4" != "--dart-define=OHC_STANDALONE=true" ]]; then
  echo "expected standalone dart-define, got: $*" >&2
  exit 1
fi

ohc start --daemon

python3 - <<'PY'
import time
import urllib.request

for _ in range(40):
    try:
        with urllib.request.urlopen("http://127.0.0.1:18789/healthz", timeout=0.5) as response:
            if response.read().decode("utf-8").strip() == "ok":
                break
    except Exception:
        time.sleep(0.25)
else:
    raise SystemExit("backend never became healthy")
PY

ohc doctor >"${TEST_TMPDIR}/standalone_doctor.txt"
ohc stop
touch "${TEST_TMPDIR}/standalone_ok"
EOF
chmod 0755 "${fake_flutter}"

export HOME="${TMPDIR}/home"
# Override the flutter binary used by the desktop launcher so the fake flutter
# is called instead of the hermetic SDK binary baked into the launcher script.
export FLUTTER_BIN_OVERRIDE="${fake_flutter}"

"${desktop_launcher}"

if [[ ! -f "${TEST_TMPDIR}/standalone_ok" ]]; then
  echo "ERROR: fake desktop launch flow did not complete" >&2
  exit 1
fi

if [[ ! -f "${TEST_TMPDIR}/standalone_doctor.txt" ]]; then
  echo "ERROR: standalone doctor output was not captured" >&2
  exit 1
fi

if ! grep -q "status: running" "${TEST_TMPDIR}/standalone_doctor.txt"; then
  echo "ERROR: standalone doctor output did not report a running backend" >&2
  cat "${TEST_TMPDIR}/standalone_doctor.txt" >&2
  exit 1
fi

echo "standalone desktop e2e passed"