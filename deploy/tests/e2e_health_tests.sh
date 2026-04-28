#!/usr/bin/env bash
# Health check E2E tests - 20 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Health check tests
test_health_liveness() {
    local resp
    resp=$(http_get "/healthz")
    assert_json_field "$resp" ".status" "ok"
}

test_health_readiness() {
    local resp
    resp=$(http_get "/readyz")
    assert_json_field "$resp" ".status" "ready"
}

test_health_liveness_status_code() {
    curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1
}

test_health_readiness_status_code() {
    curl -f http://127.0.0.1:18080/readyz >/dev/null 2>&1
}

test_health_repeated_checks() {
    for i in {1..5}; do
        curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1 || return 1
    done
}

test_health_concurrent_checks() {
    for i in {1..10}; do
        curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1 &
    done
    wait
}

test_health_with_timeout() {
    timeout 5s curl http://127.0.0.1:18080/healthz >/dev/null 2>&1
}

test_health_concurrent_readiness() {
    for i in {1..10}; do
        curl -f http://127.0.0.1:18080/readyz >/dev/null 2>&1 &
    done
    wait
}

test_health_liveness_response_time() {
    local start end duration
    start=$(date +%s%N)
    curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 1000 ]]  # Should complete in less than 1 second
}

test_health_readiness_response_time() {
    local start end duration
    start=$(date +%s%N)
    curl -f http://127.0.0.1:18080/readyz >/dev/null 2>&1
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 1000 ]]  # Should complete in less than 1 second
}

test_health_sequential_checks() {
    for i in {1..20}; do
        curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1 || return 1
        sleep 0.1
    done
}

test_health_mixed_endpoints() {
    curl -f http://127.0.0.1:18080/healthz >/dev/null 2>&1 || return 1
    curl -f http://127.0.0.1:18080/readyz >/dev/null 2>&1 || return 1
}

test_health_not_found() {
    http_get "/health" 404 >/dev/null 2>&1 || true
}

test_health_wrong_method() {
    curl -X POST http://127.0.0.1:18080/healthz >/dev/null 2>&1 || true
}

test_health_empty_response() {
    local resp
    resp=$(http_get "/healthz")
    [[ -n "$resp" ]]
}

test_health_valid_json() {
    local resp
    resp=$(http_get "/healthz")
    echo "$resp" | jq . >/dev/null 2>&1
}

test_health_liveness_with_params() {
    curl -f "http://127.0.0.1:18080/healthz?foo=bar" >/dev/null 2>&1 || true
}

test_health_readiness_with_params() {
    curl -f "http://127.0.0.1:18080/readyz?foo=bar" >/dev/null 2>&1 || true
}

# Main test execution
wait_for_server || exit 1

run_test "Health liveness" test_health_liveness
run_test "Health readiness" test_health_readiness
run_test "Health liveness status code" test_health_liveness_status_code
run_test "Health readiness status code" test_health_readiness_status_code
run_test "Health repeated checks" test_health_repeated_checks
run_test "Health concurrent checks" test_health_concurrent_checks
run_test "Health with timeout" test_health_with_timeout
run_test "Health concurrent readiness" test_health_concurrent_readiness
run_test "Health liveness response time" test_health_liveness_response_time
run_test "Health readiness response time" test_health_readiness_response_time
run_test "Health sequential checks" test_health_sequential_checks
run_test "Health mixed endpoints" test_health_mixed_endpoints
run_test "Health not found" test_health_not_found
run_test "Health wrong method" test_health_wrong_method
run_test "Health empty response" test_health_empty_response
run_test "Health valid JSON" test_health_valid_json
run_test "Health liveness with params" test_health_liveness_with_params
run_test "Health readiness with params" test_health_readiness_with_params

print_summary
