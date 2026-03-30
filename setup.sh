#!/bin/bash
set -e

function print_usage {
    echo "Usage: ./setup.sh <command>"
    echo "Commands:"
    echo "  test          Run all standard tests via Bazel."
    echo "  e2e           Run end-to-end integration tests."
    echo "  start-local   Start the local development environment."
    echo "  build         Build all targets."
    return 1
}

if [ $# -eq 0 ]; then
    print_usage
fi

COMMAND=$1
shift

# Patch rules_android cache before running bazel
bazelisk fetch @rules_android//... >/dev/null 2>&1 || true
OUTPUT_BASE=$(bazelisk info output_base)
find "$OUTPUT_BASE/external" -type d -name "*android*" | while read -r cache_dir; do
  find "$cache_dir" -name "*.bzl" -type f -exec grep -l "The CcInfo symbol has been removed" {} \; | while read -r bzl_file; do
    sed -i '1i load("@rules_cc\/\/cc\/common:cc_info.bzl", "CcInfo")' "$bzl_file"
  done
  find "$cache_dir" -name "helper.bzl" -type f | while read -r helper_file; do
    sed -i 's/load("@local_config_platform\/\/:constraints.bzl", "HOST_CONSTRAINTS")/# load("@local_config_platform\/\/:constraints.bzl", "HOST_CONSTRAINTS")/g' "$helper_file"
    sed -i 's/HOST_CONSTRAINTS/\[\]/g' "$helper_file"
  done
done


case "$COMMAND" in
    test)
        echo "Running Bazel tests..."
        bazelisk test //... "$@"
        ;;
    e2e)
        echo "Running E2E tests..."
        bazelisk test //:e2e "$@"
        ;;
    start-local)
        echo "Starting local environment..."
        bazelisk run //:deploy_dev "$@"
        ;;
    build)
        echo "Building all modules..."
        bazelisk build //... "$@"
        ;;
    *)
        echo "Unknown command: $COMMAND"
        print_usage
        ;;
esac
