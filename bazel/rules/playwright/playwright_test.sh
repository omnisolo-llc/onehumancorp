#!/bin/bash
set -euo pipefail

if [[ -n "${TEST_SRCDIR:-}" && -n "${TEST_WORKSPACE:-}" ]]; then
    ROOT="${TEST_SRCDIR}/${TEST_WORKSPACE}"
    if [[ ! -d "${ROOT}/scripts" ]]; then
        ROOT="${TEST_SRCDIR}/mono"
    fi
    cd "${ROOT}"
fi

node scripts/run-playwright.mjs "$@"
