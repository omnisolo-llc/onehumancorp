#!/bin/bash

# A simple test to verify calculate_semver.sh

# We need a clean git repo to test this properly, so we create a temporary one
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

cp $(dirname "$0")/calculate_semver.sh "$TMP_DIR/calculate_semver.sh"
cd "$TMP_DIR" || exit 1

# Initialize repo
git init > /dev/null
git config user.email "test@example.com"
git config user.name "Test User"

# Initial commit
echo "init" > init.txt
git add init.txt
git commit -m "chore: initial commit" > /dev/null

# Test default behavior (no tags) -> should be v0.0.1 since we have 1 chore commit
RESULT=$(./calculate_semver.sh)
if [ "$RESULT" != "v0.0.1" ]; then
    echo "Expected v0.0.1, got $RESULT"
    exit 1
fi

# Add a tag
git tag v1.0.0

# Add a feat commit
echo "feat" > feat.txt
git add feat.txt
git commit -m "feat: new feature" > /dev/null

RESULT=$(./calculate_semver.sh)
if [ "$RESULT" != "v1.1.0" ]; then
    echo "Expected v1.1.0, got $RESULT"
    exit 1
fi

# Add a breaking change
echo "breaking" > breaking.txt
git add breaking.txt
git commit -m "feat!: breaking change" > /dev/null

RESULT=$(./calculate_semver.sh)
if [ "$RESULT" != "v2.0.0" ]; then
    echo "Expected v2.0.0, got $RESULT"
    exit 1
fi

# Add a fix commit
git tag v2.0.0
echo "fix" > fix.txt
git add fix.txt
git commit -m "fix: bug fix" > /dev/null

RESULT=$(./calculate_semver.sh)
if [ "$RESULT" != "v2.0.1" ]; then
    echo "Expected v2.0.1, got $RESULT"
    exit 1
fi

# Test pre-release tag parsing
git tag v2.0.1-rc.1
echo "chore" > chore.txt
git add chore.txt
git commit -m "chore: another commit" > /dev/null

RESULT=$(./calculate_semver.sh)
if [ "$RESULT" != "v2.0.2" ]; then
    echo "Expected v2.0.2, got $RESULT"
    exit 1
fi

echo "All tests passed!"
exit 0
