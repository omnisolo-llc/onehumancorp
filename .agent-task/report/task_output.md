issue_title: "Feature: AI-Powered Autonomous Quote-to-Cash & Deposit Engine"
issue_description: |
  ## Core Architecture Design & Research Report

  ### 1. Problem Statement & Architectural Gap
  Currently, OneHumanCorp (OHC) lacks a unified, agent-driven Quote-to-Cash (Q2C) and Deposit capability that bridges Work Intake (DMs, calls), Sales (proposals, quotes), Operations (bookings/tasks), and Finance (invoicing, deposits).

  **Personas Affected**:
  - **Maya (Home Baker)**: Needs custom cake deposits to confirm orders after an Instagram DM negotiation.
  - **Carlos (Field Service)**: Needs to send on-site estimates and require a scheduling deposit before dispatching a truck.
  - **Nora (Agency Principal)**: Needs proposal drafting and milestone invoice generation directly from project intake conversations.

  **Competitor Analysis**:
  Legacy platforms like Jobber, HoneyBook, and Stripe require owners to manually open a form, fill in line items, calculate deposits, and copy-paste links. In OHC, this workflow must be inverted: the AI assistant reads the intake conversation, drafts the quote autonomously, and presents a 1-tap "Approve & Send" card on the owner's mobile feed.

  ### 2. Proposed System Architecture

  #### Business Journey Mapping
  1. **Acquisition/Intake**: Customer requests custom work via IG DM / Web form.
  2. **Triage & Draft**: AI Work Triage identifies "quote request" intent. The Sales Assistant drafts a `Quote` based on pricing memory.
  3. **Owner Approval (Activation)**: Owner sees a prioritized card on their 375px mobile screen: "Draft Quote: $450 Wedding Cake for Sarah. 50% deposit required." Owner taps "Approve & Send."
  4. **Customer Payment (Revenue)**: Customer receives a highly-optimized mobile web link (Stripe Checkout) and pays the deposit.
  5. **Operational Handoff**: Finance Assistant confirms the deposit via webhook; Operations Assistant automatically blocks the calendar and creates a fulfillment task.

  #### Data Model & Invariants
  - **Multi-Tenant Isolation**: `tenant_id` must be indexed and enforced via PostgreSQL Row Level Security (RLS) on all tables.
  - **Entities**:
    - `quotes` (id, tenant_id, customer_id, status: draft | sent | accepted | paid, total_amount, deposit_amount, valid_until)
    - `quote_line_items` (id, quote_id, tenant_id, description, quantity, unit_price)
    - `payments` (id, quote_id, tenant_id, stripe_intent_id, amount, type: deposit | full)

  #### AI Department Coordination
  - **Work Triage**: Listens to the Unified Inbox, triggers `DraftQuote` intent.
  - **Sales Assistant**: Executes `GenerateQuoteDraft` function based on context.
  - **Finance Assistant**: Provisions Stripe Payment Links and handles idempotent webhook reconciliation.
  - **Operations Assistant**: Subscribes to `QuotePaid` domain events and creates corresponding `WorkTask` and `CalendarEvent` entities.

  ### 3. Mobile-First UX & Technical Integrity

  - **Mobile UX Flow (375px)**:
    - The "Work Feed" displays actionable translucent glass cards.
    - Tapping a "Review Quote" card slides up a bottom sheet (Native feel) displaying the line items, required deposit, and a prominent primary action button.
    - The interface must never require horizontal scrolling. Native numpads must be invoked for any manual price overrides.
  - **Offline/Network Resilience**: The "Approve" action should use optimistic UI updates and queue a background sync task in case the operator is in a low-connectivity zone.
  - **Security**: The customer-facing quote page must be a stateless, edge-cached dynamic route with a signed token to prevent enumeration attacks.

  ### 4. Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the AI-Powered Quote-to-Cash & Deposit Engine end-to-end.

  **Acceptance Criteria**:
  1. **Database & Schema**: Create the `quotes`, `quote_line_items`, and `payments` tables with strict `tenant_id` RLS policies. Include necessary indices.
  2. **AI Tool Integration**: Add a `draft_quote` tool to the Sales Assistant's system prompt and capability set, allowing it to generate a quote payload from chat context.
  3. **API & Services Layer**: Implement the internal gRPC/REST endpoints for the frontend to fetch pending drafts, approve/reject them, and generate a Stripe Checkout session for the deposit.
  4. **Frontend UI (Flutter/PWA)**: Implement the mobile-first (375px) "Draft Quote" actionable card in the main feed, and the review bottom sheet using the OHC Premium Token translucent design system.
  5. **E2E Testing**: Write a Playwright E2E test starting from a logged-in owner, reviewing a draft quote, approving it, and verifying the customer link generation. All UI elements must strictly match the action (no dead links).
  6. **Test Coverage**: 100% unit test coverage on the newly created quote service modules.

  **Superpowers Note**: Load the relevant design superpowers (macOS translucent glass, mobile-first layouts) before building the frontend. Do not use any mock data in the UI.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
