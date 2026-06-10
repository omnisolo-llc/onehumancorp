issue_title: "OHC Instant Localized Invoicing & Smart Expense Ledger"
issue_description: |
  ## 1. Problem Statement
  Service-based SMBs, agencies, and independent professionals (e.g., Nora the Agency Principal, Carlos the Handyman) currently manage invoicing and expense tracking outside of their primary operational tools. They often piece together disconnected platforms like QuickBooks, Stripe Invoicing, or manual Excel sheets. This disjointed approach leads to delayed payments, missed expense logging, and a lack of real-time financial clarity.

  Crucially, current legacy systems lack "Invisible AI Automation." They act as dumb ledgers that wait for the user to manually input data, categorize expenses, and chase overdue invoices. They also frequently fail to cater to localized multi-currency needs dynamically, imposing heavy setup friction for cross-border or regional independent professionals.

  ## 2. Research Report
  - **Market Context**: Platforms like Xero and QuickBooks dominate the pure accounting space but are overwhelmingly complex for a solopreneur who just needs to send a quote, convert it to an invoice, get paid, and snap a picture of a receipt. Shopify is built for product e-commerce and its invoicing is an afterthought.
  - **The OHC Opportunity**: By natively integrating an intelligent "Finance & Decision Assistant" linked to a universal ledger, OHC can collapse quoting, invoicing, payment collection, and expense tracking into a single, mobile-first flow.
  - **Competitor Gaps**:
    - *QuickBooks/Xero*: High complexity, requires accounting knowledge, desktop-first UI.
    - *Stripe Invoicing*: Powerful backend but lacks integrated project management, native expense tracking, and AI-driven automated follow-ups in a unified SMB-friendly app.
    - *Square*: Good for POS, but weaker on B2B agency-style invoicing and AI-driven predictive cash flow.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App / Web UI] -->|Create Invoice / Upload Receipt| B(OHC API Gateway)
      B --> C[Finance & Decision Agent]
      C -->|Drafts Invoice & Matches Expenses| D{Smart Engine}
      D --> E[Invoicing Service]
      D --> F[Expense Service]
      E --> G[Universal Ledger Postgres]
      F --> G
      G -->|Syncs state| H[Distributed Cache Redis]
      E -->|Triggers payment link| I[Stripe Integration]
      C -->|Auto-reminders| J[Notification Service]
      J --> A
  ```

  ### Mobile UX Flow (375px First)
  1. **Invoice Creation**: Nora taps "+" -> "New Invoice". The UI is a simple, translucent glassmorphism card. The Finance Agent auto-suggests line items based on recently completed project tasks. She reviews, taps "Approve", and the invoice is sent with a Stripe payment link.
  2. **Expense Logging**: Carlos finishes a job, buys supplies, opens the OHC app, and snaps a photo of the receipt. The Vision LLM extracts the merchant, amount, date, and categorizes it. The Finance Agent asks: "Assign this $45 Home Depot expense to the Smith kitchen job?" Carlos taps "Yes".
  3. **Financial Feed**: The home feed pushes actionable cards: "Invoice #102 is 3 days overdue. Tap to send a polite automated reminder."

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant)**:
    - Actively monitors the universal ledger.
    - Drafts invoices from completed tasks or accepted quotes.
    - Uses OCR/Vision to parse receipts and auto-categorize expenses.
    - Manages multi-currency conversions dynamically based on client location.
    - Generates plain-language daily/weekly financial summaries ("Cash flow looks tight next week; consider following up on 3 pending invoices.").

  ### Key Design Decisions
  - **Universal Ledger Integration**: Build on top of the existing `ohc_universal_ledger` to maintain an immutable, append-only financial history with strict row-level security per tenant.
  - **Zero Setup Localization**: The system should automatically handle localized formatting (currency symbols, date formats, tax regimes) without requiring the user to configure complex regional settings.
  - **Proactive Not Reactive**: The UX is feed-driven. Instead of digging into a "Reports" tab, the owner is presented with actionable insights and pre-drafted reminders.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Instant Localized Invoicing & Smart Expense Ledger
  **Target Persona**: Nora the Agency Principal and Carlos the Handyman
  **Outcome**: Nora can turn a completed project phase into a paid invoice with one tap, and Carlos can log job expenses instantly via photo, with the Finance Agent handling categorization, ledger entry, and payment reminders autonomously.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Define the PostgreSQL data models for `Invoices`, `InvoiceLineItems`, and `Expenses`, ensuring they link securely to the `ohc_universal_ledger` and respect tenant isolation.
  2. Implement the API endpoints for the mobile-first UX to support invoice creation and receipt image upload.
  3. Integrate the Finance Agent (via the LLM provider) to parse uploaded receipt images (extracting amount, merchant, date) and to auto-draft overdue payment reminder notifications.
  4. Build automated Playwright E2E tests validating that an owner can create an invoice, record a payment, and see the updated state accurately reflected without horizontal scrolling on a 375px viewport.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []