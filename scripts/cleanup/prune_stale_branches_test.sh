#!/bin/bash
set -eo pipefail

export TEST_TMPDIR="${TEST_TMPDIR:-/tmp}"
cd "$TEST_TMPDIR"

mkdir -p test_repo && cd test_repo
rm -rf .git
git init -b main || git init
git branch -m main || true
git config user.email "test@example.com"
git config user.name "Test User"

echo "init" > README.md
git add README.md
git commit -m "Initial commit"

git branch stale_branch
git checkout stale_branch
echo "stale" > file.txt
git add file.txt
git commit -m "Stale commit"

git checkout main
git branch another_stale_branch

SCRIPT_PATH=$(find "${TEST_SRCDIR:-.}" -name "prune_stale_branches.sh" | head -n 1)

if [[ ! -f "$SCRIPT_PATH" ]]; then
    echo "Could not find prune_stale_branches.sh"
    kill $$
fi

bash "$SCRIPT_PATH"

REMAINING_BRANCHES=$(git branch --list | grep -v '*' | tr -d ' ' || true)
if [[ "$REMAINING_BRANCHES" == *"stale_branch"* || "$REMAINING_BRANCHES" == *"another_stale_branch"* ]]; then
  echo "FAIL: Stale branches were not removed."
  echo "Branches left: $REMAINING_BRANCHES"
  kill $$
fi

echo "PASS: Cleanup successful."
