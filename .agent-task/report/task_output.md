issue_title: "Build AI-Powered Mobile Quoting & Proposal Engine"
issue_description: |
  # Research Report: AI-Powered Mobile Quoting & Proposal Engine

  ## 1. Problem Statement
  Service-based owners and operators—such as Carlos (Field Service Owner) and Nora (Agency Principal)—lose valuable leads because drafting professional quotes and proposals from a mobile device is tedious and time-consuming. They often resort to informal text messages or disconnected third-party tools like Jobber, HoneyBook, or PandaDoc. This fragmentation leads to scattered customer records, untracked deposit payments, and missed follow-ups. OHC needs an integrated, agent-driven quoting engine that instantly turns customer intent into a structured, payable proposal directly from the owner's 375px mobile feed.

  ## 2. Research Report
  - **Market Context**: Platforms like Jobber and ServiceTitan are incredibly powerful but charge high monthly fees and require significant setup. Square Invoices and PayPal offer simple billing but lack the CRM context and conversational lead-in. HoneyBook caters to creatives but isn't optimized for quick field services or seamless multi-channel AI auto-drafting.
  - **The OHC Opportunity**: By deeply integrating quoting into the unified inbox, OHC can use the Sales & Revenue Assistant to proactively draft line-item quotes based on DM conversations. OHC's unique advantage is the "One-Tap Approve" flow, turning an Instagram DM like "How much to fix my sink?" into a professional, deposit-ready quote with zero manual typing from the owner.
  - **Competitor Gaps**:
    - *Jobber*: Excellent field service scheduling but lacks proactive AI drafting from multi-channel inboxes.
    - *Square Invoices*: Purely financial; detached from the initial customer conversation.
    - *HoneyBook*: Complex pipeline management; not built for rapid, mobile-first 30-second actions.

  ## 3. Design Doc
  ### Data Model & System Architecture
  - **Data Entities**:
    - `Quote` / `Proposal`: Linked to `tenant_id`, `customer_id`, with states (`draft`, `pending_approval`, `sent`, `accepted`, `declined`, `converted_to_invoice`).
    - `LineItem`: Services or products attached to the quote.
    - `PaymentTerms`: Deposit requirements (e.g., 50% upfront) and due dates.
  - **Multi-Tenant Isolation**: Row-level security strictly enforced via `tenant_id` on all tables.
  - **Architecture Diagram**:
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WorkTriage (Inbox)
      participant SalesAgent (AI)
      participant OwnerFeed (Mobile UI)
      participant Ledger (DB)

      Customer->>WorkTriage: "Can I get an estimate for a custom 3-tier cake?"
      WorkTriage->>SalesAgent: Context: Customer wants cake.
      SalesAgent->>Ledger: Retrieve pricing rules & availability.
      SalesAgent-->>OwnerFeed: Draft Quote (3 tiers, $150, 50% deposit)
      OwnerFeed->>OwnerFeed: Owner reviews on 375px screen
      OwnerFeed->>Customer: Approve & Send Secure Link
      Customer->>Ledger: Accept & Pay Deposit via Stripe
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification & Inbox View**: Owner receives a push notification. Opening the app shows the unified feed with a card: "Maya, I drafted a quote for Sarah's custom cake."
  2. **Approval Card (Translucent Glass UI)**: A clear, touch-friendly 375px card displaying the drafted line items, total price, and deposit amount.
  3. **Edit Mode**: If the owner needs to tweak it, tapping a line item opens a native mobile bottom sheet with large, 44x44px touch targets to adjust quantities or prices.
  4. **Customer Facing**: The customer receives a responsive web link (PWA) to view the proposal, digitally accept it, and pay the deposit via Stripe Checkout.

  ### AI Agent Integration
  - **Work Triage Agent**: Identifies intent (Quoting/Estimating) from incoming messages (Instagram, WhatsApp, Email).
  - **Sales & Revenue Agent**: Given the context, it formulates the line items based on past similar jobs or the tenant's predefined service catalog.
  - **Finance & Decision Agent**: Tracks the acceptance rate and deposit collection, notifying the owner if follow-up is needed.

  ## 4. Implementation Prompt
  **Target Persona**: Carlos (Handyman) & Nora (Agency Principal)
  **Feature Name**: Agentic Quoting & Proposal Engine

  **Implementation Steps**:
  1. Define the PostgreSQL schemas for `quotes`, `quote_line_items`, and `quote_payment_terms`, ensuring strict `tenant_id` isolation and Bazel build compatibility.
  2. Implement the gRPC and REST endpoints to Create, Update, Send, and Accept quotes.
  3. Integrate the AI Sales Agent prompt: Build the LangGraph/AI worker node that listens for "estimate_requested" events and drafts the quote payload.
  4. Develop the Flutter mobile-first (375px) UI components: the "Quote Draft Review" card in the owner feed, and the detailed "Edit Quote" bottom sheet using the OHC premium design tokens (Apple/Ubiquiti translucent glass style).
  5. **Automated Verification**: Implement comprehensive Playwright E2E tests validating the complete journey—from the AI generating the draft, to the owner editing and approving it, to the customer accepting and paying the deposit. Ensure zero mocked API calls in E2E tests; all data must flow through the real backend.

  ## 5. Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
