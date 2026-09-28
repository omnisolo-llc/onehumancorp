issue_title: Sentinel F13 Blocked finding
issue_description: |
  # Blocked no-work finding

  ## Target
  F13

  ## Justification
  After investigation, I am blocking this remediation because provider-permitted native-client subscription hosting requires a separate integration decision, Terms of Service review, and quota check which cannot be evaluated without external, non-technical business owner/legal input. I verified the current constraints in `src/server/harness/middleware/usage_meter.rs` where the `API proxy accepts managed_api or byok_api only; native_subscription sessions cannot be relayed`. The current status of `F13` in `docs/research/native_migration_and_remediation.md` is "Open" and "API key, consumer plan and native-client subscription are distinct".

  ## Trace Execution
  I verified that F13 is open via grep. I verified `native_subscription` rejection is explicitly implemented in `src/server/harness/middleware/usage_meter.rs`. The constraint correctly exists today.
issue_priority: "P1"
issue_category: "Security"
issue_type: "Defect"
issue_label: ""
assignees: []
