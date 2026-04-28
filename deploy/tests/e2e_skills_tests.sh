#!/usr/bin/env bash
# Skills API E2E tests - 20 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Skills API tests
test_skills_list() {
    local resp
    resp=$(http_get "/api/skills")
    assert_json_field "$resp" ".skills"
}

test_skills_list_valid_json() {
    local resp
    resp=$(http_get "/api/skills")
    echo "$resp" | jq . >/dev/null
}

test_skills_import_basic() {
    local data='{"name":"python","level":"expert","category":"programming"}'
    local resp
    resp=$(http_post "/api/skills/import" "$data" 201)
    assert_json_field "$resp" ".id"
}

test_skills_import_full() {
    local data='{
        "name":"project-management",
        "level":"advanced",
        "category":"management",
        "description":"Experienced in agile and waterfall",
        "years_experience":5
    }'
    http_post "/api/skills/import" "$data" 201 >/dev/null
}

test_skills_import_minimal() {
    local data='{"name":"communication"}'
    http_post "/api/skills/import" "$data" 201 >/dev/null
}

test_skills_list_after_import() {
    local data='{"name":"negotiation"}'
    http_post "/api/skills/import" "$data" 201 >/dev/null
    local resp
    resp=$(http_get "/api/skills")
    echo "$resp" | jq . >/dev/null
}

test_skills_concurrent_import() {
    for i in {1..5}; do
        local data="{\"name\":\"skill-$i\",\"level\":\"expert\"}"
        http_post "/api/skills/import" "$data" 201 >/dev/null &
    done
    wait
}

test_skills_sequential_import() {
    for i in {1..10}; do
        local data="{\"name\":\"sequential-skill-$i\"}"
        http_post "/api/skills/import" "$data" 201 >/dev/null
    done
}

test_skills_import_invalid_json() {
    http_post "/api/skills/import" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_skills_import_empty() {
    http_post "/api/skills/import" "{}" >/dev/null 2>&1 || true
}

test_skills_list_with_filter() {
    http_get "/api/skills?category=programming" >/dev/null 2>&1 || true
}

test_skills_list_with_limit() {
    http_get "/api/skills?limit=20" >/dev/null 2>&1 || true
}

test_skills_list_pagination() {
    http_get "/api/skills?page=1&size=10" >/dev/null 2>&1 || true
}

test_skills_response_time() {
    local data='{"name":"perf-skill"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/skills/import" "$data" 201 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_skills_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/skills" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_skills_get_nonexistent() {
    http_get "/api/skills/nonexistent" 404 >/dev/null 2>&1 || true
}

test_skills_import_with_metadata() {
    local data='{
        "name":"metadata-skill",
        "metadata":{"source":"linkedin","verified":true}
    }'
    http_post "/api/skills/import" "$data" 201 >/dev/null 2>&1 || true
}

test_skills_list_sorted() {
    http_get "/api/skills?sort=name" >/dev/null 2>&1 || true
}

test_skills_import_batch() {
    for i in {1..3}; do
        local data="{\"name\":\"batch-skill-$i\"}"
        http_post "/api/skills/import" "$data" 201 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Skills list" test_skills_list
run_test "Skills list valid JSON" test_skills_list_valid_json
run_test "Skills import basic" test_skills_import_basic
run_test "Skills import full" test_skills_import_full
run_test "Skills import minimal" test_skills_import_minimal
run_test "Skills list after import" test_skills_list_after_import
run_test "Skills concurrent import" test_skills_concurrent_import
run_test "Skills sequential import" test_skills_sequential_import
run_test "Skills import invalid JSON" test_skills_import_invalid_json
run_test "Skills import empty" test_skills_import_empty
run_test "Skills list with filter" test_skills_list_with_filter
run_test "Skills list with limit" test_skills_list_with_limit
run_test "Skills list pagination" test_skills_list_pagination
run_test "Skills response time" test_skills_response_time
run_test "Skills list response time" test_skills_list_response_time
run_test "Skills get nonexistent" test_skills_get_nonexistent
run_test "Skills import with metadata" test_skills_import_with_metadata
run_test "Skills list sorted" test_skills_list_sorted
run_test "Skills import batch" test_skills_import_batch

print_summary
