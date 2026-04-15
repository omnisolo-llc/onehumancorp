#!/usr/bin/env bash
set -euo pipefail

format="${1:-}"
RUNFILES="${RUNFILES_DIR:-${TEST_SRCDIR:-$PWD}}"
TMPDIR="${TEST_TMPDIR:-/tmp/ohc_package_test}"

required_paths=(
  "opt/ohc-desktop/libexec/ohc"
  "opt/ohc-desktop/libexec/ohc-server"
  "opt/ohc-desktop/libexec/ohc-desktop"
  "usr/bin/ohc"
  "usr/bin/ohc-desktop"
)

find_single_file() {
  local pattern="$1"

  find "${RUNFILES}" \( -type f -o -type l \) -name "${pattern}" | head -n 1
}

assert_tar_contains() {
  local archive="$1"
  local listing="$2"
  local verbose_listing="$3"
  local path
  local launcher_path
  local launcher_contents

  tar -tf "${archive}" | sed 's#^\./##' >"${listing}"
  tar -tvf "${archive}" >"${verbose_listing}"

  for path in "${required_paths[@]}"; do
    if ! grep -Fxq "${path}" "${listing}"; then
      echo "ERROR: ${archive} is missing ${path}" >&2
      exit 1
    fi
  done

  if ! grep -Eq '^opt/ohc-desktop(/[^/]+)?/ohc_app$' "${listing}"; then
    echo "ERROR: ${archive} is missing the packaged ohc_app executable" >&2
    exit 1
  fi

  if ! grep -Eq '^opt/ohc-desktop(/[^/]+)?/data/' "${listing}"; then
    echo "ERROR: ${archive} is missing packaged Flutter data assets" >&2
    exit 1
  fi

  if ! grep -Eq '^opt/ohc-desktop(/[^/]+)?/lib/' "${listing}"; then
    echo "ERROR: ${archive} is missing packaged Flutter shared libraries" >&2
    exit 1
  fi

  if ! grep -Eq '(\./)?usr/bin/ohc -> /opt/ohc-desktop/libexec/ohc$' "${verbose_listing}"; then
    echo "ERROR: ${archive} is missing the /usr/bin/ohc symlink" >&2
    exit 1
  fi

  if ! grep -Eq '(\./)?usr/bin/ohc-desktop -> /opt/ohc-desktop/libexec/ohc-desktop$' "${verbose_listing}"; then
    echo "ERROR: ${archive} is missing the /usr/bin/ohc-desktop symlink" >&2
    exit 1
  fi

  launcher_path="$(grep -E '^opt/ohc-desktop/libexec/ohc-desktop$' "${listing}" | head -n 1)"
  launcher_contents="$(tar -xOf "${archive}" "${launcher_path}")"
  if grep -q "flutter run" <<<"${launcher_contents}"; then
    echo "ERROR: packaged launcher still shells out to flutter run" >&2
    exit 1
  fi
  if ! grep -q 'ohc_app' <<<"${launcher_contents}"; then
    echo "ERROR: packaged launcher does not invoke the bundled executable" >&2
    exit 1
  fi
}

assert_rpm_contains_strings() {
  local rpm_path="$1"
  python3 - "$rpm_path" <<'PY'
import sys

rpm_path = sys.argv[1]
required = [
    b"opt/ohc-desktop/libexec/ohc",
    b"opt/ohc-desktop/libexec/ohc-server",
    b"opt/ohc-desktop/libexec/ohc-desktop",
    b"usr/bin/ohc",
    b"usr/bin/ohc-desktop",
    b"ohc_app",
]

data = open(rpm_path, "rb").read()
missing = [entry.decode("utf-8") for entry in required if entry not in data]
if missing:
    raise SystemExit("rpm metadata is missing expected paths: " + ", ".join(missing))
PY
}

case "${format}" in
  deb)
    deb_path="$(find_single_file '*.deb')"
    if [[ -z "${deb_path}" ]]; then
      echo "ERROR: deb package not found in runfiles" >&2
      exit 1
    fi

    data_member="$(ar t "${deb_path}" | grep '^data.tar' | head -n 1)"
    if [[ -z "${data_member}" ]]; then
      echo "ERROR: deb package ${deb_path} does not contain a data.tar member" >&2
      exit 1
    fi

    data_archive="${TMPDIR}/${data_member}"
    ar p "${deb_path}" "${data_member}" >"${data_archive}"
    assert_tar_contains "${data_archive}" "${TMPDIR}/deb.list" "${TMPDIR}/deb.verbose.list"
    ;;
  rpm)
    rpm_path="$(find_single_file '*.rpm')"
    tar_path="$(find_single_file '*.tar')"
    if [[ -z "${rpm_path}" ]]; then
      echo "ERROR: rpm package not found in runfiles" >&2
      exit 1
    fi
    if [[ -z "${tar_path}" ]]; then
      echo "ERROR: package rootfs tar not found in runfiles" >&2
      exit 1
    fi

    assert_tar_contains "${tar_path}" "${TMPDIR}/rpm.list" "${TMPDIR}/rpm.verbose.list"
    assert_rpm_contains_strings "${rpm_path}"
    ;;
  *)
    echo "usage: $0 <deb|rpm>" >&2
    exit 1
    ;;
esac

echo "linux package structure test passed for ${format}"
