issue_title: "Implement AI-Assisted Smart Quote & Proposal Generation Architecture"
issue_description: |
  # Research Report: AI-Assisted Smart Quote & Proposal Generation Architecture

  ## 1. Problem Statement
  Service-based and project-based small business owners—such as Carlos (Handyman) and Nora (Agency Principal)—lose critical time drafting quotes, estimating service costs, and formatting proposals. Existing general-purpose tools like Word/Google Docs are disconnected from inventory, services, and CRM. Even specialized estimating tools often require too much manual input, technical jargon, and don't proactively leverage AI to respond instantly to inquiries (e.g., via SMS or Instagram DMs). A non-technical owner needs the assistant to instantly analyze a request, consult their pricing/services, and draft a professional, ready-to-send proposal that accepts digital deposits.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify lack robust native quoting for services. Square Invoices and Wix Bookings provide manual quote generation, but rely on the owner manually typing line items. Specialized tools (Jobber, HoneyBook) are powerful but expensive and operate as disconnected silos.
  - **The OHC Opportunity**: By deeply integrating the Sales and Knowledge AI Agents with the core Catalog/Services database, OHC can instantly convert an inbound lead (e.g., "How much to fix a leaky pipe?" or "Need a website redesign") into a drafted, accurate, and professional quote.
  - **Competitor Gaps**:
    - *Shopify*: Extremely rigid towards physical products; weak native service quoting.
    - *Wix/Squarespace*: Manual input required; no AI-driven automated drafting based on historical data or standard service blocks.
    - *HoneyBook/Jobber*: Too complex for very small operators (e.g., Carlos on an Android phone); not unified with a single "Assistant" feed.

  ## 3. Design Doc
  ### Architecture & Data Model (PostgreSQL)
  - `QuoteProposal`: Core entity linking a Lead/Customer to proposed line items. Fields include `status` (draft, sent, accepted, rejected), `total_amount`, `deposit_required`, `expiration_date`.
  - `QuoteLineItem`: Individual services, products, or custom tasks.
  - `QuoteTemplate`: Reusable, parameterized blocks for standard jobs (e.g., "Basic Plumbing Call", "Standard Brand Package").
  - **Multi-Tenant Isolation**: Row-Level Security (RLS) on all tables using `tenant_id`.

  ### AI Integration Points
  - **Sales/Customer Success Agent**: Intercepts inbound inquiries, parses the request, queries the `QuoteTemplate` and `Service` catalog, and drafts the `QuoteProposal`.
  - **Knowledge Agent**: Uses RAG against past accepted proposals and the owner's documented pricing guidelines to ensure accuracy.
  - **Finance Agent**: Automatically schedules follow-ups for unaccepted quotes and converts accepted quotes into invoices/deposit requests (Stripe integration).

  ### Mobile UX Flow (375px)
  1. **Notification/Feed**: Nora receives a push notification and sees an Action Card in her Agent Feed: "New Lead: Website Redesign. Proposal drafted."
  2. **Review Screen**: Tapping the card opens a clean, translucent glass-styled mobile view. The proposal is laid out clearly without horizontal scrolling.
  3. **Edit/Approve**: Simple UI to tweak quantities or prices using native numeric keyboards. A prominent "Approve & Send" button at the bottom.
  4. **Customer View**: The customer receives a secure, mobile-responsive web link where they can view the proposal, e-sign, and pay the deposit via Stripe Checkout.

  ## 4. Implementation Prompt
  **Feature Name**: AI-Assisted Smart Quote & Proposal Generation
  **Target Personas**: Carlos (Field Service), Nora (Agency)
  **Outcome**: OHC listens to inbound requests, drafts accurate quotes based on the owner's services/inventory, and presents a ready-to-send proposal card in the Agent Feed.

  **Next Actions**:
  1. Implement the core Data Models (`QuoteProposal`, `QuoteLineItem`, `QuoteTemplate`) with strict PostgreSQL RLS.
  2. Develop the AI Sales Agent capability to parse natural language service requests and map them to catalog items to generate a draft.
  3. Build the Mobile-First Owner UX: An actionable Agent Feed card for reviewing, editing (with native mobile inputs), and approving the quote.
  4. Build the Customer-Facing View: A clean, public-facing React/Flutter web page for the customer to review the quote, accept it, and trigger a Stripe deposit flow.
  5. Ensure 100% unit test coverage and E2E Playwright tests simulating Carlos receiving a request and approving the drafted quote.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []