issue_title: "Implement the finding from this completed research report immediately"
issue_description: |
  # Implement the finding from this completed research report immediately

  The research report concluded that F13 (API key, consumer plan and native-client subscription are distinct) is blocked pending business agreements and provider permissions. It verified that `usage_meter.rs` correctly blocks `native_subscription` payer mode and `provider_facade.rs` enforces provider origins. No code changes are required because the required validation and isolation mechanisms are already implemented, and any further action is blocked. Therefore, this task is submitted as a blocked no-work finding.
issue_priority: "P0"
issue_category: "infrastructure"
issue_type: "feature"
issue_label: ""
assignees: []
