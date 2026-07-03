issue_title: "Implement Autonomous AI Estimate & Invoicing Agent"
issue_description: |
  ## 1. Problem Statement
  Service-based and project-based small business owners—like Carlos the Handyman and Nora the Agency Principal—spend a disproportionate amount of time gathering client requirements, manually drafting estimates or proposals, and chasing unpaid invoices. This disjointed workflow often spans multiple apps (e.g., messaging apps for intake, Word/Google Docs for proposals, and disjointed payment tools for invoicing), leading to delayed quotes, lost leads, and administrative fatigue. Traditional platforms like Shopify or Wix require expensive third-party plugins for quoting, which do not integrate natively with the unified inbox or customer context.

  ## 2. Research Report
  - **Competitor Gaps**:
    - *Shopify/Wix*: Primarily built for product checkout. Quoting requires clunky third-party apps that lack AI context awareness.
    - *FreshBooks/QuickBooks*: Strong on accounting but disconnected from the initial customer inquiry (DMs/forms). They require manual data entry.
    - *Square*: Offers basic estimates but lacks an AI agent to draft them based on conversational context.
  - **The OHC Opportunity**: Integrate an "Autonomous Estimate & Invoicing Agent." This agent sits at the intersection of the Unified Inbox (Customer Success) and the Finance Agent. When a customer messages a request (e.g., "I need 3 doors painted"), the agent cross-references the service catalog, drafts a detailed quote, and presents it to the owner for a 1-tap approval on their mobile device. Once the client approves the quote, it automatically converts into a scheduled task and an actionable invoice for deposit collection.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Unified Inbox / Form Submission] -->|New Inquiry Event| B[Work Triage Gateway]
      B --> C{Operations & Sales Agent}
      C -->|Query Service Catalog| D[(Service & Pricing DB)]
      C -->|Query Customer Context| E[(Customer Graph DB)]
      C -->|Draft Estimate| F[Estimate Draft Queue]
      F --> G[Mobile App Feed 375px]
      G -->|Owner 1-Tap Approve| H[Estimate Dispatcher]
      H -->|Sent to Client| I[Client Acceptance Portal]
      I -->|Client Approves| J[Finance & Operations Agent]
      J -->|Convert to Invoice| K[(Ledger / Invoice DB)]
      J -->|Schedule Task| L[(Calendar DB)]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Work Feed (Owner View):** A high-priority card appears: "Draft Quote Ready: Carlos, based on John's request for painting 3 doors."
  - **Interaction:** Tapping the card opens a full-screen, translucent glass-styled quote preview. It shows line items, calculated labor hours, materials, and total cost, automatically inferred by the AI.
  - **Action:** The owner can tap "Edit Line Items" or hit the primary prominent "Approve & Send Quote" button.
  - **Client View:** The client receives a clean, mobile-optimized link (OHC Premium Token styling) where they can review the quote and tap "Accept & Pay Deposit."

  ### AI Agent Integration Points
  - **Sales & Operations Agent:** Parses natural language requests ("paint 3 doors"), maps them to structured catalog items or custom line items, and generates the draft estimate.
  - **Finance Agent:** Tracks the quote status, automatically converts accepted quotes to invoices, processes deposit payments via Stripe integration, and schedules follow-up reminders.

  ### Key Design Decisions
  - **Zero Manual Entry Baseline:** The system must assume the AI drafts the first version of the quote. The owner's role is editorial, not generative.
  - **Strict Multi-Tenancy:** All estimates, line items, and invoices must enforce row-level security (`tenant_id`) and maintain strict isolation.
  - **Mobile-First Editing:** If the owner needs to edit a line item, the numeric keypad and inline editing must be flawless on a 375px viewport without horizontal scrolling.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous AI Estimate & Invoicing System
  **Target Persona**: Carlos the Handyman, Nora the Agency Principal

  **Outcome (CUJ)**:
  1. An inquiry arrives in the Unified Inbox.
  2. The system automatically drafts a Quote containing line items based on the inquiry.
  3. The owner reviews the Quote on their mobile device, makes adjustments if necessary, and approves it.
  4. The client accepts the Quote via a web link, triggering automatic invoice generation and deposit collection.

  **Acceptance Criteria**:
  - Implement PostgreSQL schemas for `Estimates`, `EstimateLineItems`, and `Invoices` with strict multi-tenant isolation.
  - Build the backend API to generate, retrieve, edit, and approve estimates.
  - Develop the mobile-first UI for the owner to review and approve the draft estimate (using OHC design tokens).
  - Develop the client-facing acceptance portal.
  - Ensure 100% unit test coverage and implement full E2E Playwright tests covering the quote-to-invoice journey.
  - No mock data in the UI; all data must flow through the real API and database.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
