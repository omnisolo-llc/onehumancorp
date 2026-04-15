#!/usr/bin/env bash
set -euo pipefail

install_root="${OHC_DESKTOP_ROOT:-/opt/ohc-desktop}"
libexec_dir="${install_root}/libexec"
bundle_root=""

if [[ ! -d "${libexec_dir}" ]]; then
  echo "Packaged runtime helpers not found at ${libexec_dir}" >&2
  exit 1
fi

for candidate in \
  "${install_root}" \
  "${install_root}/standalone_app.linux_build_artifacts" \
  "${install_root}/app.linux_build_artifacts" \
  "${install_root}/bundle"; do
  if [[ -x "${candidate}/ohc_app" ]]; then
    bundle_root="${candidate}"
    break
  fi
done

if [[ -z "${bundle_root}" ]]; then
  echo "Packaged OHC desktop bundle not found under ${install_root}" >&2
  exit 1
fi

export PATH="${libexec_dir}:${PATH}"

echo "--- Starting OHC Desktop App (Standalone Bundle: ${bundle_root}) ---"
exec "${bundle_root}/ohc_app" "$@"
