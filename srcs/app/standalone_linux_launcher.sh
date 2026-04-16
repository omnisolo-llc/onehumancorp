#!/usr/bin/env bash
set -euo pipefail

install_root="${OHC_DESKTOP_ROOT:-/opt/ohc-desktop}"
app_dir="${install_root}/srcs/app"
libexec_dir="${install_root}/libexec"
platform="${OHC_DESKTOP_PLATFORM:-linux}"

if ! command -v flutter >/dev/null 2>&1; then
  echo "Flutter SDK is required to launch the packaged OHC desktop sources." >&2
  echo "Install Flutter and rerun 'ohc-desktop'." >&2
  exit 1
fi

if [[ ! -d "${app_dir}" ]]; then
  echo "Packaged app sources not found at ${app_dir}" >&2
  exit 1
fi

if [[ ! -d "${libexec_dir}" ]]; then
  echo "Packaged runtime helpers not found at ${libexec_dir}" >&2
  exit 1
fi

export PATH="${libexec_dir}:${PATH}"

echo "--- Starting OHC Desktop App (Standalone Mode, Platform: ${platform}) ---"
cd "${app_dir}"
exec flutter run -d "${platform}" --dart-define=OHC_STANDALONE=true