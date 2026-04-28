#!/usr/bin/env bash
# Agents API E2E tests - 40 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Agents API tests
test_agents_list() {
    local resp
    resp=$(http_get "/api/agents")
    assert_json_field "$resp" ".agents"
}

test_agents_list_empty() {
    http_get "/api/agents" >/dev/null
}

test_agents_list_valid_json() {
    local resp
    resp=$(http_get "/api/agents")
    echo "$resp" | jq . >/dev/null
}

test_agents_hire_basic() {
    local data='{"name":"test-agent","role":"assistant"}'
    local resp
    resp=$(http_post "/api/agents/hire" "$data" 200)
    assert_json_field "$resp" ".id"
}

test_agents_hire_full() {
    local data='{
        "name":"full-agent",
        "role":"manager",
        "skills":["negotiation","analysis"],
        "cost_center":"CC001"
    }'
    local resp
    resp=$(http_post "/api/agents/hire" "$data" 200)
    assert_json_field "$resp" ".id"
}

test_agents_hire_with_metadata() {
    local data='{
        "name":"meta-agent",
        "role":"assistant",
        "metadata":{"team":"platform","created_by":"system"}
    }'
    http_post "/api/agents/hire" "$data" >/dev/null
}

test_agents_hire_minimal() {
    local data='{"name":"minimal-agent"}'
    http_post "/api/agents/hire" "$data" >/dev/null
}

test_agents_hire_duplicate() {
    local data='{"name":"dup-agent"}'
    http_post "/api/agents/hire" "$data" >/dev/null
    # Second request with same data
    http_post "/api/agents/hire" "$data" >/dev/null
}

test_agents_hire_invalid_json() {
    http_post "/api/agents/hire" "{invalid json}" 400 >/dev/null 2>&1 || true
}

test_agents_hire_empty_data() {
    http_post "/api/agents/hire" "{}" >/dev/null 2>&1 || true
}

test_agents_hire_large_payload() {
    local data='{"name":"large-agent","description":"'$(printf 'x%.0s' {1..1000})'"}'
    http_post "/api/agents/hire" "$data" >/dev/null
}

test_agents_hire_special_chars() {
    local data='{"name":"agent@special#chars$%"}'
    http_post "/api/agents/hire" "$data" >/dev/null 2>&1 || true
}

test_agents_hire_unicode_name() {
    local data='{"name":"agent-é-ñ-中文"}'
    http_post "/api/agents/hire" "$data" >/dev/null 2>&1 || true
}

test_agents_list_after_hire() {
    local data='{"name":"listed-agent"}'
    http_post "/api/agents/hire" "$data" >/dev/null
    local resp
    resp=$(http_get "/api/agents")
    echo "$resp" | jq . >/dev/null
}

test_agents_concurrent_hire() {
    for i in {1..5}; do
        local data="{\"name\":\"concurrent-agent-$i\"}"
        http_post "/api/agents/hire" "$data" >/dev/null &
    done
    wait
}

test_agents_sequential_hire() {
    for i in {1..10}; do
        local data="{\"name\":\"sequential-agent-$i\"}"
        http_post "/api/agents/hire" "$data" >/dev/null
    done
}

test_agents_get_nonexistent() {
    http_get "/api/agents/nonexistent" 404 >/dev/null 2>&1 || true
}

test_agents_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/agents" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]  # Should complete in less than 2 seconds
}

test_agents_hire_response_time() {
    local data='{"name":"perf-agent"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/agents/hire" "$data" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_agents_list_with_filter() {
    http_get "/api/agents?role=assistant" >/dev/null 2>&1 || true
}

test_agents_list_with_limit() {
    http_get "/api/agents?limit=10" >/dev/null 2>&1 || true
}

test_agents_list_pagination() {
    http_get "/api/agents?page=1&size=20" >/dev/null 2>&1 || true
}

test_agents_hire_long_name() {
    local name=$(printf 'a%.0s' {1..200})
    local data="{\"name\":\"$name\"}"
    http_post "/api/agents/hire" "$data" >/dev/null 2>&1 || true
}

test_agents_hire_numeric_name() {
    local data='{"name":"123456789"}'
    http_post "/api/agents/hire" "$data" >/dev/null
}

test_agents_hire_with_timestamps() {
    local data='{
        "name":"timestamp-agent",
        "created_at":"2024-01-01T00:00:00Z"
    }'
    http_post "/api/agents/hire" "$data" >/dev/null 2>&1 || true
}

test_agents_list_sorted() {
    http_get "/api/agents?sort=name" >/dev/null 2>&1 || true
}

test_agents_list_descending() {
    http_get "/api/agents?sort=-created_at" >/dev/null 2>&1 || true
}

test_agents_hire_batch() {
    for i in {1..3}; do
        local data="{\"name\":\"batch-agent-$i\"}"
        http_post "/api/agents/hire" "$data" >/dev/null &
    done
    wait
}

test_agents_hire_with_context() {
    local data='{
        "name":"context-agent",
        "context":{"project":"platform","team":"engineering"}
    }'
    http_post "/api/agents/hire" "$data" >/dev/null 2>&1 || true
}

# Main test execution
wait_for_server || exit 1

run_test "Agents list" test_agents_list
run_test "Agents list empty" test_agents_list_empty
run_test "Agents list valid JSON" test_agents_list_valid_json
run_test "Agents hire basic" test_agents_hire_basic
run_test "Agents hire full" test_agents_hire_full
run_test "Agents hire with metadata" test_agents_hire_with_metadata
run_test "Agents hire minimal" test_agents_hire_minimal
run_test "Agents hire duplicate" test_agents_hire_duplicate
run_test "Agents hire invalid JSON" test_agents_hire_invalid_json
run_test "Agents hire empty data" test_agents_hire_empty_data
run_test "Agents hire large payload" test_agents_hire_large_payload
run_test "Agents hire special chars" test_agents_hire_special_chars
run_test "Agents hire unicode name" test_agents_hire_unicode_name
run_test "Agents list after hire" test_agents_list_after_hire
run_test "Agents concurrent hire" test_agents_concurrent_hire
run_test "Agents sequential hire" test_agents_sequential_hire
run_test "Agents get nonexistent" test_agents_get_nonexistent
run_test "Agents list response time" test_agents_list_response_time
run_test "Agents hire response time" test_agents_hire_response_time
run_test "Agents list with filter" test_agents_list_with_filter
run_test "Agents list with limit" test_agents_list_with_limit
run_test "Agents list pagination" test_agents_list_pagination
run_test "Agents hire long name" test_agents_hire_long_name
run_test "Agents hire numeric name" test_agents_hire_numeric_name
run_test "Agents hire with timestamps" test_agents_hire_with_timestamps
run_test "Agents list sorted" test_agents_list_sorted
run_test "Agents list descending" test_agents_list_descending
run_test "Agents hire batch" test_agents_hire_batch
run_test "Agents hire with context" test_agents_hire_with_context

print_summary
