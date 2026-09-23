issue_title: |
  Fix tenant isolation and local data exposure vulnerabilities
issue_description: |
  # Title
  Fix tenant isolation and local data exposure vulnerabilities

  # Problem Statement
  An audit revealed that the current codebase has vulnerabilities that could break tenant isolation or expose local data.
  This report investigates the scope of those vulnerabilities and whether there is immediate action needed to address them.
  According to `docs/research/native_migration_and_remediation.md`, F06 requires verified supported-provider connection with encrypted storage, tenant binding, revoke/refresh behavior, and unsupported providers must remain explicitly unavailable. The `tool_integrations.rs` is returning 501/usable:false for secure connections.
  However, `F06` is a very specific implementation of provider vaults (OpenAI and Stripe) but it notes: "Google Workspace OAuth lifecycle and wider connector support remain unverified/incomplete. A native subscription is not supported merely by having an API key field. No live credentials were tested."

  The `tool_integrations.rs` connection endpoint `connect_integration_handler` implements tenant-scoped encrypted storage for `"openai_api"` and `"stripe"` but rejects others with 501. This is the desired behavior for unsupported providers, as stated in the acceptance register: "unsupported providers remain explicitly unavailable".

  The task is marked as Maintainer/Sentinel looking for tenant leaks and local data exposure.
  Reviewing `src/server/api/tool_integrations.rs`, the API correctly checks for tenant IDs and uses tenant-bound connections. The `connection_vault.rs` uses tenant-bound encryption keys and explicitly isolates connections by tenant.
  There is no actionable tenant data leakage or local data exposure vulnerability identified that hasn't already been addressed by the `ConnectionVault` implementation or is outside the scope of current supported providers.
  Therefore, no code changes are required for this specific sweep, and this task is a no-work finding.

  # Scope
  - Review `tool_integrations.rs` and `connection_vault.rs` for tenant isolation.
  - Review `native_migration_and_remediation.md` regarding F06.

  # Finding
  No actionable vulnerabilities found that aren't already mitigated or outside the scope of currently supported providers. The requested work is blocked or already satisfied.
issue_priority: High
issue_category: Security
issue_type: Report
issue_label: [agent-report]
assignees: []
