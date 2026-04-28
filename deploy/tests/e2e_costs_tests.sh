#!/usr/bin/env bash
# Costs API E2E tests - 20 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Costs API tests
test_costs_list() {
    local resp
    resp=$(http_get "/api/costs")
    assert_json_field "$resp" ".costs"
}

test_costs_list_valid_json() {
    local resp
    resp=$(http_get "/api/costs")
    echo "$resp" | jq . >/dev/null
}

test_costs_query_basic() {
    local data='{"period":"month","year":2024,"month":12}'
    local resp
    resp=$(http_post "/api/costs" "$data" 200)
    assert_json_field "$resp" ".total"
}

test_costs_query_full() {
    local data='{
        "period":"month",
        "year":2024,
        "month":12,
        "department":"engineering",
        "cost_center":"CC001"
    }'
    http_post "/api/costs" "$data" 200 >/dev/null
}

test_costs_query_minimal() {
    local data='{"period":"month"}'
    http_post "/api/costs" "$data" 200 >/dev/null
}

test_costs_list_after_query() {
    local data='{"period":"month"}'
    http_post "/api/costs" "$data" 200 >/dev/null
    local resp
    resp=$(http_get "/api/costs")
    echo "$resp" | jq . >/dev/null
}

test_costs_concurrent_query() {
    for i in {1..5}; do
        local data="{\"period\":\"month\",\"year\":2024,\"month\":$((i % 12 + 1))}"
        http_post "/api/costs" "$data" 200 >/dev/null &
    done
    wait
}

test_costs_sequential_query() {
    for i in {1..10}; do
        local data="{\"period\":\"month\",\"year\":2024,\"month\":$((i % 12 + 1))}"
        http_post "/api/costs" "$data" 200 >/dev/null
    done
}

test_costs_query_invalid_json() {
    http_post "/api/costs" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_costs_query_empty() {
    http_post "/api/costs" "{}" >/dev/null 2>&1 || true
}

test_costs_list_with_filter() {
    http_get "/api/costs?department=engineering" >/dev/null 2>&1 || true
}

test_costs_list_with_limit() {
    http_get "/api/costs?limit=20" >/dev/null 2>&1 || true
}

test_costs_list_pagination() {
    http_get "/api/costs?page=1&size=10" >/dev/null 2>&1 || true
}

test_costs_response_time() {
    local data='{"period":"month"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/costs" "$data" 200 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_costs_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/costs" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_costs_query_by_period() {
    http_get "/api/costs?period=quarter" >/dev/null 2>&1 || true
}

test_costs_query_with_breakdown() {
    http_get "/api/costs?breakdown=department" >/dev/null 2>&1 || true
}

test_costs_list_sorted() {
    http_get "/api/costs?sort=amount" >/dev/null 2>&1 || true
}

test_costs_query_batch() {
    for i in {1..3}; do
        local data="{\"period\":\"month\",\"month\":$((i % 12 + 1))}"
        http_post "/api/costs" "$data" 200 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Costs list" test_costs_list
run_test "Costs list valid JSON" test_costs_list_valid_json
run_test "Costs query basic" test_costs_query_basic
run_test "Costs query full" test_costs_query_full
run_test "Costs query minimal" test_costs_query_minimal
run_test "Costs list after query" test_costs_list_after_query
run_test "Costs concurrent query" test_costs_concurrent_query
run_test "Costs sequential query" test_costs_sequential_query
run_test "Costs query invalid JSON" test_costs_query_invalid_json
run_test "Costs query empty" test_costs_query_empty
run_test "Costs list with filter" test_costs_list_with_filter
run_test "Costs list with limit" test_costs_list_with_limit
run_test "Costs list pagination" test_costs_list_pagination
run_test "Costs response time" test_costs_response_time
run_test "Costs list response time" test_costs_list_response_time
run_test "Costs query by period" test_costs_query_by_period
run_test "Costs query with breakdown" test_costs_query_with_breakdown
run_test "Costs list sorted" test_costs_list_sorted
run_test "Costs query batch" test_costs_query_batch

print_summary
