issue_title: "Implement actual provider connection or explicit pending/unavailable state for checkout links"
issue_description: |
  # Research Report: Checkout Link Generation

  ## Market Findings & User Frictions

  Our research investigated the payment and invoicing processes across popular small business operational tools, focusing on how payment request URLs are generated and presented to non-technical business owners.

  ### Existing Market Offerings

  1. **Stripe (Direct):** Standard checkout flow provides immediate, reliable checkout URLs.
  2. **HoneyBook:** Generates combined proposal and payment links dynamically upon document creation.
  3. **Square Invoices:** Integrated links that tie directly to Square POS processing systems.

  ### User Frictions and Sentiment

  Operators frequently complain about "broken links" or "fake links" when using automated systems that try to pre-generate URLs without completing the backend provider (e.g. Stripe) connection.
  * *Nora (Solo Web Pro):* "I sent a proposal to a client and they replied saying the link doesn't go anywhere. I lost the project because it looked unprofessional."

  ## Gap Analysis: OneHumanCorp Codebase

  ### Current State

  The OHC codebase currently contains functionality to create proposals, quotes, and invoices. However, there is a noted flaw recorded in `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md` (Finding F08).

  Specifically, earlier iterations of the code in `invoice.rs` (and booking helpers) would generate "checkout-looking URLs" that were completely fabricated and not tied to an actual provider session.

  The current codebase handles this by ensuring `stripe_payment_link` or `checkout_url` remains empty instead of generating fake URLs:
  *   In `src/server/api/invoice.rs`:
      ```rust
      // This operation creates a local draft, not a provider checkout session.
      // Empty means payment has not been configured; never invent a payable URL.
      let stripe_payment_link = String::new();
      ```

  However, finding F08 states: "Full provider sandbox replay/payment event reconciliation, persisted provider receipts across every workflow and all business transitions remain outstanding."

  Furthermore, in `docs/research/business_capability_and_usage_economics_audit.md`, the gap is described as: "Invoice creation separately generates a checkout-looking UUID URL without creating a provider session (`invoice.rs:45-48`). Real Stripe plumbing and placeholders coexist; consolidate and verify rather than claiming all payments are fake or all are ready."

  ### The Necessary Resolution

  To fully close F08, we need to design the final correct state for payment generation. A proposal or invoice creation should interact with the configured payment provider (e.g. Stripe) to generate a legitimate session, OR it should explicitly mark the state as pending/unavailable until the owner configures payment.

  ## Agentic Solution & Design Doc

  **High-Level Architecture:**

  1.  **Payment Configuration State:** OHC must explicitly know if a tenant has a configured, active payment provider.
  2.  **Invoice/Proposal Generation:** When a user creates an invoice or proposal that requires payment, the system checks the payment configuration.
      *   If **configured**: The system initiates a real provider session (e.g., Stripe Checkout Session) and securely persists the resulting real `checkout_url` and session ID.
      *   If **not configured**: The invoice is saved as a draft with a `NEEDS_PAYMENT_CONFIG` status (or similar explicit state), and no `checkout_url` is assigned.
  3.  **UI Representation:** The frontend must clearly distinguish between an invoice ready to send (has real URL) and an invoice missing payment setup.

  **Mobile UX Flow (375px):**
  *   **Owner View:** An invoice card shows a warning icon if "Payment Setup Required". Tapping it opens a modal to connect Stripe.
  *   **Client View:** The client cannot receive or view the invoice until the owner resolves the payment setup.

  ## Implementation Prompt

  **Objective:** Implement the final payment URL generation logic to consolidate real Stripe plumbing and remove any residual placeholder generation.

  **Requirements:**
  1.  **Provider Integration:** Ensure that proposal and invoice approval/generation flows only assign a `checkout_url` if a legitimate provider session is successfully created.
  2.  **Explicit State:** If no provider is configured, the system must set an explicit state (e.g., `PAYMENT_UNAVAILABLE`) and ensure `checkout_url` remains null/empty.
  3.  **Verify F08 Closure:** Ensure the changes satisfy the F08 requirement: "Real provider session or explicit pending/unavailable state; persist provider IDs, validate money, idempotent retries".
  4.  **No Fabrications:** Do not generate any UUID-based or dummy URLs for checkout links.

  **Priority:** P1
  **Estimated Scope:** Medium

  **Strategy Admission:**
  *   **OHC Target ID:** OHC-06 (Delivery -> invoice -> collected/reconciled balance)
  *   **Launch/Run stage:** Launch
  *   **Observed/Inferred Gap:** Observed (F08 in remediation ledger)
  *   **Baseline:** Currently uses empty strings to avoid fabrication, but does not fully integrate real sessions universally.
  *   **Dependencies/Reuse:** Stripe client module (`src/server/api/billing_api.rs`, `proposals.rs:488`)
  *   **Non-goals:** Building a custom payment processor.
  *   **Authority class:** Routine execution (generating links for approved amounts).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
