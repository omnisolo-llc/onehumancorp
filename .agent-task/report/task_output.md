issue_title: "Implement Agentic Smart Invoicing and Payment Recovery"
issue_description: |
  # Implement Agentic Smart Invoicing and Payment Recovery

  ## Problem Statement
  Small business owners like Nora (agency principal) and Carlos (field service owner) struggle to keep track of unpaid invoices, follow up on late payments, and reconcile payments with their ongoing work context. Managing invoices manually takes time away from actual work. Traditional software either requires clunky manual setup of automated reminders (which are often too robotic) or demands that the owner log into a complex accounting dashboard just to see who hasn't paid. When a client asks a question about an invoice over email or WhatsApp, the owner has to manually cross-reference their invoicing software.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix:** Primarily designed for upfront payment of products or services. Invoicing is a secondary feature and follow-ups are rigid, template-based emails.
  - **Freshbooks / QuickBooks / Square Invoices:** Excellent accounting capabilities, but they are standalone silos. They do not deeply integrate with the owner's conversational inbox (DMs, WhatsApp) or project task list.
  - **OHC Opportunity:** Leverage the "Finance & Decision Assistant" and "Customer Relationships Assistant". The system should autonomously detect overdue invoices, contextually read recent communications with the client (to avoid asking for payment if the client just complained about a broken service), draft a polite, context-aware payment reminder, and present it to the owner in the daily feed as a 1-tap "Approve & Send" action.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Central Ledger / Invoices DB] -->|Daily Scan| B(Finance AI Agent)
      B -->|Overdue Detection| C{Context Resolution}
      C -->|Query| D[Unified Customer Graph DB]
      C -->|Query| E[Recent Communications / DMs]
      C --> F[Draft Reminder]
      F --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I --> J[Email / SMS to Client]
  ```

  ### UI Wireframes & Screen Flow (375px first)
  - **Home Feed Card:** A translucent glass card appears in the owner's "Today" feed.
    - Title: "Invoice #104 is 3 days overdue."
    - Body: "Client (Acme Corp) was last active in WhatsApp yesterday. Drafted a polite reminder."
    - Button 1: "Review & Send" (Primary, highlighted)
    - Button 2: "Snooze"
  - **Review Screen:**
    - Shows the AI-drafted message.
    - Shows the invoice details (amount, due date, link).
    - Owner can tap to edit the message or just hit "Send".

  ### Mobile UX Flow
  1. Owner opens OHC app on their phone.
  2. The first screen (Assistant-First Shell) highlights urgent tasks.
  3. Owner sees the "Overdue Invoice" action card.
  4. Owner taps "Review & Send".
  5. Owner quickly reads the generated draft which includes context from a recent project completion message.
  6. Owner taps "Send" and the card dismisses with a satisfying completion state.

  ### AI Agent Integration Points
  - **Finance Agent:** Monitors the central ledger. When an invoice crosses its due date, it triggers a workflow.
  - **Customer Relationship Agent:** Provides recent context for the client. If the client recently reported a critical issue, the agent flags this to the owner and suggests pausing the reminder.
  - **Task Coordination:** If approved, the system updates the invoice's "last reminded" timestamp.

  ### Key Design Decisions
  - **Human-in-the-Loop:** Automated payment reminders are sensitive. Until the system gains high confidence, reminders are drafted and put in the owner's feed for 1-tap approval rather than sent entirely autonomously.
  - **Context-Aware:** Sending an invoice reminder right after a customer complains is bad business. The AI must cross-reference recent omnichannel messages before drafting.

  ## Implementation Prompt
  **Goal:** Build the Agentic Smart Invoicing and Payment Recovery feature.

  **User Journey:**
  1. The system detects an overdue invoice.
  2. The system drafts a context-aware reminder based on recent customer interactions.
  3. The drafted reminder appears as an actionable card on the owner's mobile feed (375px viewport).
  4. The owner reviews the draft and approves it with one tap.
  5. The reminder is dispatched to the client, and the invoice status is updated.

  **Acceptance Criteria:**
  - Introduce the backend logic to scan for overdue invoices and coordinate with the Customer Relationship Agent to draft a context-aware message.
  - Implement the "Action Required" card in the Flutter frontend, adhering to the mobile-first (375px) constraint and using the OHC Premium Token library (translucent materials).
  - Ensure the feature integrates with the unified inbox context so reminders are not sent inappropriately.
  - Provide complete E2E tests using Playwright simulating an owner approving an overdue invoice reminder.
  - 100% unit test coverage for new backend and frontend logic.
  - Ensure zero mock data in the UI; use real seeded data or create records through real backend APIs.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []