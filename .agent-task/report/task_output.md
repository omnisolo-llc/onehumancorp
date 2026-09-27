issue_title: '🛡️ Sentinel: [blocked no-work finding: F13: BYOK versus subscriptions]'
issue_description: |
  # Sentinel Report: Tenant Isolation Audit

  ## Target
  F13: BYOK versus subscriptions (from business_capability_and_usage_economics_audit.md).

  ## Scope
  API key, consumer plan and native-client subscription are distinct. Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference.

  ## Findings
  Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented.

  ## Expected Business Result
  N/A

  ## Scope/non-goals
  N/A

  ## Dependencies
  N/A

  ## Stable acceptance criteria
  N/A

  ## Recovery/authority/cost requirements
  N/A

  ## Verification
  Checked `docs/research/native_migration_and_remediation.md` and `docs/research/business_capability_and_usage_economics_audit.md`.

  ## Final Evidence
  - Superpowers skill: `using-superpowers`
  - Loaded from revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks: Code audit completed
  - Outcomes: Blocked no-work finding

issue_priority: 'P2'
issue_category: 'Security'
issue_type: 'Audit'
issue_label: 'agent-report'
assignees: []
