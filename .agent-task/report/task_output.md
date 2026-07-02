issue_title: "Agentic Automated Invoicing & Cash Flow Management"
issue_description: |
  # Research Report: Agentic Automated Invoicing & Cash Flow Management

  ## Problem Statement
  Small business owners and agency principals (like Nora) struggle with cash flow management because creating, tracking, and following up on invoices is a manual, disjointed process. Traditional platforms (like QuickBooks or FreshBooks) require extensive manual entry and often lack deep integration with the core operations and customer relationship management systems. As a result, invoices are delayed, reminders are forgotten, and owners lack real-time visibility into their cash flow.

  ## Research Report
  **Market Mapping & Competitor Discovery:**
  - **Shopify/Wix:** Primarily focus on B2C e-commerce and immediate checkout. Their invoicing tools are basic and not designed for milestone-based or contractor/agency workflows.
  - **QuickBooks/FreshBooks/Xero:** Robust accounting tools but complex and disconnected from the day-to-day operational workflow. They are not mobile-first and lack agentic automation (e.g., they won't automatically draft an invoice based on project completion in a project management tool).
  - **Stripe Invoicing:** Powerful API but the dashboard requires technical understanding or manual intervention.
  - **OHC Opportunity:** Leverage the "Finance Agent" and "Operations Agent" to automatically translate completed tasks, approved proposals, or delivered services into drafted invoices. The owner simply receives an "Approve Invoice" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Project/Task Completion Event] -->|Webhook| B(Event Mesh)
      C[Proposal Approval Event] -->|Webhook| B
      B --> D[Finance Agent]
      D -->|Fetch Pricing & Customer Data| E[(PostgreSQL Central Ledger)]
      D -->|Draft Invoice| F[Stripe Invoicing API]
      F --> G[Draft Invoice Created]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Stripe Finalize & Send]
      J --> K[Customer Receives Invoice]
      J --> L[Finance Agent: Schedule Reminders]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A prominent card appears: "Project 'Website Redesign' marked complete. Invoice #1024 for $2,500 drafted."
  - **Interaction:** Tapping the card expands to show a summary of the invoice (line items, total, customer details).
  - **Action:** A massive "Approve & Send" primary button and a secondary "Edit" button.
  - **Visual Design:** Translucent Glass materials, clear status indicators (Draft, Sent, Paid).

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Triggers on operational milestones. Automatically drafts invoices via Stripe API. Monitors payment status and drafts follow-up reminder emails if payment is overdue, presenting them in the feed for approval.
  - **Operations Agent ("The Manager"):** Signals task/project completion to the Event Mesh.

  ### Key Design Decisions and Why
  - **Stripe Invoicing as the Engine:** Avoids building a billing engine from scratch. Stripe handles tax calculation, local currency formatting, and compliance.
  - **Event-Driven Drafting:** Invoices must be drafted automatically based on real-world actions (e.g., Nora marking a project as complete), eliminating manual data entry.
  - **Approval Gate:** The owner retains full control. No invoice is sent without explicit approval via the mobile feed, ensuring trust in the system.

  ## Implementation Prompt
  **User-Facing Outcome:** When a project or service is marked as complete, the owner receives a notification in their unified feed to review and send a pre-drafted invoice.

  **CUJ:**
  1. Nora (Agency Principal) marks a project milestone "Phase 1 Complete" in her OHC mobile app.
  2. The system captures this event and the Finance Agent automatically drafts a Stripe Invoice based on the agreed proposal terms.
  3. A card appears in Nora's mobile feed: "Draft Invoice ready for Phase 1".
  4. Nora taps "Approve & Send".
  5. The invoice is finalized via Stripe and sent to the client.

  **Acceptance Criteria:**
  - Implement an event listener for `project_milestone_completed`.
  - Connect the Finance Agent to the Stripe Invoicing API to create a `draft` invoice upon event trigger.
  - Render an actionable card in the Unified Agent Feed for the drafted invoice.
  - Implement the "Approve" action to finalize and send the invoice via Stripe.
  - Ensure the feed card is fully responsive and optimized for a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
