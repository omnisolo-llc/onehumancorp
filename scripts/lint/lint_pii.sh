#!/usr/bin/env bash

# Find the real path to the workspace root if we are in bazel runfiles
if [ -n "${TEST_WORKSPACE:-}" ]; then
    cd "${TEST_SRCDIR}/${TEST_WORKSPACE}"
fi

function run_lint() {
    local LEAKS_FOUND=0

    while IFS= read -r file; do
        LEAKS=$(grep -n -E "log\.(Printf|Println|Fatal|Fatalf|Print)|fmt\.(Printf|Println|Print)" "$file" | grep -i -E "tenant_id|payload|email|password|credit_card" | grep -v "\[REDACTED\]" || true)

        if [ ! -z "$LEAKS" ]; then
            echo "Found potential PII leakage in $file:"
            echo "$LEAKS"
            LEAKS_FOUND=1
        fi
    done < <(find srcs/server -type f -name "*.go")

    if [ "$LEAKS_FOUND" -eq 1 ]; then
        echo "PII LINT FAILED: Please replace sensitive terms with generic ones like 'user data' or 'account id', or use '[REDACTED]'."
        return 1
    fi

    echo "PII LINT PASSED."
    return 0
}

run_lint
