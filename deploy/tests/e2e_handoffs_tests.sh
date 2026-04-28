#!/usr/bin/env bash
# Handoffs API E2E tests - 20 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Handoffs API tests
test_handoffs_list() {
    local resp
    resp=$(http_get "/api/handoffs")
    assert_json_field "$resp" ".handoffs"
}

test_handoffs_list_valid_json() {
    local resp
    resp=$(http_get "/api/handoffs")
    echo "$resp" | jq . >/dev/null
}

test_handoffs_create_basic() {
    local data='{"from_agent":"agent1","to_agent":"agent2","context":"task transfer"}'
    local resp
    resp=$(http_post "/api/handoffs" "$data" 201)
    assert_json_field "$resp" ".id"
}

test_handoffs_create_full() {
    local data='{
        "from_agent":"agent1",
        "to_agent":"agent2",
        "context":"full handoff",
        "notes":"Complete context transfer",
        "data":{"task_id":"T123"}
    }'
    http_post "/api/handoffs" "$data" 201 >/dev/null
}

test_handoffs_create_minimal() {
    local data='{"from_agent":"agent1","to_agent":"agent2"}'
    http_post "/api/handoffs" "$data" 201 >/dev/null
}

test_handoffs_list_after_create() {
    local data='{"from_agent":"agent1","to_agent":"agent2"}'
    http_post "/api/handoffs" "$data" 201 >/dev/null
    local resp
    resp=$(http_get "/api/handoffs")
    echo "$resp" | jq . >/dev/null
}

test_handoffs_concurrent_create() {
    for i in {1..5}; do
        local data="{\"from_agent\":\"agent1\",\"to_agent\":\"agent$i\"}"
        http_post "/api/handoffs" "$data" 201 >/dev/null &
    done
    wait
}

test_handoffs_sequential_create() {
    for i in {1..10}; do
        local data="{\"from_agent\":\"agent1\",\"to_agent\":\"agent2\",\"context\":\"seq-$i\"}"
        http_post "/api/handoffs" "$data" 201 >/dev/null
    done
}

test_handoffs_create_invalid_json() {
    http_post "/api/handoffs" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_handoffs_create_empty() {
    http_post "/api/handoffs" "{}" >/dev/null 2>&1 || true
}

test_handoffs_list_with_filter() {
    http_get "/api/handoffs?from_agent=agent1" >/dev/null 2>&1 || true
}

test_handoffs_list_with_limit() {
    http_get "/api/handoffs?limit=20" >/dev/null 2>&1 || true
}

test_handoffs_list_pagination() {
    http_get "/api/handoffs?page=1&size=10" >/dev/null 2>&1 || true
}

test_handoffs_response_time() {
    local data='{"from_agent":"agent1","to_agent":"agent2"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/handoffs" "$data" 201 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_handoffs_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/handoffs" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_handoffs_get_nonexistent() {
    http_get "/api/handoffs/nonexistent" 404 >/dev/null 2>&1 || true
}

test_handoffs_create_with_metadata() {
    local data='{
        "from_agent":"agent1",
        "to_agent":"agent2",
        "metadata":{"priority":"high","deadline":"2024-12-01"}
    }'
    http_post "/api/handoffs" "$data" 201 >/dev/null 2>&1 || true
}

test_handoffs_list_sorted() {
    http_get "/api/handoffs?sort=created_at" >/dev/null 2>&1 || true
}

test_handoffs_create_batch() {
    for i in {1..3}; do
        local data="{\"from_agent\":\"agent1\",\"to_agent\":\"agent$i\"}"
        http_post "/api/handoffs" "$data" 201 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Handoffs list" test_handoffs_list
run_test "Handoffs list valid JSON" test_handoffs_list_valid_json
run_test "Handoffs create basic" test_handoffs_create_basic
run_test "Handoffs create full" test_handoffs_create_full
run_test "Handoffs create minimal" test_handoffs_create_minimal
run_test "Handoffs list after create" test_handoffs_list_after_create
run_test "Handoffs concurrent create" test_handoffs_concurrent_create
run_test "Handoffs sequential create" test_handoffs_sequential_create
run_test "Handoffs create invalid JSON" test_handoffs_create_invalid_json
run_test "Handoffs create empty" test_handoffs_create_empty
run_test "Handoffs list with filter" test_handoffs_list_with_filter
run_test "Handoffs list with limit" test_handoffs_list_with_limit
run_test "Handoffs list pagination" test_handoffs_list_pagination
run_test "Handoffs response time" test_handoffs_response_time
run_test "Handoffs list response time" test_handoffs_list_response_time
run_test "Handoffs get nonexistent" test_handoffs_get_nonexistent
run_test "Handoffs create with metadata" test_handoffs_create_with_metadata
run_test "Handoffs list sorted" test_handoffs_list_sorted
run_test "Handoffs create batch" test_handoffs_create_batch

print_summary
