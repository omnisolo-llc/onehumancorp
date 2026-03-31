#!/bin/bash

# Calculate SemVer bump based on git history and git tag parsing.

# Get the latest tag. If no tags exist, default to v0.0.0
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")

# Extract the version components from the tag (assuming format vX.Y.Z)
VERSION=${LATEST_TAG#v}

# Split the version string into MAJOR, MINOR, and PATCH components
IFS='.' read -r -a VERSION_PARTS <<< "$VERSION"

# Initialize variables to hold the current version components
MAJOR=${VERSION_PARTS[0]}
MINOR=${VERSION_PARTS[1]}
PATCH=${VERSION_PARTS[2]}

# Remove pre-release or build metadata from the patch component
PATCH=${PATCH%%-*}
PATCH=${PATCH%%+*}

# If the array was smaller than 3, fill missing parts with 0
if [ -z "$MAJOR" ]; then MAJOR=0; fi
if [ -z "$MINOR" ]; then MINOR=0; fi
if [ -z "$PATCH" ]; then PATCH=0; fi

# Default to bumping patch unless a higher bump is needed
BUMP_MAJOR=false
BUMP_MINOR=false
BUMP_PATCH=true

# Get the commits since the latest tag. If this is the first tag, get all commits.
if [ "$LATEST_TAG" = "v0.0.0" ]; then
  COMMITS=$(git log --format="%s%n%b" 2>/dev/null || echo "")
else
  COMMITS=$(git log ${LATEST_TAG}..HEAD --format="%s%n%b" 2>/dev/null || echo "")
fi

# Analyze commit messages to determine the bump level
while IFS= read -r LINE; do
  # Check for breaking changes
  if echo "$LINE" | grep -q "BREAKING CHANGE" || echo "$LINE" | grep -qE "^[a-zA-Z]+(\([^)]+\))?!:"; then
    BUMP_MAJOR=true
    break # Breaking change requires major bump, no need to check further
  # Check for features
  elif echo "$LINE" | grep -qE "^feat(\([^)]+\))?:"; then
    BUMP_MINOR=true
  fi
done <<< "$COMMITS"

# Calculate the new version components based on the required bump
if [ "$BUMP_MAJOR" = true ]; then
  NEW_MAJOR=$((MAJOR + 1))
  NEW_MINOR=0
  NEW_PATCH=0
elif [ "$BUMP_MINOR" = true ]; then
  NEW_MAJOR=$MAJOR
  NEW_MINOR=$((MINOR + 1))
  NEW_PATCH=0
else
  NEW_MAJOR=$MAJOR
  NEW_MINOR=$MINOR
  NEW_PATCH=$((PATCH + 1))
fi

# Output the new version with the 'v' prefix
echo "v${NEW_MAJOR}.${NEW_MINOR}.${NEW_PATCH}"
