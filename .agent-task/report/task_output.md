issue_title: "✍️ Scribe: [new documentation feature]"
issue_description: |
  **Title:** Owner Guide: Setup, Connected Accounts, and Standing Authority

  **Source Dates:**
  - 2026-09-18 (from native_migration_and_remediation.md baseline)
  - 2026-09-19 (from native_build_measurements_2026-09-19.md)

  **Study Populations:**
  - Non-technical business owners establishing digital operations (per RESEARCH.md capability map "Establish the operation").

  **Uncertainties:**
  - Google Workspace OAuth lifecycle and wider connector support remain unverified/incomplete (F06).
  - Supported native-client subscription mode is not yet demonstrated end-to-end.
  - Exact willingness to pay and desired autonomy levels remain unknown (from business capability audit).

  **Metric Definitions:**
  - **Serving Cost:** Includes OHC-funded model usage, allocated CPU/memory/GPU, and support labor, measured in exact integer subunits.
  - **Budget Reservation:** Hard spend caps established atomically before any external spend is authorized (F03).
  - **Connection State:** Valid states include 'verified', 'verification_required', or 'revoked' (from src/server/api/tool_integrations.rs).

  **Scope:**
  - Documenting existing capabilities for Stripe and OpenAI API connections.
  - Detailing standing authority limits, budget enforcement, and connection revocation.
  - Excludes planned features like Google Workspace OAuth or local inference modes that lack verified end-to-end evidence.

  **Documentation Findings:**

  ### 1. Setup and Connected Accounts
  To run your business, your AI team needs access to specific tools. Currently, you can securely connect **Stripe** (for payments and invoicing) and **OpenAI** (for AI inference).
  - **Security:** Your credentials are encrypted and locked to your specific business (tenant-bound). We never store credentials in plain text.
  - **Unsupported Tools:** Connections to other tools (like Google Workspace) are currently disabled to ensure your data remains secure until those integrations are fully verified.

  ### 2. Standing Authority and Exceptions
  Your AI team operates strictly within the policies you set.
  - **Revocation:** As an owner or admin, you can revoke a connection at any time. When you revoke an account (e.g., Stripe), the system immediately blocks new requests, though requests already sent to the provider may finish.
  - **Exceptions & Recovery:** If a connection becomes invalid, its status changes to "verification required." The AI team will pause workflows relying on that tool until you re-authenticate.

  ### 3. Cost, Evidence, and Budget Limits
  - **Hard Spend Caps:** Before the AI team takes any action that costs money, it makes an atomic reservation against your budget. If your budget is exhausted, the system fails safely and requests your intervention. It will not overspend.
  - **Usage Accounting:** Every action is durably logged with specific attribution (payer, auth mode, model, and rate). We never double-charge you for usage on your own API keys.
  - **Evidence:** When the AI team completes a task (like creating an invoice), you receive a verified, source-linked receipt directly from the provider (e.g., Stripe), ensuring you have proof the work was completed.

  **Skill Provenance:**
  Applied Superpowers workflow (using-superpowers, brainstorming, writing-plans) from revision 5bf4e78011075bcfc0dc295f0724994cd123ee71.
issue_priority: P2
issue_category: Documentation
issue_type: Feature
issue_label: ohc:lane:documentation
assignees: []
