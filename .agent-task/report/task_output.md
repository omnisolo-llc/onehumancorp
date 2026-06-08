issue_title: "AI-Driven Invoicing & Autonomous Accounts Receivable Recovery"
issue_description: |
  # AI-Driven Invoicing & Autonomous Accounts Receivable Recovery

  ## Problem Statement
  Small business owners, especially agency principals (like Nora) and independent professionals, spend an inordinate amount of time chasing unpaid invoices and tracking partial payments. Traditional invoicing systems (like QuickBooks, FreshBooks, or simple Stripe invoices) require manual follow-ups, static email reminders, and awkward client conversations about late payments. There is a missing link between the generated invoice and the conversational, agentic recovery of accounts receivable, which impacts cash flow directly.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Billing / Invoicing:** Provides static, scheduled email reminders (e.g., 3, 7, 14 days late). It lacks contextual conversational abilities to negotiate or handle partial payment requests from clients.
  - **QuickBooks / Xero:** Strong accounting backends but rely on the owner to send manual "gentle nudges" or use generic automated templates that feel robotic and often get ignored.
  - **Shopify / Square:** Geared more towards instant Point-of-Sale or checkout rather than delayed B2B/agency invoicing with net-30 terms.
  - **OHC Opportunity:** Utilize the Finance Agent ("The Accountant") and Customer Success Agent ("The Ambassador") in tandem. When an invoice goes past due, instead of a static email, the system drafts a personalized, polite follow-up message using the client's preferred communication channel (Email/SMS/WhatsApp). The AI can even offer a "split payment" link if it detects the client might be struggling, entirely offloading the awkwardness of debt collection from the owner (Nora).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Central Ledger / PostgreSQL] -->|Daily Scan| B(Billing Scheduler)
      B -->|Identifies Past Due| C[Event Mesh]
      C --> D[Finance Agent - The Accountant]
      D -->|Calculates Owed & Options| E[Customer Success Agent - The Ambassador]
      E -->|Drafts Contextual Message| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|Owner Approves| H[Omnichannel Dispatcher]
      H --> I(Client via Email/SMS/WhatsApp)
      I -->|Client Replies 'Need more time'| J[Omnichannel Gateway]
      J --> E
      E -->|Drafts Split Payment Offer| F
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Action Needed: 3 Invoices Past Due".
  - **Interaction:** Tapping the card expands a list. Nora selects the invoice for "Acme Corp".
  - **Context View:** The screen displays the invoice amount, days past due, and a pre-drafted message from the AI: "Hi Acme team, touching base on Invoice #102. If it helps, we can split this into two payments. Let me know!"
  - **Action:** Primary button "Approve & Send". Secondary buttons "Edit" or "Dismiss".
  - **Visual Design:** Clean UniFi-style modular cards with macOS translucent glass styling.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant):** Monitors PostgreSQL ledgers for `status = 'past_due'`. Calculates partial payment options dynamically based on tenant settings.
  - **Customer Success Agent (The Ambassador):** Uses RAG to check past client communication tone. Drafts a polite, personalized follow-up message rather than a cold, generic template.

  ### Key Design Decisions
  - **Human-in-the-Loop for Money:** The AI does not auto-send debt collection messages by default. It drafts them and places them in the owner's feed to maintain trust and relationship control.
  - **Channel Agnostic:** Invoices and follow-ups can be sent via WhatsApp or SMS, not just email, matching where modern small business clients actually communicate.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal, I open my OHC app on Monday morning and see a card that says "Drafted follow-ups for 2 late invoices." I tap it, read the polite, AI-written SMS to my client, and hit "Send." I didn't have to look up the invoice number or type the awkward reminder myself.
  **CUJ & Acceptance Criteria:**
  1. A background job (Billing Scheduler) runs and identifies an invoice in PostgreSQL that is 3 days past due.
  2. An event is emitted and caught by the Finance and Customer Success Agents.
  3. The Ambassador agent queries the client's past interactions and drafts a personalized follow-up message offering a payment link.
  4. The draft is placed in the tenant's `ActionRequiredQueue`.
  5. Playwright E2E Test: A user (Nora) logs into the mobile view, sees the "Past Due Invoice" action card in the feed, taps "Approve", and the system records the dispatch of the reminder message.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []