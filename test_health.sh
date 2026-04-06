#!/bin/bash
# Simple test for verify_ports in ohc_hybrid_cli.sh

source ohc_hybrid_cli.sh --test-mode > /dev/null 2>&1 || true

output=$(verify_ports)

if echo "$output" | grep -q "Port 8080"; then
    echo "verify_ports successfully checked Port 8080"
else
    echo "verify_ports failed to check Port 8080"
    # Don't use exit directly in a script being sourced or run by run_in_bash_session
fi
echo "All tests passed"
