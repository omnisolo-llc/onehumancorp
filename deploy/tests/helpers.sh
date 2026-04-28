#!/usr/bin/env bash
# E2E test helper functions
set -euo pipefail

# HTTP helper functions for testing
http_get() {
    local endpoint="$1"
    local expected_status="${2:-200}"
    local response
    local status
    
    response=$(curl -sf -w "\n%{http_code}" "http://127.0.0.1:18080${endpoint}" 2>/dev/null || echo "000")
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [[ "$status" != "$expected_status" ]]; then
        echo "FAIL: GET $endpoint - expected $expected_status, got $status"
        return 1
    fi
    echo "$body"
    return 0
}

http_post() {
    local endpoint="$1"
    local data="$2"
    local expected_status="${3:-200}"
    local response
    local status
    
    response=$(curl -sf -X POST -H "Content-Type: application/json" \
        -d "$data" -w "\n%{http_code}" \
        "http://127.0.0.1:18080${endpoint}" 2>/dev/null || echo "000")
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [[ "$status" != "$expected_status" ]]; then
        echo "FAIL: POST $endpoint - expected $expected_status, got $status"
        return 1
    fi
    echo "$body"
    return 0
}

http_put() {
    local endpoint="$1"
    local data="$2"
    local expected_status="${3:-200}"
    local response
    local status
    
    response=$(curl -sf -X PUT -H "Content-Type: application/json" \
        -d "$data" -w "\n%{http_code}" \
        "http://127.0.0.1:18080${endpoint}" 2>/dev/null || echo "000")
    status=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [[ "$status" != "$expected_status" ]]; then
        echo "FAIL: PUT $endpoint - expected $expected_status, got $status"
        return 1
    fi
    echo "$body"
    return 0
}

http_delete() {
    local endpoint="$1"
    local expected_status="${2:-200}"
    local response
    local status
    
    response=$(curl -sf -X DELETE -w "\n%{http_code}" \
        "http://127.0.0.1:18080${endpoint}" 2>/dev/null || echo "000")
    status=$(echo "$response" | tail -n1)
    
    if [[ "$status" != "$expected_status" ]]; then
        echo "FAIL: DELETE $endpoint - expected $expected_status, got $status"
        return 1
    fi
    return 0
}

# JSON validation helper
assert_json_field() {
    local json="$1"
    local field="$2"
    local expected="${3:-}"
    
    local value
    value=$(echo "$json" | jq -r "$field" 2>/dev/null || echo "")
    
    if [[ -n "$expected" ]] && [[ "$value" != "$expected" ]]; then
        echo "FAIL: Expected $field=$expected, got $value"
        return 1
    fi
    
    if [[ -z "$value" ]] || [[ "$value" == "null" ]]; then
        echo "FAIL: Field $field is missing or null"
        return 1
    fi
    
    return 0
}

# Wait for server to be ready
wait_for_server() {
    local max_attempts=60
    local attempt=0
    
    while [[ $attempt -lt $max_attempts ]]; do
        if curl -sf http://127.0.0.1:18080/healthz >/dev/null 2>&1; then
            echo "Server is ready"
            return 0
        fi
        attempt=$((attempt + 1))
        sleep 1
    done
    
    echo "Server failed to become ready after 60 seconds"
    return 1
}

# Test counter
TEST_PASSED=0
TEST_FAILED=0

run_test() {
    local test_name="$1"
    local test_func="$2"
    
    echo -n "Testing: $test_name ... "
    if $test_func; then
        echo "PASS"
        ((TEST_PASSED++))
        return 0
    else
        echo "FAIL"
        ((TEST_FAILED++))
        return 1
    fi
}

print_summary() {
    local total=$((TEST_PASSED + TEST_FAILED))
    echo ""
    echo "================================"
    echo "Test Summary:"
    echo "  Passed: $TEST_PASSED"
    echo "  Failed: $TEST_FAILED"
    echo "  Total:  $total"
    echo "================================"
    
    if [[ $TEST_FAILED -gt 0 ]]; then
        return 1
    fi
    return 0
}
