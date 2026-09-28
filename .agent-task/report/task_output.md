issue_title: "Blocked no-work finding: F13: BYOK versus subscriptions"
issue_description: |
  Target F13 (API key, consumer plan and native-client subscription are distinct) is marked as Open in docs/research/native_migration_and_remediation.md.
  However, this issue is a research topic ("Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented.").

  Verified trace limitations:
  1. No specific code change was made because the task requires a product decision about "Provider-permitted native-client subscription hosting".
  2. The issue describes that BYOK vs subscription is a business integration decision, not a purely technical implementation flaw in the codebase.

  Executed test commands:
  - `make test-node` (Truncated output)
  - `cargo check --locked --workspace --exclude app --all-targets` (Manually terminated)
issue_priority: "P0"
issue_category: "research"
issue_type: "blocked"
issue_label: "blocked"
assignees: []
