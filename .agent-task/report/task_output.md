issue_title: Documentation for Connected Accounts and Authority
issue_description: |
  **Title:** Documentation for Connected Accounts and Authority (F06)

  **Problem Statement:** The system needs plain language documentation on how an owner accomplishes verified business work with the current product, specifically explaining setup, connected accounts, standing authority, evidence, cost, exceptions and recovery for the current native-build product.

  **Research Report:**
  Source Dates: Based on the current code audit (revision 2026-09-18).
  Study Populations: Non-technical small business owners/operators.
  Uncertainties: The exact onboarding funnel for Google Workspace OAuth is still incomplete according to the audit, so documentation must reflect the current verified state (OpenAI API/Stripe).
  Metric Definitions: Cost tracking reflects idempotent records tied to tenant and task, not generic API balances.
  Scope: The focus is strictly on explaining the current verified behavior of connecting accounts, not planned features.

  **Design Doc:**
  # Connected Accounts & Your Standing Authority

  **Setting Up Your Accounts**
  To enable your AI team to work on your behalf, you need to connect your existing provider accounts (like Stripe for payments and OpenAI for AI services).
  - **Your Data Stays Yours:** OHC connects securely to your accounts and stores credentials in an encrypted, tenant-bound vault. We do not pool your subscriptions.
  - **Supported Providers:** Currently, secure connections are supported for OpenAI and Stripe. Other providers (like generic Google Workspace OAuth) are explicitly marked as unavailable until fully verified.

  **Understanding Standing Authority**
  When you connect an account, you grant the system "standing authority" to perform specific, approved business rules (like generating a draft proposal).
  - **Drafts vs. Actions:** The system will persist drafts (like a proposal or invoice). It will not send them, charge accounts, or make unauthorized commitments without your explicit approval path.
  - **Revocation:** You can revoke this authority at any time. If verification of your connection fails, the system safely marks the connection as stale and stops automated actions.

  **Costs & Evidence**
  - **Your Costs:** OHC tracks costs precisely. You will see an itemized OHC bill for the execution environment and a separate bill directly from your provider (e.g., OpenAI) for your API usage.
  - **Evidence of Work:** Every action the system takes generates a durable receipt tied to your specific task and attempt, ensuring you never pay for duplicate work.

  **Exceptions and Recovery**
  If a connection fails or a budget limit is reached, the system will pause the affected automation and notify you. It is designed to recover safely once the connection is refreshed or the limit is resolved.

  **Implementation Prompt:** Add this documentation to the In-App Help Center.
  **Priority:** High
  **Estimated Scope:** Small
issue_priority: High
issue_category: Documentation
issue_type: Task
issue_label: [documentation]
assignees: []
