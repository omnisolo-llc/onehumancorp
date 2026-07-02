issue_title: "AI-Driven Dynamic Quoting & Proposal Architecture"
issue_description: |
  # Research Report: AI-Driven Dynamic Quoting & Proposal Architecture

  ## Problem Statement
  Service-based owners (e.g., Carlos the handyman, Nora the agency principal) lose hours each week triaging vague customer requests, calculating estimates based on disjointed historical pricing, drafting proposals, and waiting for approvals. Currently, OHC lacks a unified architecture to turn inbound service requests into accurate, context-aware quotes that can be approved by the owner with a single tap on their mobile device and automatically converted into payment requests by the Operations and Finance AI Agents.

  ## Research Report
  - **Market Dynamics:** Competitors like HoneyBook and Jobber offer robust quoting tools but require heavy manual data entry. Wix and Shopify lack native quoting for bespoke services.
  - **The Gap:** OHC requires an Agentic Quoting Architecture where the "Sales & Revenue Assistant" autonomously parses a customer request (e.g., via IG DM, WhatsApp, or website form), queries the central PostgreSQL ledger for similar past services/pricing rules, drafts a formal quote, and pushes it to the owner for one-tap approval.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Inbound as Multi-Channel Inbox
      participant SalesAgent as Sales & Revenue Assistant
      participant DB as PostgreSQL (Ledger & Pricing Config)
      participant Owner as OHC Mobile App (375px)

      Customer->>Inbound: "I need a quote to fix my leaky sink."
      Inbound->>SalesAgent: Trigger Inbound Request Event
      SalesAgent->>DB: Query historical pricing for "sink repair"
      DB-->>SalesAgent: Return estimated range & materials
      SalesAgent->>DB: Draft `Quote` record (Status: Pending Approval)
      SalesAgent->>Owner: Push Notification: "Quote Drafted: Sink Repair for John"
      Owner->>Owner: Review Quote Card (375px UI)
      Owner->>DB: Tap "Approve & Send"
      DB-->>SalesAgent: Status Updated
      SalesAgent->>Customer: Send interactive Quote / Payment Link
  ```

  ### Mobile UX Flow (375px First)
  1. **Push Notification:** Carlos receives a notification on his Android phone: "New Quote Drafted: Sink Repair for John ($150 - $200)."
  2. **Quote Triage Card:** Opening the app reveals the "Work Triage" feed. The top item is a translucent glass-styled card showing the customer's request summary, the AI's proposed price breakdown (Labor & Materials), and any missing context flagged by the agent.
  3. **One-Tap Action:** Carlos can tap "Approve & Send" directly from the card. If adjustments are needed, tapping the price opens a native numeric keyboard for quick edits.
  4. **Offline Resilience:** If Carlos is in a basement with no cell service, the approval is queued locally (optimistic UI) and synced via the background sync protocol once the connection is restored.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Acts as the primary orchestrator for draft generation. Requires system prompts tuned with the tenant's historical pricing and service catalog.
  - **Operations Assistant:** Automatically provisions calendar availability or a tentative booking slot associated with the draft quote.
  - **Finance Assistant:** Upon customer acceptance, immediately generates a Stripe Payment Intent or Invoice for the deposit.

  ### Key Design Decisions
  - **Immutable Quote Revisions:** Quotes are stored as immutable records in PostgreSQL. Any owner edits create a new revision linked to the parent request to maintain a clear audit trail.
  - **Zero-Config Pricing Models:** Rather than forcing owners to build complex service catalogs, the AI infers pricing guidelines from past finalized invoices and natural language instructions stored in the Knowledge Assistant memory.
  - **Optimistic Concurrency:** Quote approvals use row-level locking or optimistic concurrency control (`version` fields) to prevent the owner and an agent from modifying the same quote simultaneously.

  ## Implementation Prompt

  **User-Facing Outcome:** The owner receives an automatically drafted quote for any incoming service request. The owner can review, edit, and send the quote to the customer with a single tap from their mobile Work Triage feed.

  **Critical User Journey (CUJ):**
  1. Customer requests a quote via the business's contact form.
  2. The Sales & Revenue Assistant drafts the quote and places it in the Work Triage feed.
  3. The owner reviews the quote on their 375px mobile view, edits the price using a numeric keypad, and taps "Approve & Send".
  4. The system sends the finalized quote and payment link to the customer.
  5. The Operations Assistant reserves the tentative booking slot in the calendar.

  **Acceptance Criteria:**
  - The database schema must include `quotes` and `quote_revisions` tables with strict tenant isolation (`tenant_id`).
  - The Sales AI Agent is equipped with tools to query past invoices and draft quotes.
  - The mobile frontend implements the Quote Triage Card using OHC Premium Tokens (translucent materials, clear hierarchy, ≥44x44px touch targets).
  - Full Playwright E2E test verifying the flow from inbound request -> AI drafting -> owner approval -> customer delivery.
  - Unit test coverage is 100% for the new service layer.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []