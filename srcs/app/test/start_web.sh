#!/usr/bin/env bash
# Serve the Bazel-built Flutter web bundle on a local HTTP port.
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
runfiles_base="${RUNFILES_DIR:-${BASH_SOURCE[0]}.runfiles}"

port="${1:-8081}"

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

runfiles_root="$(find_runfiles_root || true)"
web_artifacts=""

if [[ -n "${runfiles_root}" ]]; then
	web_artifacts="$(find_web_artifacts "${runfiles_root}" \
		"${TEST_WORKSPACE:-mono}/srcs/app/app_web.web_build_artifacts" \
		"${TEST_WORKSPACE:-mono}/srcs/app/app_web_build_artifacts" \
		"_main/srcs/app/app_web.web_build_artifacts" \
		"_main/srcs/app/app_web_build_artifacts" \
		"__main__/srcs/app/app_web.web_build_artifacts" \
		"__main__/srcs/app/app_web_build_artifacts" || true)"
fi

if [[ -z "${web_artifacts}" ]]; then
	echo "ERROR: could not locate Bazel-built Flutter web artifacts in runfiles." >&2
	echo "Ensure this helper is launched via 'bazelisk run //srcs/app:start'." >&2
	exit 1
fi

echo "Serving Bazel-built Flutter app from ${web_artifacts}"
echo "URL: http://127.0.0.1:${port}"
exec python3 -m http.server "${port}" --directory "${web_artifacts}"
