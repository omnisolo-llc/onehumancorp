issue_title: "Agentic Custom Order & Deposit Pipeline Architecture"
issue_description: |
  # Research Report: Agentic Custom Order & Deposit Pipeline

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) handle a large portion of their business via custom requests and negotiations (e.g., custom cake designs via Instagram DMs, or home repair estimates via text). Currently, these owners must manually translate a chat conversation into a formal quote, generate an invoice in a separate tool, request a deposit (partial payment), and track the fulfillment and final payment. This fragmented workflow causes lost leads, delayed payments, and significant manual overhead.

  ## Research Report & Market Discovery (Track 1)
  - **Market Context**: Traditional platforms like Shopify or WooCommerce require third-party plugins (e.g., "Partial.ly" or "Globo Request a Quote") to handle custom pricing and deposits, and these do not integrate seamlessly with customer messaging channels.
  - **Competitor Gaps**:
    - *Shopify*: Excellent for fixed-price catalog items, but highly rigid for conversational commerce and custom pricing with split payments (deposits).
    - *Square*: Offers invoicing with deposits, but lacks an AI assistant to automatically read an Instagram DM, draft the quote, and generate the payment link.
    - *Wix/Squarespace*: Basic invoicing capabilities, but no agentic workflows to handle follow-ups or dynamic adjustments based on chat.
  - **OHC Opportunity**: By combining our Work Triage, Customer & Relationship Assistant, and Sales & Revenue Assistant, OHC can instantly transform a custom request (e.g., "Can you make a vegan cake for 20 people this Saturday?") into a conversational reply containing a fully executable, trackable quote with a one-tap deposit link.

  ## Design Doc: System Architecture (Track 2 & 3)

  ### Architecture & Data Model (PostgreSQL)
  - `Quote`: Represents the customized offer. Fields include `tenant_id`, `customer_id`, `total_amount`, `deposit_amount`, `status` (draft, sent, accepted, declined, expired), and `expiration_date`.
  - `PaymentIntent` (Stripe Integration): Tied to the `Quote` to handle the initial deposit, storing the tokenized payment method for the final balance.
  - `Order`: Generated automatically once the quote is accepted and the deposit is paid, moving the state into the Operations Agent's domain for fulfillment.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Detects high-intent custom requests from DMs, SMS, or web forms, and tags them as "Quote Requested".
  - **Sales Assistant Agent**: Reads the context of the conversation, drafts the quote details (items, price, deposit required based on tenant preferences), and generates a Stripe Payment Link. It presents this draft to the owner for one-tap approval.
  - **Finance Assistant Agent**: Tracks deposit payments, schedules reminders for the remaining balance, and reconciles the payment against the final order.

  ### Mobile UX Flow (375px First)
  1. **Owner Inbox (Triage)**: Owner sees an incoming Instagram DM. The UI displays an AI-suggested action chip: "Draft Quote".
  2. **Quote Generation Screen**: Tapping the chip opens a half-sheet modal. The AI has pre-filled the customer name, requested items, suggested price, and a standard 50% deposit rule.
  3. **Review & Send**: Owner adjusts the price if needed (large touch targets) and taps "Send".
  4. **Customer Experience**: Customer receives a link in their DM. The link opens a highly optimized, mobile-friendly landing page showing the quote details and a one-tap Apple Pay/Google Pay button for the deposit.

  ## Implementation Prompt
  **Feature Name**: Agentic Custom Order & Deposit Pipeline
  **User Persona**: Maya the Baker
  **Target Outcome**: An owner can receive a custom request via message, generate a quote with a deposit requirement using AI, and send it back to the customer within 30 seconds on a mobile device, resulting in a trackable `Quote` entity and a Stripe checkout session.

  **Acceptance Criteria**:
  1. Create the `quotes` table with row-level security for `tenant_id` and strict multi-tenant isolation.
  2. Implement the backend service to generate a quote and a corresponding Stripe deposit checkout session.
  3. Build the mobile-first UI for the owner to review and approve an AI-drafted quote in the chat/inbox view.
  4. Build the customer-facing landing page (edge-cached, highly performant) to view the quote and pay the deposit.
  5. The UI must strictly adhere to the OHC Premium Token library (macOS Translucent Glass, UniFi-style layouts) and be fully responsive starting at 375px.
  6. MUST include full unit test coverage and Playwright E2E tests for the quote creation and deposit payment CUJ.

  ## Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
