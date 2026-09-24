issue_title: "Blocked: Consolidate and verify fabricated checkout-looking URLs (F08)"
issue_description: |
  **Date:** 2026-09-19
  **Scope:** Implement real provider session or explicit pending/unavailable state, persist provider IDs, validate money, idempotent retries for checkout/booking URLs. (Item F08 in remediation ledger)

  **Finding:**
  The migration from fabricated checkout-looking URLs to real Stripe provider sessions is currently blocked.

  **Reason:**
  A missing provider sandbox and valid test credentials for Stripe prevents end-to-end verification. Live credentials cannot be tested, and a dedicated test sandbox setup is an outstanding verification dependency. Without this, we cannot verify normal navigation, result/pending/error states, or live-provider cost reconciliation for the checkout flow.

  **Code Evidence:**
  The current implementation correctly generates placeholder `String::new()` or `None` when a provider is unconfigured, which is safe, but cannot be advanced to real API usage without test access.
  - `src/server/api/invoice.rs:171` - `let stripe_payment_link = String::new();`
  - `src/server/api/booking/public.rs:109` - `let mut stripe_url = None;`

issue_priority: P1
issue_category: Maintainer
issue_type: Blocked
issue_label: [agent-report]
assignees: []
