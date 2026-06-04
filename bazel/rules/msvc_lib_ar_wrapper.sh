#!/usr/bin/env bash
set -euo pipefail

out=
args=()

for arg in "$@"; do
    case "$arg" in
        -out:* | -OUT:* | /out:* | /OUT:*)
            out="${arg#*:}"
            ;;
        -nologo | /nologo)
            ;;
        *)
            args+=("$arg")
            ;;
    esac
done

if [[ -z "${out}" ]]; then
    exec "${AR:-ar}" "$@"
fi

ar_tool="${BAZEL_MSVC_AR:-${AR:-}}"
if [[ -z "${ar_tool}" ]]; then
    echo "msvc_lib_ar_wrapper.sh: AR or BAZEL_MSVC_AR must be set" >&2
    exit 1
fi

exec "${ar_tool}" crs "${out}" "${args[@]}"
