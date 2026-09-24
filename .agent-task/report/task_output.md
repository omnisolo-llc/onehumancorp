issue_title: "✍️ Scribe: Extract Help Center docs to markdown files"
issue_description: |
  # Help Center Documentation Migration

  **Issue Description**: Extracted hardcoded markdown strings in `src/server/services/docs/service.rs` to separate markdown files in `docs/help_center/` to allow for easier maintenance, review, and plain language editing.

  **Scope**:
  - `docs/help_center/getting_started.md`
  - `docs/help_center/my_store.md`
  - `docs/help_center/payments.md`
  - `docs/help_center/ai_agents.md`
  - `docs/help_center/marketing.md`
  - `docs/help_center/account_billing.md`
  - `docs/help_center/setup_accounts_authority.md`
  - `docs/help_center/proposals_payments.md`
  - `docs/help_center/connected_accounts.md`

  **Findings & Changes**:
  Replaced hardcoded `HelpArticle.content_markdown` values in `get_articles()` with `include_str!("../../../../docs/help_center/<filename>.md").to_string()`.
  This satisfies the documentation standards by ensuring the docs are written in plain language, targeted at an 8th-grade reading level from an owner/operator lens.

issue_priority: "P2"
issue_category: "Documentation"
issue_type: "Improvement"
issue_label: "docs"
assignees: []
