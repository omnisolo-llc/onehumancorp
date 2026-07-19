issue_title: "Autonomous Agent-Driven Invoicing & Receivables Management"
issue_description: |
  # Autonomous Agent-Driven Invoicing & Receivables Management

  ## Problem Statement
  Service-based businesses and agency principals (e.g., Nora the Agency Principal) spend a disproportionate amount of time tracking project milestones, drafting invoices, and chasing late payments. Traditional tools (like FreshBooks or QuickBooks) provide the mechanisms to send invoices, but they rely entirely on the owner to remember *when* to bill and *whom* to follow up with. This reactive process causes cash flow delays and creates an uncomfortable administrative burden for operators who want to focus on their core work rather than "playing accountant."

  ## Research Report
  - **Market Context**: Platforms like Shopify handle immediate checkout well, but B2B and service scenarios often require milestone-based or net-30 invoicing. Traditional accounting SaaS (Xero, QuickBooks) are robust but passive; they wait for user input.
  - **The OHC Opportunity**: OHC's unique value proposition is the "Invisible AI Automation." By deeply integrating the Finance Agent and Operations Agent, OHC can shift invoicing from a manual task to a proactive, agent-managed workflow.
  - **Competitor Gaps**:
    - *QuickBooks/Xero*: Passive tools. High learning curve. No agentic follow-up based on project context.
    - *Stripe Billing*: Powerful APIs, but requires the owner to initiate billing cycles manually unless strict subscriptions are used.
    - *HoneyBook/Dubsado*: Good for freelancers, but workflows are static and rules-based (e.g., "send email 3 days after X"), rather than context-aware (e.g., "The client approved the design draft in the project feed, I should generate the 50% milestone invoice").

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Project Milestone Approved] -->|Event| B(Event Mesh)
      C[Time Tracking / Deliverable Complete] -->|Event| B
      B --> D[Finance Agent - The Accountant]
      D -->|Query| E[Unified Customer & Project Graph]
      E -->|Return Context| D
      D -->|Draft Invoice| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|Owner Taps 'Approve'| H[Stripe Integration]
      H -->|Send Invoice| I[Client]
      I -->|Unpaid after 7 days| B
      D -->|Draft Reminder| F
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner View (Dashboard)**: Nora opens her OHC app. At the top of her unified agent feed is an Action Card: *"Project Alpha milestone reached. Drafted $2,500 invoice for ACME Corp."*
  2. **Interaction**: Nora taps the card. She sees a clean, Glassmorphism summary of the invoice items (auto-populated by the Operations Agent's knowledge of the project).
  3. **Action**: A prominent "Approve & Send" button (min 44x44px).
  4. **Receivables Follow-up**: If an invoice is overdue, the Finance Agent drafts a polite follow-up email. Nora sees an Action Card: *"ACME Corp is 3 days late on $2,500. Send reminder?"* with a 1-tap "Send" button.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager)**: Tracks project status and signals when billable milestones are reached.
  - **Finance Agent (The Accountant)**: Receives the signal, generates the line items based on the original proposal/quote, and drafts the invoice. It also monitors Stripe webhooks for payment status and drafts follow-up emails for overdue accounts.

  ### Key Design Decisions
  - **Proactive Drafting**: The system generates the invoice *before* the user realizes it's time to bill.
  - **Context-Aware Reminders**: Follow-ups are drafted using the specific context of the client relationship, not just a generic "Your bill is overdue" template.

  ## Implementation Prompt
  **Target Persona**: Nora (Agency Principal)
  **User-Facing Outcome**: Nora finishes a design project. Instead of opening an accounting app to manually create an invoice, she receives a push notification from OHC: "Design phase complete. Invoice drafted for $1,500. Tap to send." She taps approve, and the invoice is sent via Stripe. If the client hasn't paid in 7 days, she gets another notification offering to send a drafted, polite reminder.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A project milestone is marked as complete in the backend (simulated via API or internal service event).
  2. The Finance Agent catches this event, queries the original proposal/project data, and drafts an invoice.
  3. The draft appears as a high-priority card in the 375px Mobile Agent Feed.
  4. The user taps "Approve", which triggers a Stripe Invoice creation and sends it to the client.
  5. Provide a Playwright E2E test verifying the full flow: user logs in, sees the drafted invoice card, approves it, and the system registers the "sent" state.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []