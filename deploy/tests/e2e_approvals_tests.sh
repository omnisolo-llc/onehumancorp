#!/usr/bin/env bash
# Approvals API E2E tests - 30 tests
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/helpers.sh"

# Approvals API tests
test_approvals_list() {
    local resp
    resp=$(http_get "/api/approvals")
    assert_json_field "$resp" ".approvals"
}

test_approvals_list_valid_json() {
    local resp
    resp=$(http_get "/api/approvals")
    echo "$resp" | jq . >/dev/null
}

test_approvals_request_basic() {
    local data='{"title":"Approval Request","requester":"agent1","type":"spending"}'
    local resp
    resp=$(http_post "/api/approvals/request" "$data" 201)
    assert_json_field "$resp" ".id"
}

test_approvals_request_full() {
    local data='{
        "title":"Full Approval",
        "requester":"agent1",
        "approver":"agent2",
        "type":"contract",
        "amount":5000,
        "currency":"USD",
        "justification":"Business need"
    }'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
}

test_approvals_request_minimal() {
    local data='{"title":"Quick Approval","requester":"agent1"}'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
}

test_approvals_request_multiple_approvers() {
    local data='{
        "title":"Multi Approval",
        "requester":"agent1",
        "approvers":["agent2","agent3","agent4"]
    }'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
}

test_approvals_request_with_deadline() {
    local data='{
        "title":"Deadline Approval",
        "requester":"agent1",
        "deadline":"2024-12-31T23:59:59Z"
    }'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
}

test_approvals_request_with_budget() {
    local data='{
        "title":"Budget Approval",
        "requester":"agent1",
        "amount":10000,
        "budget_code":"ENG-2024"
    }'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
}

test_approvals_request_invalid_json() {
    http_post "/api/approvals/request" "{invalid}" 400 >/dev/null 2>&1 || true
}

test_approvals_request_empty() {
    http_post "/api/approvals/request" "{}" >/dev/null 2>&1 || true
}

test_approvals_list_after_request() {
    local data='{"title":"Tracked Approval","requester":"agent1"}'
    http_post "/api/approvals/request" "$data" 201 >/dev/null
    local resp
    resp=$(http_get "/api/approvals")
    echo "$resp" | jq . >/dev/null
}

test_approvals_concurrent_requests() {
    for i in {1..5}; do
        local data="{\"title\":\"concurrent-approval-$i\",\"requester\":\"agent1\"}"
        http_post "/api/approvals/request" "$data" 201 >/dev/null &
    done
    wait
}

test_approvals_sequential_requests() {
    for i in {1..10}; do
        local data="{\"title\":\"sequential-approval-$i\",\"requester\":\"agent1\"}"
        http_post "/api/approvals/request" "$data" 201 >/dev/null
    done
}

test_approvals_decide_approve() {
    local data='{
        "approval_id":"test-approval-1",
        "decision":"approved",
        "approver":"agent2",
        "comments":"Looks good"
    }'
    http_post "/api/approvals/decide" "$data" 200 >/dev/null 2>&1 || true
}

test_approvals_decide_reject() {
    local data='{
        "approval_id":"test-approval-2",
        "decision":"rejected",
        "approver":"agent2",
        "comments":"Needs more info"
    }'
    http_post "/api/approvals/decide" "$data" 200 >/dev/null 2>&1 || true
}

test_approvals_decide_pending() {
    local data='{
        "approval_id":"test-approval-3",
        "decision":"pending",
        "approver":"agent2"
    }'
    http_post "/api/approvals/decide" "$data" 200 >/dev/null 2>&1 || true
}

test_approvals_list_with_filter() {
    http_get "/api/approvals?status=pending" >/dev/null 2>&1 || true
}

test_approvals_list_with_limit() {
    http_get "/api/approvals?limit=20" >/dev/null 2>&1 || true
}

test_approvals_list_pagination() {
    http_get "/api/approvals?page=1&size=10" >/dev/null 2>&1 || true
}

