issue_title: "Implement Multi-Currency Tap-to-Pay Terminal & Instant Localized Invoicing"
issue_description: |
  # Research Report: Multi-Currency Tap-to-Pay Terminal & Instant Localized Invoicing

  ## 1. Problem Statement
  Owners like Priya (boutique operator) and Carlos (field service owner) often face customers who prefer local payment methods or need instant, localized invoicing after a service or purchase. Currently, OHC lacks a unified architecture that handles multi-currency transactions via tap-to-pay (using Stripe Terminal or similar SDKs) combined with instant, localized invoice generation that the Operations and Finance AI Agents can seamlessly track. This gap prevents global usability and frictionless in-person sales for non-technical operators.

  ## 2. Research Report & Competitive Analysis
  - **Stripe & Square:** Both offer robust Tap-to-Pay on iPhone/Android, bypassing the need for dedicated hardware. However, their raw SDKs are highly technical and not easily unified with an AI assistant that drafts follow-ups.
  - **Shopify POS:** Offers excellent multi-currency support, but the setup process is complex and often feels like an "admin portal" rather than an invisible assistant.
  - **Wix & GoDaddy:** Basic POS integrations exist, but they lack the localized, instant invoice generation triggered seamlessly by a successful tap-to-pay event.
  - **The OHC Opportunity:** By wrapping Tap-to-Pay SDKs within our Assistant-First shell, the owner simply taps "Collect Payment." The AI handles the currency conversion, the NFC session, and instantly drafts a localized receipt/invoice, sending it via WhatsApp/Email based on customer context.

  ## 3. Design Doc: Architectural Design

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile Client (375px)
      participant API as OHC API Layer (gRPC/REST)
      participant DB as PostgreSQL (Tenant Isolated)
      participant Stripe as Stripe Terminal / Payments
      participant AI as AI Job Queue (Operations/Finance Agents)

      Owner->>API: Request Payment Session (Amount, Currency)
      API->>DB: Check Tenant Preferences & Customer Context
      API->>Stripe: Create PaymentIntent & TerminalSession
      Stripe-->>API: Return Session Token
      API-->>Owner: Initialize Tap-to-Pay UI
      Owner->>Stripe: Customer Taps Card (NFC)
      Stripe-->>API: Webhook: Payment Succeeded
      API->>DB: Record Transaction (Multi-Currency)
      API->>AI: Trigger Invoice & Follow-up Job
      AI->>DB: Save Localized Invoice
      AI-->>Owner: Notify: "Payment collected. Receipt sent."
  ```

  ### UI Flow (Mobile-First, 375px)
  1. **Action Request:** On the Work Triage or Customer view, the owner taps a prominent "Collect Payment" button (≥44x44px target).
  2. **Amount & Currency:** A bottom sheet slides up displaying the amount and allowing a quick currency toggle if international.
  3. **Tap-to-Pay Mode:** The screen transitions to a translucent, Apple-style NFC prompt instructing the customer to tap their phone or card.
  4. **Instant Confirmation:** Upon success, a vibrant success token appears. The UI instantly displays a preview of the generated localized invoice/receipt.
  5. **Background Agent Work:** The Assistant automatically sends the invoice via the customer's preferred channel (Email/WhatsApp) without further owner action.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Handles the creation of the PaymentIntent, ensuring the correct multi-currency conversion rates are applied.
  - **Customer & Relationship Assistant:** Drafts and localized the invoice/receipt based on the customer's language and region.
  - **Finance & Decision Assistant:** Logs the transaction into the daily summary and flags any anomalies (e.g., high-value international transaction).

  ### Multi-Tenant & Security Constraints
  - Uses Stripe Checkout/Terminal Sessions with idempotency keys.
  - All database interactions must use row-level tenant isolation (`tenant_id`).
  - Webhooks must verify signatures before triggering AI Job Queue tasks.

  ## 4. Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to implement the "Multi-Currency Tap-to-Pay Terminal & Instant Localized Invoicing" feature for the OHC mobile client.

  **Acceptance Criteria:**
  - Create a mobile-first (375px) Tap-to-Pay flow that initiates a payment session.
  - Ensure the backend securely integrates with Stripe Terminal/Payments API for multi-currency PaymentIntents.
  - Implement a PostgreSQL `SKIP LOCKED` job queue worker that generates a localized invoice upon successful payment webhooks.
  - The UI must display realistic, translucent glass materials (OHC Premium Tokens) and provide clear success/error states.
  - Do not mock the database or API; all data must flow through the real architecture. Ensure E2E tests (Playwright) cover the full CUJ from clicking "Collect Payment" to viewing the generated invoice in the owner's feed.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
