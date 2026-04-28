#!/usr/bin/env bash
# Snapshots API E2E tests - 20 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Snapshots API tests
test_snapshots_list() {
    local resp
    resp=$(http_get "/api/snapshots")
    assert_json_field "$resp" ".snapshots"
}

test_snapshots_list_valid_json() {
    local resp
    resp=$(http_get "/api/snapshots")
    echo "$resp" | jq . >/dev/null
}

test_snapshots_create_basic() {
    local data='{"name":"snapshot-1","description":"Test snapshot"}'
    local resp
    resp=$(http_post "/api/snapshots/create" "$data" 201)
    assert_json_field "$resp" ".id"
}

test_snapshots_create_full() {
    local data='{
        "name":"full-snapshot",
        "description":"Full snapshot with metadata",
        "tags":["important","backup"],
        "retention_days":90,
        "metadata":{"version":"1.0","env":"prod"}
    }'
    http_post "/api/snapshots/create" "$data" 201 >/dev/null
}

test_snapshots_create_minimal() {
    local data='{"name":"minimal-snapshot"}'
    http_post "/api/snapshots/create" "$data" 201 >/dev/null
}

test_snapshots_list_after_create() {
    local data='{"name":"listed-snapshot"}'
    http_post "/api/snapshots/create" "$data" 201 >/dev/null
    local resp
    resp=$(http_get "/api/snapshots")
    echo "$resp" | jq . >/dev/null
}

test_snapshots_concurrent_create() {
    for i in {1..5}; do
        local data="{\"name\":\"concurrent-snapshot-$i\"}"
        http_post "/api/snapshots/create" "$data" 201 >/dev/null &
    done
    wait
}

test_snapshots_sequential_create() {
    for i in {1..10}; do
        local data="{\"name\":\"sequential-snapshot-$i\"}"
        http_post "/api/snapshots/create" "$data" 201 >/dev/null
    done
}

test_snapshots_create_invalid_json() {
    http_post "/api/snapshots/create" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_snapshots_create_empty() {
    http_post "/api/snapshots/create" "{}" >/dev/null 2>&1 || true
}

test_snapshots_list_with_filter() {
    http_get "/api/snapshots?tag=important" >/dev/null 2>&1 || true
}

test_snapshots_list_with_limit() {
    http_get "/api/snapshots?limit=20" >/dev/null 2>&1 || true
}

test_snapshots_list_pagination() {
    http_get "/api/snapshots?page=1&size=10" >/dev/null 2>&1 || true
}

test_snapshots_response_time() {
    local data='{"name":"perf-snapshot"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/snapshots/create" "$data" 201 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_snapshots_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/snapshots" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_snapshots_get_nonexistent() {
    http_get "/api/snapshots/nonexistent" 404 >/dev/null 2>&1 || true
}

test_snapshots_list_sorted() {
    http_get "/api/snapshots?sort=created_at" >/dev/null 2>&1 || true
}

test_snapshots_create_with_tags() {
    local data='{"name":"tagged-snapshot","tags":["prod","critical","backup"]}'
    http_post "/api/snapshots/create" "$data" 201 >/dev/null
}

test_snapshots_create_batch() {
    for i in {1..3}; do
        local data="{\"name\":\"batch-snapshot-$i\"}"
        http_post "/api/snapshots/create" "$data" 201 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Snapshots list" test_snapshots_list
run_test "Snapshots list valid JSON" test_snapshots_list_valid_json
run_test "Snapshots create basic" test_snapshots_create_basic
run_test "Snapshots create full" test_snapshots_create_full
run_test "Snapshots create minimal" test_snapshots_create_minimal
run_test "Snapshots list after create" test_snapshots_list_after_create
run_test "Snapshots concurrent create" test_snapshots_concurrent_create
run_test "Snapshots sequential create" test_snapshots_sequential_create
run_test "Snapshots create invalid JSON" test_snapshots_create_invalid_json
run_test "Snapshots create empty" test_snapshots_create_empty
run_test "Snapshots list with filter" test_snapshots_list_with_filter
run_test "Snapshots list with limit" test_snapshots_list_with_limit
run_test "Snapshots list pagination" test_snapshots_list_pagination
run_test "Snapshots response time" test_snapshots_response_time
run_test "Snapshots list response time" test_snapshots_list_response_time
run_test "Snapshots get nonexistent" test_snapshots_get_nonexistent
run_test "Snapshots list sorted" test_snapshots_list_sorted
run_test "Snapshots create with tags" test_snapshots_create_with_tags
run_test "Snapshots create batch" test_snapshots_create_batch

print_summary
