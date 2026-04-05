#!/bin/bash
# OHC Hybrid Agentic OS - run_bazel wrapper

set -e

# Pass all arguments to bazelisk
bazelisk "$@"
