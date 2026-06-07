#!/bin/bash
# Script to rebuild and load all Docker images locally via Bazel.

set -e

# --- Runfiles setup ---
# Copy-pasted from the Bazel Bash runfiles library v2.
# https://github.com/bazelbuild/bazel/blob/master/tools/bash/runfiles/runfiles.bash
if [[ ! -d "${RUNFILES_DIR:-/dev/null}" && ! -f "${RUNFILES_MANIFEST_FILE:-/dev/null}" ]]; then
  if [[ -f "$0.runfiles_manifest" ]]; then
    export RUNFILES_MANIFEST_FILE="$0.runfiles_manifest"
  elif [[ -f "$0.runfiles/MANIFEST" ]]; then
    export RUNFILES_MANIFEST_FILE="$0.runfiles/MANIFEST"
  elif [[ -f "$0.runfiles/bazel_tools/tools/bash/runfiles/runfiles.bash" ]]; then
    export RUNFILES_DIR="$0.runfiles"
  fi
fi
if [[ -f "${RUNFILES_DIR:-/dev/null}/bazel_tools/tools/bash/runfiles/runfiles.bash" ]]; then
  source "${RUNFILES_DIR}/bazel_tools/tools/bash/runfiles/runfiles.bash"
elif [[ -f "${RUNFILES_MANIFEST_FILE:-/dev/null}" ]]; then
  source "$(grep -m1 "^bazel_tools/tools/bash/runfiles/runfiles.bash "             "$RUNFILES_MANIFEST_FILE" | cut -d ' ' -f 2-)"
else
  echo >&2 "ERROR: cannot find @bazel_tools//tools/bash/runfiles:runfiles.bash"
  exit 1
fi
# --- end runfiles setup ---


echo "--- Loading Bazel-built images ---"

load_image() {
  local IMAGE_NAME=$1
  if [[ -n "$RUNFILES_DIR" ]]; then
    # Find load script inside runfiles
    LOAD_SCRIPT=$(find "$RUNFILES_DIR" -name "${IMAGE_NAME}.sh" | head -n 1)
    if [[ -n "$LOAD_SCRIPT" ]]; then
      echo "Running $LOAD_SCRIPT"
      (
        export RUNFILES_DIR="${RUNFILES_DIR}"
        export RUNFILES_MANIFEST_FILE="${RUNFILES_MANIFEST_FILE:-}"
        "$LOAD_SCRIPT"
      )
    else
      echo "Error: ${IMAGE_NAME}.sh not found in runfiles."
      exit 1
    fi
  else
    # Try finding it relative to current path if not under bazel run
    LOAD_SCRIPT=$(find . -name "${IMAGE_NAME}.sh" | head -n 1)
    if [[ -n "$LOAD_SCRIPT" ]]; then
      echo "Running $LOAD_SCRIPT"
      bash "$LOAD_SCRIPT"
    else
      echo "Error: ${IMAGE_NAME}.sh not found. Run this via bazel run //deploy:load_all_images"
      exit 1
    fi
  fi
}

load_image "server_load"
load_image "mono_core_load"
load_image "agent_load"
load_image "default_agent_load"

echo "All images loaded successfully!"
