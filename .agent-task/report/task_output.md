issue_title: "[Research] AI Automated Invoice Generation & Collection Workflow"
issue_description: |
  # Research Report: AI Automated Invoice Generation & Collection Workflow

  ## 1. Problem Statement
  For professional services, agencies, and B2B small businesses (e.g., Nora the Agency Principal), managing invoicing is a highly manual, error-prone, and disconnected process. The journey from project completion or milestone achievement to getting paid requires multiple manual steps: drafting the invoice, calculating hours/costs, sending it to the client, tracking views, and sending polite but persistent follow-up emails for late payments. Legacy systems (like QuickBooks or basic Stripe billing) require the owner to drive the entire process, creating friction and delaying cash flow.

  ## 2. Research Report
  - **Market Context:** Most small businesses piece together tools like Harvest (for time tracking), Google Docs/Word (for drafting), and Stripe/PayPal (for payment). Platforms like FreshBooks or Xero automate some recurring invoices, but lack the intelligence to contextually draft invoices based on project milestones or automatically handle bespoke follow-up communications.
  - **Competitor Gaps:**
    - *QuickBooks/Xero:* Excellent for accounting, but not designed as an active "assistant" that drafts invoices based on a project's context or client conversations.
    - *Stripe Billing:* Great infrastructure, but requires the owner to log in, create a customer, and manually input line items. It doesn't write the accompanying email or explain the charges based on recent work.
    - *Bonsai/HoneyBook:* Good for freelancers, but often require heavy initial setup and exist outside of a unified, multi-channel customer memory system.
  - **The OHC Opportunity:** By leveraging the Finance Agent ("The Accountant") and Customer Success Agent ("The Ambassador"), OHC can transform invoicing from a manual task into an autonomous workflow. The system can detect when a project milestone is met, draft an invoice with contextual line items, propose an accompanying email, and manage follow-ups entirely through the Agent Feed for simple owner approval.

  ## 3. Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Project/Milestone Completion Event] -->|Event Bus| B[Finance Agent]
      B -->|Query| C[Customer Graph & Project Context]
      B -->|Generate Line Items| D[Draft Invoice]
      B -->|Notify| E[Customer Success Agent]
      E -->|Draft Email/Message| F[Combined Invoice Proposal]
      F --> G[Agent Feed - Mobile View]
      G -->|Owner Approves| H[Stripe Invoice Creation & Dispatch]
      H -->|Payment Webhook| I[Update Ledger & Notify Owner]
      H -->|Overdue Event| E[Draft Follow-up Message]
  ```

  ### Mobile UX Flow (375px First)
  1. **The Trigger:** Nora marks a project phase as "Complete" in the Operations view.
  2. **The Proposal Card (Agent Feed):** Nora receives a push notification and sees an Action Card in her feed:
     - *Title:* "Draft Invoice Ready: Q3 Website Redesign for Acme Corp."
     - *Content:* Shows the total amount, key line items, and a preview of the drafted email ("Hi team, attached is the invoice for the completion of the design phase...").
  3. **Interaction:**
     - A large, primary "Approve & Send" button.
     - A secondary "Edit Line Items" or "Edit Message" button.
  4. **The Follow-up:** If the invoice goes 3 days past due, another card appears in the feed: "Acme Corp invoice is overdue. Send polite reminder?" with a pre-drafted message.

  ### AI Agent Integration
  - **Finance Agent:** Parses project details to accurately determine line items, amounts, and tax configurations based on tenant settings. Interfaces with Stripe to create the actual invoice object.
  - **Customer Success Agent:** Uses the communication history with the client to adopt the correct tone (formal vs. casual) for the invoice delivery and follow-up messages.

  ## 4. Implementation Prompt
  **Feature Name:** Autonomous Invoicing & Intelligent Follow-ups
  **Target Persona:** Nora the Agency Principal

  **Outcome:** Nora can complete a project and immediately receive an AI-drafted invoice and accompanying email in her Agent Feed. She approves it with one tap, and the system handles delivery and any necessary follow-up reminders.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  1. As Nora, when a project milestone is marked complete (or via a natural language command like "Draft an invoice for Acme for the design work"), the Finance Agent generates a draft invoice.
  2. The draft invoice appears as an actionable card in the Agent Feed on a 375px mobile view.
  3. The card includes the invoice details and an AI-drafted message to the client.
  4. Nora taps "Approve & Send". The system creates a Stripe Invoice (or equivalent), sends the email, and updates the invoice status to "Sent".
  5. (Simulated) When an invoice becomes overdue, a new Action Card appears in the feed suggesting a follow-up reminder.

  **Next Actions for Engineering:**
  1. Create the `InvoiceDraft` and `InvoiceMessageContext` data models.
  2. Implement the Finance Agent capability to generate line items from project/task context.
  3. Build the Mobile UX Action Card for Invoice Approval in the Agent Feed.
  4. Integrate with the existing Stripe/Payment infrastructure to finalize and dispatch the approved invoice.

  **Priority:** P1
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
