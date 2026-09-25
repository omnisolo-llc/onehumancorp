issue_title: "Authentic Provider Checkout Sessions & Idempotent Retry Architecture (F08)"
issue_description: |
  **Title**: Authentic Provider Checkout Sessions & Idempotent Retry Architecture (F08)

  **Problem Statement**:
  The current invoice and booking systems fabricate checkout-looking URLs, misleading users into believing a real payment session is active. From a non-technical owner/operator's perspective, this means payments can fail without warning, reconciliation is impossible, and money can be lost because real payment gateway IDs are not generated, persisted, or verified. This breaks trust and safe execution boundaries.

  **Research Report**:
  Based on the `business_capability_and_usage_economics_audit.md` and codebase analysis, we observe that `src/server/api/invoice.rs` and related booking helpers are omitting real provider integrations for payment links, instead surfacing unverified URLs. The remediation requires generating real provider sessions (e.g., Stripe Checkout Sessions), persisting these provider IDs locally for reconciliation, and handling the "pending/unavailable" states explicitly. Competitors like Shopify, Wix, and Squarespace enforce strict server-side checkout session creation before presenting URLs to users to guarantee safe funds capture.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
        actor User as Customer
        participant OHC as OmniSolo UI
        participant API as OHC API
        participant DB as Postgres Ledger
        participant Stripe as Payment Provider

        User->>OHC: Clicks "Pay Invoice"
        OHC->>API: Request Payment Link for Invoice ID
        API->>DB: Check for existing unexpired Provider Session
        alt Session exists & valid
            DB-->>API: Return existing Session URL
        else No session or expired
            API->>Stripe: Create Checkout Session (Idempotent Key)
            Stripe-->>API: Return Checkout URL & Session ID
            API->>DB: Persist Session ID, Amount, Expiry, Status = 'pending'
        end
        API-->>OHC: Return Checkout URL
        OHC-->>User: Redirect to Provider Checkout
    ```

  - **UI wireframes or screen flow description (375px first)**:
    - Mobile View (375px): On the "Invoice Details" screen, instead of a static fake link, show a loading state "Generating secure payment link..." upon tapping "Pay Now". If the provider is unavailable, present a clear, actionable error: "Payment gateway currently unavailable. Please try again later." with an option to refresh.
    - The UI must adopt OHC Premium Design Standards (macOS-style Translucent Glass materials): `background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`, `border: 1px solid rgba(255, 255, 255, 0.4)`. Button radius is `8px`, containers `16px`. Fluid motion transitions `cubic-bezier(0.4, 0, 0.2, 1)`.

  - **Mobile UX flow**:
    - Invoice Received -> Tap Pay -> Fluid Spinner (API call to Stripe) -> Redirect to Stripe Mobile Checkout -> Success Return to OHC "Payment Received" confirmation screen.

  - **AI agent integration points**:
    - The `invoice_followup_worker` must check the real payment session status before sending follow-up emails. If a user is actively on the checkout page or the session is marked pending payment verification, delay the follow-up.

  - **Key design decisions and why**:
    - Late generation of checkout URLs: Generating the URL only when the user intends to pay prevents creating stale or abandoned checkout sessions in the provider's system.
    - Idempotency Keys: Crucial to ensure that rapid double-taps by users on mobile networks do not create duplicate provider sessions or double charges.

  **Implementation Prompt**:
  Implement real provider session generation for invoices to replace fabricated URLs. Update the invoice API and domain logic to ensure that when a user requests a payment link, an idempotent call is made to the payment provider (e.g., Stripe) to create a checkout session. Persist the generated provider session ID, validate the money amount before creation, and handle provider unavailability by returning an explicit pending or unavailable state to the UI. Ensure the UI safely reflects these states with appropriate glassmorphism styling and mobile-first responsiveness. Do not prescribe specific database schemas or API signatures; design them to fit the existing Rust/Postgres architecture.

  **Priority**: P0

  **Estimated Scope**: Medium

  **Strategy Admission**:
  - Stable OHC target ID: F08
  - Selected customer/stage: Days 15-45 (Booking/deposit/final payment, OHC-03-08 slices)
  - Evidence level: Codebase inspection and audit ledger findings
  - Baseline/result metric: 0 fabricated links generated / 100% real provider sessions
  - Dependencies/reuse: Existing Stripe client integration
  - Non-goals: Building a custom payment gateway or reinventing Stripe Checkout
  - Authority class: Owner-authorized payment collection
  - Cost plan: Uses existing Stripe integration without extra AI compute cost
  - Happy/failure-path acceptance checks: Verify idempotent checkout creation succeeds, verify provider unavailability fails gracefully without fabricating a URL.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
