#!/bin/bash
# OHC Hybrid Agentic OS - test wrapper

set -e

# Run all tests using bazelisk
bazelisk test //... "$@"
