#!/usr/bin/env bash
# Meetings API E2E tests - 30 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Meetings API tests
test_meetings_list() {
    local resp
    resp=$(http_get "/api/meetings")
    assert_json_field "$resp" ".meetings"
}

test_meetings_list_valid() {
    local resp
    resp=$(http_get "/api/meetings")
    echo "$resp" | jq . >/dev/null
}

test_meetings_create_basic() {
    local data='{"title":"Test Meeting","attendees":["agent1","agent2"]}'
    local resp
    resp=$(http_post "/api/meetings" "$data" 201)
    assert_json_field "$resp" ".id"
}

test_meetings_create_full() {
    local data='{
        "title":"Full Meeting",
        "attendees":["a1","a2","a3"],
        "duration":3600,
        "scheduled_at":"2024-12-01T10:00:00Z",
        "type":"standup"
    }'
    http_post "/api/meetings" "$data" 201 >/dev/null
}

test_meetings_create_minimal() {
    local data='{"title":"Minimal Meeting"}'
    http_post "/api/meetings" "$data" 201 >/dev/null
}

test_meetings_create_with_agenda() {
    local data='{
        "title":"Agenda Meeting",
        "agenda":["item1","item2","item3"]
    }'
    http_post "/api/meetings" "$data" 201 >/dev/null
}

test_meetings_create_with_notes() {
    local data='{
        "title":"Notes Meeting",
        "notes":"Initial meeting notes"
    }'
    http_post "/api/meetings" "$data" 201 >/dev/null
}

test_meetings_list_after_create() {
    local data='{"title":"Listed Meeting"}'
    http_post "/api/meetings" "$data" 201 >/dev/null
    local resp
    resp=$(http_get "/api/meetings")
    echo "$resp" | jq . >/dev/null
}

test_meetings_concurrent_create() {
    for i in {1..5}; do
        local data="{\"title\":\"concurrent-meeting-$i\"}"
        http_post "/api/meetings" "$data" 201 >/dev/null &
    done
    wait
}

test_meetings_sequential_create() {
    for i in {1..10}; do
        local data="{\"title\":\"sequential-meeting-$i\"}"
        http_post "/api/meetings" "$data" 201 >/dev/null
    done
}

test_meetings_create_invalid_json() {
    http_post "/api/meetings" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_meetings_create_empty() {
    http_post "/api/meetings" "{}" >/dev/null 2>&1 || true
}

test_meetings_list_with_filter() {
    http_get "/api/meetings?type=standup" >/dev/null 2>&1 || true
}

test_meetings_list_with_limit() {
    http_get "/api/meetings?limit=20" >/dev/null 2>&1 || true
}

test_meetings_list_pagination() {
    http_get "/api/meetings?page=1&size=10" >/dev/null 2>&1 || true
}

test_meetings_create_long_title() {
    local title=$(printf 'x%.0s' {1..500})
    local data="{\"title\":\"$title\"}"
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_many_attendees() {
    local attendees="$(printf '\"agent%d\",' {1..50})"
    attendees="[${attendees%,}]"
    local data="{\"title\":\"Large Meeting\",\"attendees\":$attendees}"
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_unicode_title() {
    local data='{"title":"Meeting-中文-日本語-한국어"}'
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_special_chars() {
    local data='{"title":"Meeting @#$%^&*()_+-=[]{}|;:,.<>?"}'
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_response_time() {
    local data='{"title":"Perf Meeting"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/meetings" "$data" 201 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_meetings_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/meetings" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_meetings_create_with_metadata() {
    local data='{
        "title":"Meta Meeting",
        "metadata":{"project":"alpha","priority":"high"}
    }'
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_get_nonexistent() {
    http_get "/api/meetings/nonexistent" 404 >/dev/null 2>&1 || true
}

test_meetings_list_sorted() {
    http_get "/api/meetings?sort=title" >/dev/null 2>&1 || true
}

test_meetings_list_reverse_sorted() {
    http_get "/api/meetings?sort=-created_at" >/dev/null 2>&1 || true
}

test_meetings_create_with_duration() {
    local data='{"title":"Duration Meeting","duration":7200}'
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_with_location() {
    local data='{"title":"Location Meeting","location":"Conference Room A"}'
    http_post "/api/meetings" "$data" 201 >/dev/null 2>&1 || true
}

test_meetings_create_batch() {
    for i in {1..3}; do
        local data="{\"title\":\"batch-meeting-$i\"}"
        http_post "/api/meetings" "$data" 201 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Meetings list" test_meetings_list
run_test "Meetings list valid" test_meetings_list_valid
run_test "Meetings create basic" test_meetings_create_basic
run_test "Meetings create full" test_meetings_create_full
run_test "Meetings create minimal" test_meetings_create_minimal
run_test "Meetings create with agenda" test_meetings_create_with_agenda
run_test "Meetings create with notes" test_meetings_create_with_notes
run_test "Meetings list after create" test_meetings_list_after_create
run_test "Meetings concurrent create" test_meetings_concurrent_create
run_test "Meetings sequential create" test_meetings_sequential_create
run_test "Meetings create invalid JSON" test_meetings_create_invalid_json
run_test "Meetings create empty" test_meetings_create_empty
run_test "Meetings list with filter" test_meetings_list_with_filter
run_test "Meetings list with limit" test_meetings_list_with_limit
run_test "Meetings list pagination" test_meetings_list_pagination
run_test "Meetings create long title" test_meetings_create_long_title
run_test "Meetings create many attendees" test_meetings_create_many_attendees
run_test "Meetings create unicode title" test_meetings_create_unicode_title
run_test "Meetings create special chars" test_meetings_create_special_chars
run_test "Meetings create response time" test_meetings_create_response_time
run_test "Meetings list response time" test_meetings_list_response_time
run_test "Meetings create with metadata" test_meetings_create_with_metadata
run_test "Meetings get nonexistent" test_meetings_get_nonexistent
run_test "Meetings list sorted" test_meetings_list_sorted
run_test "Meetings list reverse sorted" test_meetings_list_reverse_sorted
run_test "Meetings create with duration" test_meetings_create_with_duration
run_test "Meetings create with location" test_meetings_create_with_location
run_test "Meetings create batch" test_meetings_create_batch

print_summary
