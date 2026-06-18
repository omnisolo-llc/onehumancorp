issue_title: "The Finance Agent: Autonomous Accounts Receivable & Invoice Recovery"
issue_description: |
  # Research Report: The Finance Agent & Accounts Receivable Automation

  ## Executive Summary
  This report investigates the architectural gap in OneHumanCorp (OHC) concerning post-service revenue collection and accounts receivable (AR). For service-based owners and agency principals, following up on unpaid invoices is a high-friction, time-consuming task. Traditional software sends generic, robotic reminders. We propose "The Finance Agent" (The Collector) — an autonomous department that monitors the ledger, understands the customer relationship context, and drafts personalized follow-ups for the owner to approve with one tap.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **QuickBooks / FreshBooks:** Provide standard invoice aging reports and rigid auto-reminders (e.g., "Invoice #123 is Overdue"). They lack contextual awareness of recent customer conversations.
  - **Stripe Billing:** Excellent for subscriptions and payment links but doesn't handle relationship-aware communication.
  - **The Gap:** Small business owners (like Nora and Carlos) want to preserve their client relationships while ensuring they get paid. They delay sending manual follow-ups because it feels awkward or they forget. An agentic solution that drafts a polite, context-aware nudge based on recent project interactions bridges this gap.

  ## 2. Deep Dive Architecture Design (Track 2)
  - **Data Model:**
    - `Ledger` / `Invoices` table in PostgreSQL with multi-tenant row-level security (`tenant_id`).
    - `PaymentIntents` tracking partial deposits vs. final balances.
  - **Event & Job Orchestration:**
    - A daily AR aging job (using PostgreSQL `SKIP LOCKED` job queue) scans for invoices matching late criteria (e.g., 3 days, 15 days, 30 days overdue).
  - **AI Coordination (The Finance Agent):**
    - The Finance Agent receives the overdue event.
    - It queries the "Customer Knowledge Base" (recent emails, meeting notes, project statuses via RAG) to ensure we don't send a harsh reminder if the client complained about an issue yesterday.
    - It generates a drafted SMS/email message with a personalized tone and an embedded Stripe Payment Link.
  - **Mobile-First UX Flow:**
    - A 375px Action Card appears in the owner's Agent Feed: "Drafted Follow-up for Client X ($1,200 overdue)".
    - Owner taps to view the draft: "Hi Client X, it was great wrapping up the design phase last week! Just a friendly reminder that the final invoice is due..."
    - The owner can "Approve & Send", "Edit", or "Snooze".

  ## 3. Mobile Parity & Security Integrity (Track 3)
  - Ensure the "Finance Feed" renders flawlessly on mobile without horizontal scrolling.
  - Action cards must have clear, high-contrast touch targets (44x44px min).
  - Stripe Checkout Links must be securely generated server-side using the `tenant_id` context.

  ## 4. Implementation Prompt
  **Target Personas:** Nora (Agency Principal) and Carlos (Field Service Owner).

  **Outcome:** The Finance Agent proactively monitors unpaid invoices, synthesizes recent customer interactions, and drafts contextually appropriate payment reminders pushed to the owner's mobile feed for one-tap approval.

  **Critical User Journey (CUJ):**
  1. Nora logs into the OHC mobile app.
  2. Her Agent Feed surfaces an Action Card: "Action Needed: 2 Invoices Overdue".
  3. Nora taps the card and sees a drafted email to her client: "Hi [Client], checking in on the $2,000 balance from our project last month. Here is a quick link to settle it via card or ACH."
  4. Nora taps "Approve & Send". The message is sent via the unified inbox and the invoice status is updated to "Follow-up Sent".
  5. The Stripe webhook later receives the payment and the Finance Agent resolves the ticket automatically, pushing a success notification: "$2,000 paid by [Client]."

  **Next Actions for Engineering:**
  - Implement the daily AR aging job queue in PostgreSQL.
  - Integrate the Finance Agent prompt flow with Gemini Pro, injecting invoice details and recent customer interactions.
  - Build the 375px mobile UI card for "Invoice Follow-up Draft" featuring "Approve", "Edit", and "Discard" actions.
  - Wire the approval button to dispatch the message via the existing customer communication channels.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []