#!/bin/bash
set -eo pipefail

echo "Scanning for stale branches..."
# Exclude main branch and the currently active branch
CURRENT_BRANCH=$(git branch --show-current)
git for-each-ref --format '%(refname:short)' refs/heads | \
  grep -v "^main$" | \
  grep -v "^${CURRENT_BRANCH}$" | \
  xargs -r git branch -D || true

echo "Cleanup complete."
