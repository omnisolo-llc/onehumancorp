#!/usr/bin/env bash
# Serve the Bazel-built Flutter web bundle on a local HTTP port.
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
script_real_dir="$(cd -- "$(dirname -- "$(realpath "${BASH_SOURCE[0]}")")" && pwd)"
runfiles_base="${RUNFILES_DIR:-${BASH_SOURCE[0]}.runfiles}"
workspace_root="${BUILD_WORKSPACE_DIRECTORY:-$(cd -- "${script_real_dir}/../../.." && pwd)}"

port="${1:-8081}"

is_complete_web_bundle() {
	local candidate="$1"

	[[ -d "$candidate" ]] || return 1
	[[ -f "$candidate/assets/FontManifest.json" ]] || return 1
	[[ -f "$candidate/assets/fonts/MaterialIcons-Regular.otf" ]] || return 1
	grep -q 'MaterialIcons' "$candidate/assets/FontManifest.json"
}

find_runfiles_root() {
	local candidate

	for candidate in "${runfiles_base}" "${RUNFILES_DIR:-}" "${TEST_SRCDIR:-}"; do
		if [[ -n "${candidate}" && -d "${candidate}" ]]; then
			printf '%s\n' "${candidate}"
			return 0
		fi
	done

	return 1
}

find_web_artifacts() {
	local root="${1}"
	shift
	local candidate

	for candidate in "$@"; do
		if [[ -d "${root}/${candidate}" ]]; then
			printf '%s\n' "${root}/${candidate}"
			return 0
		fi
	done

	return 1
}

find_ohc_binary() {
	local root="${1}"
	shift
	local candidate

	for candidate in "$@"; do
		if [[ -f "${root}/${candidate}" ]]; then
			printf '%s\n' "${root}/${candidate}"
			return 0
		fi
	done

	return 1
}

runfiles_root="$(find_runfiles_root || true)"
web_artifacts=""

workspace_web_bundle="${workspace_root}/src/app/build/web"
if is_complete_web_bundle "${workspace_web_bundle}"; then
	web_artifacts="${workspace_web_bundle}"
fi

if [[ -z "${web_artifacts}" && -n "${runfiles_root}" ]]; then
	web_artifacts="$(find_web_artifacts "${runfiles_root}" \
		"${TEST_WORKSPACE:-mono}/src/app/app_web.web_build_artifacts" \
		"${TEST_WORKSPACE:-mono}/src/app/app_web_build_artifacts" \
		"_main/src/app/app_web.web_build_artifacts" \
		"_main/src/app/app_web_build_artifacts" \
		"__main__/src/app/app_web.web_build_artifacts" \
		"__main__/src/app/app_web_build_artifacts" \
		"_main/src/app/app.web_build_artifacts" \
		"_main/src/app/app_build_artifacts" \
		"__main__/src/app/app.web_build_artifacts" \
		"__main__/src/app/app_build_artifacts" \
		"mono/src/app/app.web_build_artifacts" \
		"mono/src/app/app_build_artifacts" || true)"
fi

if [[ -z "${web_artifacts}" ]]; then
	echo "ERROR: could not locate Bazel-built Flutter web artifacts in runfiles." >&2
	echo "Ensure this helper is launched via 'bazelisk run //src/app:start'." >&2
	exit 1
fi

# Ensure manifest.json is present (sometimes missed by build rules)
if [[ ! -f "${web_artifacts}/manifest.json" ]]; then
	echo "Warning: manifest.json missing in artifacts. Attempting to copy from source..."
	cp "${workspace_root}/src/app/web/manifest.json" "${web_artifacts}/manifest.json" 2>/dev/null || true
fi

ohc_binary=""
if [[ -n "${runfiles_root}" ]]; then
	ohc_binary="$(find_ohc_binary "${runfiles_root}" \
		"${TEST_WORKSPACE:-mono}/src/server/ohc_/ohc" \
		"${TEST_WORKSPACE:-mono}/src/server/ohc" \
		"_main/src/server/ohc_/ohc" \
		"_main/src/server/ohc" \
		"__main__/src/server/ohc_/ohc" \
		"__main__/src/server/ohc" \
		"mono/src/server/ohc_/ohc" \
		"mono/src/server/ohc" || true)"
fi

if [[ -z "${ohc_binary}" ]]; then
	echo "ERROR: could not locate ohc server binary." >&2
	exit 1
fi

state_dir=$(mktemp -d -t ohc-start-XXXXXX)
trap "rm -rf ${state_dir}" EXIT

echo "Starting One Human Corp FULL SERVICE"
echo "Web Artifacts: ${web_artifacts}"
echo "Server Binary: ${ohc_binary}"
echo "State Directory: ${state_dir}"
echo "URL: http://127.0.0.1:${port}"

export OHC_STANDALONE=true
export OHC_HEADLESS=false
export OHC_SERVE_UI=true
export FRONTEND_STATIC_DIR="${web_artifacts}"
export PORT="${port}"
export STATE_DIR="${state_dir}"
export OHC_RUNTIME_DIR="${state_dir}"
export DATABASE_URL="sqlite://${state_dir}/ohc_state.db"

exec "${ohc_binary}"
