issue_title: "OHC Autonomous Quoting & Milestone Billing Architecture"
issue_description: |
  # Architecture Design: OHC Autonomous Quoting & Milestone Billing Engine

  ## Problem Statement
  Service-based owner/operators—like Carlos (Field Service Owner) and Nora (Agency Principal)—lose significant time and momentum in the gap between "customer inquiry" and "paid deposit." Creating accurate quotes, drafting proposals, tracking approvals, and following up on split payments (deposit vs. final) traditionally require disjointed tools (CRM, Word/Docs, email, invoicing software) and manual data entry. Legacy tools like Joist or Jobber are forms-heavy and desktop-oriented.

  From the owner’s perspective, the "App Tax" of piecing together scheduling, quoting, and invoicing software creates friction, leading to lost leads and delayed cash flow. OHC must bridge this gap by transforming an incoming raw inquiry into a structured, executable, and payable quote natively within the platform, handled autonomously by an AI Sales/Operations Agent.

  ## Research Report
  **Competitive Analysis:**
  - **Jobber / Housecall Pro:** Excellent for field services but require manual line-item entry for every quote. Not agentic.
  - **HoneyBook / Dubsado:** Geared towards creatives. High setup friction and heavily rely on pre-built templates rather than dynamic parsing of the customer's intent.
  - **Shopify:** Primarily built for physical products. Custom quoting and milestone billing require expensive third-party apps and clumsy workarounds.

  **Market Needs (Persona Evidence):**
  - **Carlos (Handyman):** Receives a text/DM saying "Need my fence fixed, 20ft long." He needs an agent to instantly parse the length, calculate an estimated material/labor cost based on his stored rates, and draft a professional proposal he can approve with one tap on his Android phone.
  - **Nora (Agency Principal):** Needs to send a $10k design proposal structured as 50% deposit, 25% milestone, 25% final, with a built-in approval signature and auto-reminders.

  **The OHC Differentiator:**
  Zero-data-entry quoting. The OHC Sales Agent parses the work intake context (chat history, forms), structures the cost data, and generates an interactive, mobile-optimized (375px) proposal link. The system natively handles the deposit payment intent and schedules the remainder as a background job.

  ## Design Doc

  **Data Model & Multi-Tenancy (PostgreSQL):**
  - **`quotes`**: `tenant_id`, `quote_id`, `customer_id`, `status` (draft, sent, approved, declined), `expires_at`, `total_amount`.
  - **`quote_line_items`**: `tenant_id`, `line_item_id`, `quote_id`, `description`, `quantity`, `unit_price`, `is_optional`.
  - **`milestone_payments`**: `tenant_id`, `milestone_id`, `quote_id`, `percentage`, `amount`, `status` (pending, paid), `due_condition` (e.g., 'on_approval', 'on_completion').
  *All tables enforce row-level security (`ENABLE ROW LEVEL SECURITY`) keyed on `tenant_id`.*

  **AI Agent Department Coordination:**
  - **Work Triage / Intake:** Captures the raw customer request (e.g., via Instagram DM integration) and forwards it to the Sales Agent.
  - **Sales & Revenue Agent:** Uses RAG against the owner's past quotes, standard pricing, and material costs to generate `quote_line_items`. Drafts the proposal copy and proposes milestone splits.
  - **Finance Agent:** Upon customer approval, instantly generates the Stripe Checkout Session for the deposit (`on_approval` milestone). Schedules an automated follow-up for the remaining balance.

  **Architecture Diagram (Mermaid):**
  ```mermaid
  sequenceDiagram
      autonumber
      participant C as Customer (Browser/Mobile)
      participant T as OHC Work Triage
      participant SA as Sales Agent
      participant O as Owner (OHC App - 375px)
      participant DB as PG Central Ledger
      participant F as Finance Agent
      participant S as Stripe

      C->>T: "I need a 20ft fence repaired."
      T->>SA: Forward raw intent & context
      SA->>DB: Query standard rates & past fences
      SA->>DB: Draft Quote (Line items, 50% deposit)
      SA->>O: Push "Quote Ready for Review" to Agent Feed
      O->>SA: Taps [Approve & Send]
      SA->>C: Send interactive Quote Link (SMS/Email)
      C->>SA: Reviews and taps "Accept"
      SA->>F: Trigger Deposit Collection
      F->>S: Create Checkout Session (50%)
      S-->>C: Prompt Payment
      C->>S: Pays Deposit
      S-->>F: Webhook (payment_intent.succeeded)
      F->>DB: Mark Milestone 1 Paid, Update Ledger
      F->>O: Push "Deposit Received, Work Ready" to Feed
  ```

  **Mobile UX Flow (375px Constraint):**
  1. **Owner View (The Agent Feed):** Carlos sees a card: *"Drafted quote for John: 20ft Fence Repair ($850). 50% Deposit required."*
  2. **Detail View:** Tapping the card opens a vertical, translucent glass-styled summary of line items. Touch targets for editing numbers or adjusting the deposit split are ≥ 44x44px.
  3. **Action:** A sticky bottom bar contains a massive primary button: `[Approve & Send to Customer]`.
  4. **Customer View:** The customer receives a responsive web link. The quote appears as a conversational UI or clean, modern card layout. They tap `[Accept & Pay $425 Deposit]`, invoking Apple Pay / Google Pay directly without creating an account.

  **Zero Trust & Security:**
  - Quote generation is scoped strictly to the `tenant_id`. The Sales Agent's LLM context window is populated *only* with the specific tenant's historical pricing data to prevent cross-tenant data leakage.
  - Stripe webhook payloads are verified cryptographically before mutating `milestone_payments` status.

  ## Implementation Prompt
  Implement the "Autonomous Quoting & Milestone Billing" backend service and corresponding Mobile-First UX.

  **User-Facing Outcome:** An owner receives a customer request, the AI drafts a multi-line-item quote with a deposit requirement, and the owner approves it with one tap. The customer pays the deposit via a generated link, and the system automatically tracks the remaining balance.

  **Critical User Journey (CUJ):**
  1. Login as Carlos (Field Service Owner).
  2. Navigate to the Work Triage or Agent Feed view on a 375px viewport.
  3. Trigger the creation of a new quote from a mock customer message ("Need a roof repair").
  4. The system presents the AI-drafted quote card with line items and a 50% deposit milestone.
  5. Carlos taps [Approve & Send].
  6. Verify that the quote is persisted in the database with the correct `tenant_id` and milestone structures.
  7. Simulate the customer accepting the quote and the Stripe deposit webhook firing.
  8. Verify the Finance Agent updates the invoice status to "Deposit Paid, 50% Remaining" and pushes a notification card to Carlos's feed.

  **Acceptance Criteria:**
  - Implement PostgreSQL migrations for `quotes`, `quote_line_items`, and `milestone_payments` with `tenant_id` RLS policies.
  - Implement the gRPC/REST endpoints for creating, retrieving, and updating quotes.
  - Implement the UI components in Flutter (375px mobile-first) using OHC Premium Tokens (Glassmorphism, strong typography).
  - Include an AI helper function (mocked or real LLM call) that transforms natural language intent into structured quote JSON.
  - Write Playwright E2E tests verifying the full flow: from quote generation to approval to deposit payment status update.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
