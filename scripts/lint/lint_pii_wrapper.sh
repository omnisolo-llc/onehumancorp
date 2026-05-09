#!/bin/bash
# Wrapper to run in root
cd "$(dirname "$0")/../.."
scripts/lint/lint_pii.sh
