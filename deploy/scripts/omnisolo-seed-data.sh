#!/usr/bin/env bash
# Kept as a fail-closed migration message for old operator automation.
set -euo pipefail
printf '%s\n' 'Demo seeding of installed applications has been removed.' >&2
printf '%s\n' 'Use make test-e2e to build and test the real app with a disposable isolated fixture database.' >&2
exit 2
