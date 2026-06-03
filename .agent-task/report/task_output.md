issue_title: "Implement Conversational Checkout & Instant Deposit Engine"
issue_description: |
  # Conversational Checkout & Instant Deposit Engine

  ## Problem Statement
  **The Pain Point:** Business owners like Maya (The Home Baker) and Carlos (The Freelance Handyman) close up to 80% of their business via social media DMs (Instagram, WhatsApp) or text messages. When a customer says "I want the vegan cake for Saturday," the owner currently has to leave the app, manually generate a Stripe or payment link, copy it, paste it back to the customer, and then manually verify if the deposit was paid before adding the booking to their calendar. This multi-step friction causes massive drop-offs and lost revenue.

  **The Goal:** Enable the "Sales & Acquisition" AI Agent to autonomously generate a secure, localized, zero-click checkout card directly inside the DM thread. The moment the deposit is paid, the system must instantly lock the inventory/capacity and notify the business owner.

  ## Research Report
  - **Competitor Systems Audit:**
    - **Shopify Inbox:** Allows sending product links in chat, but still redirects to a full browser checkout flow. Doesn't support service deposits natively.
    - **Stripe Payment Links:** Fast, but lacks deep bidirectional sync with the merchant's live calendar/inventory without custom webhooks.
    - **Meta WhatsApp Business:** Native payments exist (UPI in India, Pix in Brazil), but they are heavily fragmented and not synced with an omnichannel unified inventory mesh.
  - **Identified Gaps:** OHC needs a universal engine that bridges the DM thread, localized payment gateways (Mercado Pago, Stripe), and the `Unified Capacity Mesh`. The AI must construct a transaction state, hold a soft lock on the inventory, and release the checkout card seamlessly.

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Instagram/WhatsApp DM] -->|Message: "I want to book Tuesday"| B(Omnichannel AI Inbox);
      B -->|Intent: Booking| C[Sales & Acquisition AI Agent];
      C -->|Request Capacity Hold| D[Unified Capacity Mesh];
      D -- Soft Lock Granted (15 mins) --> C;
      C -->|Generate Session| E[Conversational Checkout Engine];
      E -->|Create Intent| F[Payment Gateway: Stripe / Mercado Pago];
      E -->|Render Interactive Card| B;
      B -->|Send DM with Deep Link| A;
      F -- Webhook: Deposit Paid --> G[Ledger & Reconciliation];
      G -->|Commit Inventory| D;
      G -->|Notify Operations| H[Operations Agent];
  ```

  ### 2. Data Model & Invariants
  - **CheckoutSession Entity:** Needs fields for `id`, `tenant_id`, `customer_id`, `type` (deposit/full), `amount`, `status` (pending/paid/expired), and `inventory_lock_id`.
  - **Tenant Isolation:** Must be strictly enforced via RLS in Postgres. All webhook processing requires tenant context verification.
  - **Invariants:**
    - A Soft Lock on inventory/capacity must expire strictly after 15 minutes to prevent capacity hogging.
    - The conversational card must deep-link to a native OS payment sheet (Apple Pay / Google Pay / Pix) where supported, falling back to a minimal WebP-optimized webview.

  ### 3. Mobile-First UX Flow & Performance
  - **375px Flow:** The customer taps the checkout bubble directly in the DM. A half-sheet modal slides up displaying the quote, deposit amount, and a one-tap payment button (e.g., "Apple Pay" or "Pix"). No keyboard entry should be required.
  - **Performance Targets:** The checkout sheet modal must render in < 200ms.

  ## Implementation Prompt
  **For Implementer Agent:**
  Please implement the Conversational Checkout & Instant Deposit Engine based on this design.
  - **User-Facing Outcome:** The AI Sales Agent can reply to a customer in WhatsApp/IG with a dynamic checkout bubble. The customer taps it to pay a deposit instantly via Apple Pay/Google Pay/Mercado Pago, which auto-secures their booking.
  - **Acceptance Criteria:**
    1. Create the backend data models for `ConversationalCheckoutSession` and soft-locks.
    2. Integrate with the AI Inbox to trigger checkout generation based on intent.
    3. Provide an E2E test verifying a mock DM flow: Customer requests quote -> AI sends checkout link -> Mock payment webhook -> Inventory is permanently locked.
    4. Ensure strict tenant isolation throughout the flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
