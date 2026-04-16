#!/bin/bash
set -e

export PATH=$PATH:/home/jules/go/bin:/usr/local/go/bin:/app/bin:/home/jules/.local/bin

echo "Running Pre-audit"
python3 srcs/tests/day_one_audit_test.py || true

echo "Simulating Feature Changes..."
# e.g., Update variables, simplify wizards

echo "Running Post-audit"
python3 srcs/tests/day_one_audit_test.py || true
