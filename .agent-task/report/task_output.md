issue_title: "AI-Agentic Financial Insights & Automated Invoice Chaser (The Accountant Agent)"
issue_description: |
  # AI-Agentic Financial Insights & Automated Invoice Chaser

  ## Title
  AI-Agentic Financial Insights & Automated Invoice Chaser (The Accountant Agent)

  ## Problem Statement
  Small business owners, such as Nora (Agency Principal) and Carlos (Field Service Owner), frequently struggle with cash flow management and overdue payments. Traditional tools like QuickBooks, Xero, or standalone invoice software require significant manual data entry, reconciliation, and proactive monitoring to chase down late payments. For an owner-operator, reviewing financial dashboards is often neglected in favor of core operational work. They need an integrated assistant that not only tracks revenue and expenses but autonomously chases overdue invoices, reconciles payments, and provides plain-language daily/weekly financial summaries without requiring technical accounting knowledge.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **QuickBooks / Xero:** Industry standards for accounting, but heavily skewed towards accountants rather than non-technical business owners. They offer basic automated email reminders for invoices, but lack agentic negotiation (e.g., offering a payment plan if a client is struggling) or conversational insights (e.g., asking "Why is cash flow tight this month?").
  - **HoneyBook / Bonsai:** Great for freelancers and offer invoice automation, but they function as a separate silo from the main storefront and inventory operations.
  - **Stripe Billing:** Powerful underlying engine but lacks a personalized, proactive owner-facing assistant to explain what the data means.
  - **OHC Opportunity:** By positioning "The Accountant" (Finance & Decision Assistant) natively alongside operations, OHC can monitor the lifecycle of a project or service—from initial quote to final payment. If an invoice is overdue, the agent can autonomously draft a polite follow-up, or suggest offering a 5% discount for immediate payment. It translates complex ledgers into a simple daily mobile notification: "You have $1,200 coming in today, but 3 clients are late. Tap to send reminders."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Checkout/Invoicing Events] --> B(Stripe Billing Webhooks)
      B --> C[Central PostgreSQL Ledger]
      C --> D{Financial Event Mesh}
      D --> E[The Accountant Agent]
      E -->|Analyze Ledger| F[Insights Engine]
      E -->|Identify Overdue| G[Invoice Chaser Logic]
      F --> H[Owner Agent Feed 375px]
      G --> I[Action Required: Approve Chaser Email]
      I --> H
      H -->|1-Tap Approve| J[Email/SMS Dispatcher]
      J --> K[Client]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Home Feed (Mobile):** The top priority card in the Owner Feed displays: "⚠️ $850 Overdue from 2 Clients".
  2. **Interaction (Tap Card):** Opens a detail view showing the overdue invoices (e.g., "Client: Acme Corp, $500, 14 days late").
  3. **Agent Suggestion:** Below the invoice details, the AI agent provides a pre-drafted message: "Hi Acme Corp, just a gentle reminder that invoice #102 for $500 is now 14 days past due. Let me know if you need another copy of the payment link!"
  4. **Action:** The owner sees a prominent primary button: "Send Reminders". Secondary option: "Edit Draft".
  5. **Visual Design:** Uses the OHC Premium Token library with translucent glass styling, red/yellow warning indicators for overdue amounts, and large, clear typography for monetary values.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant (The Accountant):** Continuously monitors the central ledger and Stripe webhooks. It runs daily crons to identify overdue payments and drafts context-aware reminders based on past client interactions.
  - **Customer Success Agent (The Ambassador):** Collaborates with the Finance agent if a client replies to a chaser email with a dispute or request for a payment plan, drafting an appropriate response for the owner's approval.

  ### Key Design Decisions
  - **Action-Oriented Summaries over Dashboards:** Owners don't have time to stare at charts. Financial data must be translated into explicit, clear actions (e.g., "Approve Reminders", "Pay Vendor").
  - **Unified Ledger:** All financial events (online storefront purchases, in-person POS, and B2B invoices) must feed into the exact same PostgreSQL ledger for a single source of truth.
  - **Zero-Touch Fallback:** If the AI is unsure about the tone to use for a high-value client, it will generate a neutral draft and explicitly flag it for manual review.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal (Nora), I open my OHC app in the morning and see a single card summarizing my cash flow and highlighting two overdue invoices. The app has already drafted polite follow-up emails for both clients. I tap "Send Reminders" and my financial follow-up for the day is complete in 3 seconds.
  **CUJ & Acceptance Criteria:**
  1. A cron job triggers the Accountant Agent to scan the PostgreSQL ledger for `Invoice` records with a status of `OVERDUE`.
  2. For each overdue invoice, the agent generates a personalized reminder email draft using the LLM and the client's context.
  3. The drafts are grouped and presented in the user's mobile feed as an `Action Required` card.
  4. The user can tap a single button to approve and dispatch the emails via the internal notification service.
  5. Provide Playwright E2E tests: A user logs in, sees the overdue invoice card on the 375px mobile feed, taps "Approve," and the system dispatches the drafted emails to the mocked external email service.

  ## Priority
  P1

  ## Estimated Scope
  Medium

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
