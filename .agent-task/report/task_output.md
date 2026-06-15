issue_title: "Implement AI-Powered Dynamic Service Quoting & Invoicing Agent"
issue_description: |
  # Research Report: AI-Powered Dynamic Service Quoting & Invoicing

  ## Problem Statement
  Service-based small business owners (e.g., Carlos the Field Service Owner, Nora the Agency Principal) spend hours manually drafting quotes, estimating costs, and chasing invoices. Existing platforms like Shopify are built for physical goods, not custom service estimating. Specialized tools (like Jobber or QuickBooks) are too complex, require manual data entry, and are detached from the primary customer inbox. Owners need a way to instantly convert a customer inquiry (e.g., "How much to fix a leaky pipe?") into a professional, actionable quote and invoice without leaving their phone.

  ## Research Report & Competitive Analysis
  - **Current Solutions (Jobber, Housecall Pro):** Excellent for field services, but they are purely functional tools. They do not proactively draft estimates based on conversational context.
  - **Invoicing Tools (QuickBooks, FreshBooks):** Require formal itemization and are highly rigid. They intimidate micro-SMBs and non-technical operators.
  - **The OHC Opportunity:** Leverage the Sales & Finance AI Agents to parse customer DMs/emails, extract scope details, reference the owner's historical pricing (from the unified Ledger), and instantly draft an interactive Quote/Invoice card. The owner simply taps "Approve" on their 375px mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer DM/Email] --> B[Work Triage Agent]
      B --> C{Sales & Revenue Assistant}
      C -->|Query Past Pricing| D[Ledger & Billing DB]
      C -->|Draft Quote| E[Quote Generation Engine]
      E --> F[Owner Mobile Feed - 375px]
      F -->|1-Tap Approve| G[Payment Link Generator Stripe]
      G --> H[Dispatch Quote to Customer]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Carlos receives a push notification: "New Estimate Request from John (Leaky Pipe)".
  2. **Review Feed Card:** Carlos opens the OHC app. The feed displays a translucent glass card summarizing the request.
  3. **AI Drafted Quote:** Below the summary, the AI proposes: "Standard Plumbing Visit: $150 + Parts Deposit: $50". It bases this on his previous similar invoices.
  4. **Action:** Carlos can tap "Edit Items", "Send as Quote", or "Send as Invoice (Payment Required)". The entire flow takes < 15 seconds.

  ### AI Agent Integration Points
  - **Work Triage:** Parses incoming text/audio to identify it as a request for quote.
  - **Finance/Sales Agent:** Queries the PostgreSQL `ohc.ledger` and `ohc.billing` records to find average historical prices for similar work to generate the line items.
  - **Knowledge Assistant:** Appends the standard terms of service or cancellation policy to the quote automatically.

  ## Implementation Prompt
  - Build the backend service that listens for `QuoteRequested` events from the unified inbox.
  - Implement the generative AI pipeline to extract line items and prices from conversational text, referencing historical billing data.
  - Create the data models for `Quote` and `QuoteLineItem` with strict multi-tenant isolation.
  - Develop the 375px mobile-first UX feed card that allows the owner to review, edit, and approve the AI-drafted quote with a single tap.
  - Integrate with the existing Stripe payment system to generate a Payment Link or Checkout Session upon quote approval.

  ## Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
