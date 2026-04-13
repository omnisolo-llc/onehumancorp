#!/bin/bash
# OHC Hybrid Agentic OS - test wrapper

set -e

# Run all tests using bazelisk
~/go/bin/bazelisk test //... "$@"
