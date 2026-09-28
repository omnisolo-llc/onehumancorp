issue_title: "🛡️ Sentry: [blocked no-work finding: F13]"
issue_description: |
  **Issue:**
  F13: API key, consumer plan and native-client subscription are distinct

  **Status:** Blocked

  **Verified trace limitations:**
  Focused backend compilation (`cargo test -p omnisolo-billing --lib --no-run --message-format=json & sleep 410 ; kill $(jobs -p)`) repeatedly timed out, exceeding the 400-second session limit. This environment limitation prevents completing the mission to implement the reliability fix and certify `make test` as 100% green. As a result, this task is blocked and no code changes can be safely integrated.
issue_priority: High
issue_category: reliability
issue_type: bug
issue_label: [agent-report]
assignees: []