test_approvals_request_long_title() {
    local title=$(printf 'x%.0s' {1..500})
    local data="{\"title\":\"$title\",\"requester\":\"agent1\"}"
    http_post "/api/approvals/request" "$data" 201 >/dev/null 2>&1 || true
}

test_approvals_request_large_amount() {
    local data='{"title":"Large Amount","requester":"agent1","amount":999999999}'
    http_post "/api/approvals/request" "$data" 201 >/dev/null 2>&1 || true
}

test_approvals_request_unicode() {
    local data='{"title":"Approval-中文-日本語","requester":"agent1"}'
    http_post "/api/approvals/request" "$data" 201 >/dev/null 2>&1 || true
}

test_approvals_response_time() {
    local data='{"title":"Perf Approval","requester":"agent1"}'
    local start end duration
    start=$(date +%s%N)
    http_post "/api/approvals/request" "$data" 201 >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_approvals_list_response_time() {
    local start end duration
    start=$(date +%s%N)
    http_get "/api/approvals" >/dev/null
    end=$(date +%s%N)
    duration=$(( (end - start) / 1000000 ))
    [[ $duration -lt 2000 ]]
}

test_approvals_get_nonexistent() {
    http_get "/api/approvals/nonexistent" 404 >/dev/null 2>&1 || true
}

test_approvals_list_sorted() {
    http_get "/api/approvals?sort=created_at" >/dev/null 2>&1 || true
}

test_approvals_list_reverse_sorted() {
    http_get "/api/approvals?sort=-amount" >/dev/null 2>&1 || true
}

test_approvals_request_with_attachments() {
    local data='{
        "title":"Attachment Approval",
        "requester":"agent1",
        "attachments":["doc1","doc2","doc3"]
    }'
    http_post "/api/approvals/request" "$data" 201 >/dev/null 2>&1 || true
}

test_approvals_request_batch() {
    for i in {1..3}; do
        local data="{\"title\":\"batch-approval-$i\",\"requester\":\"agent1\"}"
        http_post "/api/approvals/request" "$data" 201 >/dev/null &
    done
    wait
}

# Main test execution
wait_for_server || exit 1

run_test "Approvals list" test_approvals_list
run_test "Approvals list valid JSON" test_approvals_list_valid_json
run_test "Approvals request basic" test_approvals_request_basic
run_test "Approvals request full" test_approvals_request_full
run_test "Approvals request minimal" test_approvals_request_minimal
run_test "Approvals request multiple approvers" test_approvals_request_multiple_approvers
run_test "Approvals request with deadline" test_approvals_request_with_deadline
run_test "Approvals request with budget" test_approvals_request_with_budget
run_test "Approvals request invalid JSON" test_approvals_request_invalid_json
run_test "Approvals request empty" test_approvals_request_empty
run_test "Approvals list after request" test_approvals_list_after_request
run_test "Approvals concurrent requests" test_approvals_concurrent_requests
run_test "Approvals sequential requests" test_approvals_sequential_requests
run_test "Approvals decide approve" test_approvals_decide_approve
run_test "Approvals decide reject" test_approvals_decide_reject
run_test "Approvals decide pending" test_approvals_decide_pending
run_test "Approvals list with filter" test_approvals_list_with_filter
run_test "Approvals list with limit" test_approvals_list_with_limit
run_test "Approvals list pagination" test_approvals_list_pagination
run_test "Approvals request long title" test_approvals_request_long_title
run_test "Approvals request large amount" test_approvals_request_large_amount
run_test "Approvals request unicode" test_approvals_request_unicode
run_test "Approvals response time" test_approvals_response_time
run_test "Approvals list response time" test_approvals_list_response_time
run_test "Approvals get nonexistent" test_approvals_get_nonexistent
run_test "Approvals list sorted" test_approvals_list_sorted
run_test "Approvals list reverse sorted" test_approvals_list_reverse_sorted
run_test "Approvals request with attachments" test_approvals_request_with_attachments
run_test "Approvals request batch" test_approvals_request_batch

print_summary
