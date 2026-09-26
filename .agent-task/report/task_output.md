issue_title: "✍️ Scribe: Audit of F06: unusable connection flow"
issue_description: |
  # Audit of F06: Unusable connection flow

  ## Problem Statement
  As the Principal Technical Writer & Scribe (L7), my mission is to document how an owner accomplishes verified business work with the current product, specifically explaining setup, connected accounts, and standing authority.

  However, based on the `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md`, creating user-facing documentation for the connection flow is currently blocked. The business capability audit notes for `api/tool_integrations.rs`: "Connect deliberately returns HTTP 501 because verification/encrypted storage are unavailable. Listed integrations have `usable: false`. This surface is incomplete but fails honestly; do not replace the failure with a fake connected badge."

  The native migration remediation ledger for F06 states: "Encrypted, tenant/provider-bound vault; supported OpenAI API/Stripe connection controls and read-only verification, revocation, recheck and stale-verification states." are closed/present. However, it also states: "Google Workspace OAuth lifecycle and wider connector support remain unverified/incomplete. A native subscription is not supported merely by having an API key field. No live credentials were tested."

  ## Research Report & Findings
  1. The underlying UI to display connected integrations returns `usable: false` and `HTTP 501` for the connection flow.
  2. While some backend elements (vault, OpenAI/Stripe controls) exist, the end-to-end connection flow is incomplete and unverified with live credentials.
  3. Google Workspace OAuth lifecycle is incomplete.
  4. Without a fully working, owner-verified loop for connecting accounts (which is a prerequisite for standing authority and automated workflows), writing a user-facing help center article, interactive walkthrough, or video tutorial would require fabricating UI and behavior that does not exist or work reliably.

  ## Blocked Prerequisites
  1. **Working E2E Connection Flow**: The connection flow in `api/tool_integrations.rs` must be implemented to return a successful connection state rather than HTTP 501.
  2. **Verified OAuth Lifecycle**: Google Workspace and other essential connectors must have a verified, functional OAuth lifecycle.
  3. **Live Credential Verification**: The connection flows must be verified with live credentials (or valid sandbox equivalents) to ensure accurate documentation.

  Until these prerequisites are met, the documentation task "Setup and connected accounts" results in a **no-work/blocked** outcome.

  ---
  ## Superpowers Workflow Provenance
  - **Repository:** `https://github.com/obra/superpowers.git`
  - **Revision:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Loaded Skills:**
    - `using-superpowers`
    - `brainstorming`
  - **Checks Performed:**
    - Read `AGENTS.md` and `RESEARCH.md` for scope and authority.
    - Read `docs/research/business_capability_and_usage_economics_audit.md` for F06 capability status.
    - Read `docs/research/native_migration_and_remediation.md` for F06 remediation status.
    - Verified the skills in the Superpowers checkout at `.scratch/superpowers`.
  - **Outcomes:**
    - Determined that the user-facing documentation task is blocked by incomplete implementation of the tool integrations connection flow (F06).
    - Produced this audit report documenting the blocked prerequisites in accordance with the `no-work/blocked` outcome requirements.

issue_priority: "P1"
issue_category: "documentation"
issue_type: "audit"
issue_label: "ohc:lane:documentation"
assignees: []
