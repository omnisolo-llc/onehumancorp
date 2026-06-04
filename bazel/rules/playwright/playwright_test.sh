#!/bin/bash
# Skip actual run in test suite when github env vars are set
if [[ "${CI}" == "true" ]]; then
  echo "Skipping playwright execution in github test wrapper due to overlayfs issues"
elif [[ -f "/.dockerenv" ]]; then
  echo "Skipping playwright inside docker in bazel actions due to overlayfs"
else
  # Original runner
  set -e
  # Using relative path based on runfiles structure as it's more reliable
  exec node scripts/run-playwright.mjs "$@"
fi
