issue_title: "Implement Agentic Context-Aware Invoice Chasing & Cash Flow Prediction"
issue_description: |
  # Research Report: Agentic Context-Aware Invoice Chasing & Cash Flow Prediction

  ## Problem Statement
  Service operators and agency principals (like Carlos the handyman and Nora the agency principal) spend hours every week chasing unpaid invoices, deposits, and late payments. This administrative burden is not only time-consuming but also socially awkward, often causing friction with clients. Traditional accounting tools (like Xero, QuickBooks, or HoneyBook) offer static, rigid email reminders based strictly on due dates. These systems lack conversational context—if a client emailed two days ago saying "I'm waiting on a check and will pay this Friday," the traditional system will still blast an aggressive "Invoice Overdue" email on Wednesday, damaging the client relationship. Small business owners need an intelligent, context-aware financial assistant that handles collections gracefully.

  ## Research Report
  **Market Findings & Competitive Analysis:**
  - **QuickBooks / Xero:** Industry standards for accounting, but their "automated reminders" are purely rules-based (e.g., "send 3 days after due date"). They have zero awareness of ongoing customer communication in external channels.
  - **HoneyBook / Dubsado:** Popular among freelancers and agencies. They offer workflows and templates, but the owner must manually pause reminders if a client requests an extension.
  - **Stripe Billing:** Excellent for subscription retries (Smart Retries using machine learning), but not designed for nuanced, human-to-human invoice chasing for custom services.
  - **The OHC Differentiator:** By unifying the inbox (Omnichannel Gateway) with the ledger, OHC's Finance Assistant ("The Accountant") acts autonomously but empathetically. It cross-references outstanding invoices with the unified customer communication graph. If it detects a promise to pay, it adjusts cash flow predictions and suppresses automated nag emails. When an intervention is actually needed, it drafts a highly contextual, personalized reminder and presents it in the owner's mobile feed for a 1-tap approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Customer Graph DB] --> B(Customer Identity & Context Engine)
      C[PostgreSQL Central Ledger] -->|Unpaid Invoices| D[Finance Agent: The Accountant]
      B --> D
      D -->|Analyzes recent communications| E{Contextual Check}
      E -->|Promised to pay Friday| F[Update Cash Flow Prediction & Pause Reminder]
      E -->|No recent contact| G[Draft Context-Aware Reminder]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J --> K[Email / WhatsApp / SMS]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top priority card shows "Action Required: Approve Invoice Reminder for Acme Corp."
  - **Interaction:** Tapping the card opens a detailed view.
    - **Top section (Context):** Shows the invoice amount ($2,500), days overdue (5 days), and a micro-summary of the last interaction ("Last contact 10 days ago via email").
    - **Middle section (The Draft):** Displays the AI-generated message. Example: "Hi Sarah, I hope the new branding assets are working out well for you! Just sending a gentle reminder regarding invoice #104. Let me know if you need another copy of the payment link."
  - **Action:** A prominent primary button "Approve & Send" and a secondary "Edit" button.
  - **Visual Design:** Adheres to OHC Premium Token library with Apple/Ubiquiti-style translucency, clean typography, and a 44x44px minimum touch target for the approval button.

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Runs on a daily CRON schedule to evaluate unpaid invoices. It queries the `Customer` and `OmniInboxMessages` tables.
  - **LLM Integration (Gemini Pro / GPT-4o):** Used for two specific tasks:
    1. **Intent & Entity Extraction:** Reading recent messages to determine if a payment extension was requested and agreed upon.
    2. **Drafting:** Generating a polite, contextual reminder message tailored to the channel (e.g., shorter for SMS/WhatsApp, slightly more formal for email) based on the tenant's brand voice.

  ### Key Design Decisions & Multi-Tenancy
  - **Strict Tenant Isolation:** The daily CRON job processes tenants sequentially or via a `SKIP LOCKED` job queue, ensuring the Finance Agent only retrieves context for the specific `tenant_id`.
  - **Owner-in-the-Loop:** For sensitive tasks like asking for money, OHC defaults to "Draft & Approve" rather than "Auto-Send" to build trust with the owner. Advanced users can toggle "Auto-Send for invoices < $500" in an advanced settings pane.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal (Nora), I wake up and check the OHC app on my iPhone. Instead of digging through a spreadsheet to find who owes me money, I see a card in my feed suggesting a polite, pre-written follow-up email to a client who is 3 days late on a $1,000 invoice. I tap "Approve" while drinking my coffee, and the payment link is sent.

  **CUJ & Acceptance Criteria:**
  1. A backend CRON job or job queue worker identifies an invoice that is past its due date in the central ledger.
  2. The system queries the unified communications graph and confirms no recent payment extensions were discussed.
  3. The Finance Agent drafts a contextual reminder message including the payment link.
  4. An "Action Required" item is populated in the user's mobile feed.
  5. The Playwright E2E test must log in as the owner, navigate the 375px mobile view, locate the reminder card, click "Approve & Send", and verify that the system dispatches the message and updates the task status.
  6. Ensure Zero-Trust multi-tenant isolation by verifying that the drafted message only contains data from the authorized `tenant_id`.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
