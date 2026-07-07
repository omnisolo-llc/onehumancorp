issue_title: "Implement Agent-Driven Autonomous Invoicing & Collections Engine"
issue_description: |
  # Research Report: Agent-Driven Autonomous Invoicing & Collections Engine

  ## Problem Statement
  Service-based and project-based small businesses (such as Nora, the Agency Principal) spend a disproportionate amount of time on the administrative overhead of getting paid. Creating proposals, generating invoices, tracking approvals, and chasing down late payments takes time away from billable work. Existing platforms either provide passive invoicing tools (like QuickBooks or Stripe Billing) that still require manual intervention to follow up, or complex CRMs that are too technical for solopreneurs. When a payment is missed, the awkwardness and manual effort of follow-ups often result in delayed cash flow or uncollected revenue.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Invoicing / QuickBooks:** Provide excellent infrastructure for creating and sending invoices, but they are passive. They can send automated email reminders at set intervals, but they do not dynamically adapt to the client's communication channel (e.g., WhatsApp, Instagram DMs) or tone.
  - **HoneyBook / Dubsado:** Tailored for creative professionals, these offer proposal-to-invoice workflows but lack true agentic autonomy. They rely on rigid, user-configured rule sets (if X days past due, send Y template).
  - **OHC Opportunity:** OHC must integrate the proposal, invoicing, and collections lifecycle directly into the AI assistant workflow. The "Finance Agent" should proactively draft invoices upon project completion, and the "Customer Success Agent" should handle collections contextually—sending a polite WhatsApp nudge instead of a cold automated email if that's where the client relationship lives.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Project / Work Status] -->|Completion Event| B[Finance Agent]
      B -->|Drafts| C[Proposal / Invoice]
      C -->|Pushes to| D[Owner Action Feed 375px]
      D -->|Owner Approves| E[Invoice Sent via Stripe]
      E --> F{Payment Status Tracker}
      F -->|Overdue Event| G[Customer Success Agent]
      G -->|Queries Context| H[Unified Omnichannel Identity]
      G -->|Drafts Reminder| D
      D -->|Owner Approves| I[Omnichannel Dispatcher WhatsApp/Email/DM]
  ```

  ### Mobile UX Flow (375px First)
  - **Feed Notification (Invoicing):** "Project X marked complete. Drafted Invoice #102 for $1,500. Tap to review."
  - **Interaction (Invoicing):** Owner taps card. Sees summary of line items, generated from project tasks. "Approve & Send" or "Edit".
  - **Feed Notification (Collections):** "Client Y is 3 days late on Invoice #101. Drafted a friendly reminder for WhatsApp based on your last chat. Tap to review."
  - **Interaction (Collections):** Owner taps card. Sees AI-drafted message: "Hi Y! Hope the new designs are working well. Just a quick reminder about the final invoice when you get a chance. Here's the link: [Link]". "Approve & Send" or "Edit".

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Listens to operational events (e.g., booking completed, project milestone reached) and automatically generates Draft Invoices in the Stripe integration, syncing back to the OHC feed.
  - **Customer Success Agent (The Ambassador):** Listens to overdue events from the ledger. Instead of a generic email, it uses the LLM to draft a personalized, channel-appropriate nudge (email, SMS, WhatsApp) based on past interaction history.

  ### Key Design Decisions
  - **Zero-Config Reminders:** Remove the need for the owner to set up "Reminder rules". The AI simply knows when a payment is late and drafts the appropriate action.
  - **Channel Fluidity:** Collections don't just happen over email. If the client booked via Instagram DM, the reminder draft is for Instagram DM.
  - **Owner Control:** Critical financial communications are always drafted for approval ("Action Required"), preventing the AI from sending overly aggressive or inappropriate payment demands autonomously.

  ## Implementation Prompt
  **User-Facing Outcome:** As an Agency Principal (Nora), when my team finishes a project, an invoice is already drafted in my feed waiting for 1-tap approval. If a client is late, I don't have to remember to chase them; the app drafts a friendly, personalized follow-up message ready to send on the channel I usually talk to them on.

  **CUJ & Acceptance Criteria:**
  1. Implement a webhook listener or scheduled job that polls the Stripe Integration (or internal ledger) for overdue invoices.
  2. When an invoice becomes overdue, trigger the Customer Success Agent to generate a personalized reminder draft.
  3. The agent must use the customer's unified identity graph to determine the best channel (e.g., if the last 5 messages were via WhatsApp, draft for WhatsApp).
  4. Place the draft in the `Unified Feed` as an "Action Required" card for the tenant owner.
  5. Provide Playwright E2E tests: Simulate an overdue invoice event, verify the drafted reminder card appears in the mobile UI feed, and verify the owner can tap "Approve" to dispatch the message.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []