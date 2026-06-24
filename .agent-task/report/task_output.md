issue_title: "Autonomous Invoice Follow-Up and Reconciliation Agent"
issue_description: |
  # Research Report: Autonomous Invoice Follow-Up and Reconciliation

  ## Problem Statement
  For professional service providers and agency principals (e.g., Nora), managing client invoices is a huge source of friction and administrative overhead. They create the invoice manually, send it over email, and then repeatedly chase clients who forget or delay payments. Current platforms only offer rudimentary features—like sending an automated email exactly on the due date—but fail to adapt to complex follow-up needs. There's no AI to handle polite but firm email follow-ups, answer client questions about line items autonomously, and auto-reconcile once payment arrives.

  ## Research Report
  - **Traditional Methods (Shopify/Wix):** While they support basic recurring billing or simple payment reminders, they aren't designed for service businesses dealing in net-30 custom invoices with milestone tracking. The "chase" is manual.
  - **Specialized Tools (Freshbooks, QuickBooks):** Better for accounting but act strictly as systems of record. They send templated automated reminders, but they don't negotiate or respond intelligently to client queries like "Can I pay half now and half next week?".
  - **The OHC Opportunity:** Introduce the "Finance Agent" (or "The Accountant"). It continuously tracks open invoices. Instead of a dumb cron job, it drafts personalized, context-aware reminders based on the client relationship. If a client replies to the reminder with a question or payment arrangement request, the Ambassador agent handles the negotiation directly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Invoice Created in Ledger] -->|State: Draft| B[Finance Agent]
      B --> C[Send Initial Invoice]
      C --> D{Wait for Payment}
      D -->|Unpaid at 7 days| E[Draft Polite Reminder]
      E --> F[Owner Approval via Mobile Feed]
      F -->|Approve| G[Send Reminder]
      G --> H{Client Reply?}
      H -->|Yes| I[Ambassador Agent: Contextual Response]
      H -->|No| J[Escalation Path]
      D -->|Paid via Stripe| K[Webhook Received]
      K --> L[Auto-Reconcile Ledger]
      L --> M[Notify Owner]
  ```

  ### Mobile UX Flow (375px First)
  1. **Feed Notification:** Nora sees a card in her Agent Feed: "Acme Corp invoice is 3 days overdue. The Accountant has drafted a polite reminder."
  2. **Card Expansion:** She taps the card to see the drafted message. It's not a generic template, but refers to the recent project completion and asks if they need another copy of the invoice.
  3. **Action Buttons:** Large touch targets: "Approve & Send" or "Edit".
  4. **Client Reply Handling:** If the client replies, it routes to her unified inbox where the Ambassador agent has already drafted a response based on the invoice terms.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Monitors the PostgreSQL Ledger. Calculates delays, risk, and drafts reminders using RAG over the specific client's history.
  - **Customer Success Agent (The Ambassador):** Intercepts email replies related to the invoice. Understands intents like "I lost the link" or "Can I pay next Friday?" and proposes the correct action.

  ## Implementation Prompt
  **Feature Name:** Autonomous Invoice Follow-Up Workflow
  **Target Persona:** Nora the Agency Principal

  **Outcome:** Nora creates an invoice for a completed design project. She never has to manually check if it was paid or draft an awkward follow-up email. The Finance Agent monitors the status and proposes drafted, contextual follow-ups in her mobile feed until the invoice is settled.

  **Next Actions for Engineering:**
  1. Build a scheduled background worker (using the existing `TaskQueue`) that sweeps the `invoices` table for overdue states.
  2. Integrate the Finance Agent to generate contextual reminder drafts based on the `Invoice` and `Customer` records.
  3. Plumb these drafts into the unified Agent Feed for 1-tap owner approval.
  4. Ensure the Stripe Webhook handler correctly updates the invoice status to 'paid' and halts any further follow-ups.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
