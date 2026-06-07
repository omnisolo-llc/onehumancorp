issue_title: "AI-Automated Quote-to-Cash (Q2C) & Deposit Workflow Architecture"
issue_description: |
  ## Problem Statement
  Small business owners like Carlos (Handyman), Maya (Baker), and Nora (Agency Principal) often struggle with "Payment Gateway Confusion" and manual invoice tracking. A critical pain point is the friction between capturing a lead's interest and securing the revenue (the Quote-to-Cash process). For custom orders or services, owners manually bounce between Instagram DMs/emails, spreadsheet quotes, and disjointed Stripe/PayPal payment links to collect deposits. This multi-tool jumping causes dropped leads, delayed payments, and unorganized financial records. We need an integrated, AI-assisted Quote-to-Cash flow natively built for mobile viewports (375px) that handles proposal generation, deposit requests, and payment collection seamlessly.

  ## Research Report
  - **Market Context**: In the global SMB market, over 18% of owners suffer from Payment Gateway Confusion. Custom service providers lose up to 25% of potential bookings due to delayed quotes and cumbersome payment collection processes.
  - **Competitor Analysis**:
    - **Shopify**: Excellent for standard inventory, but requires cumbersome third-party apps for custom quotes and phased deposits.
    - **Wix/Squarespace**: Offer basic invoicing, but the mobile UX is clunky and heavily relies on manual owner input rather than AI generation.
    - **Square/GoDaddy**: Strong in-person POS but disjointed omnichannel messaging and quoting.
  - **OHC Opportunity**: OHC can differentiate by leveraging the "Sales & Revenue Assistant" agent to listen to the "Work Triage" feed. When a custom inquiry arrives (e.g., "Do you do vegan cakes?"), the AI drafts a response AND automatically generates a pending quote with a one-tap deposit link.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via DM/Web] -->|Work Intake| B(Work Triage Feed)
      B --> C{AI Customer & Sales Assistant}
      C -->|Drafts Quote/Proposal| D[Proposal Engine]
      D -->|Owner Approval 1-Tap| E[Payment Gateway / Stripe API]
      E -->|Generates Deposit Link| F(Send to Customer)
      F -->|Customer Pays| G[Stripe Webhook]
      G -->|Idempotent Update| H[(PostgreSQL Database)]
      H -->|Trigger| I(Finance Assistant)
      I -->|Dashboard Notification| J[Owner Mobile UI 375px]
  ```

  ### Mobile UX Flow (375px)
  1. **Notification Card**: Owner receives a "New Custom Request" card in the OHC Work Feed.
  2. **Quote Review Screen**: Tapping the card opens an AI-generated quote draft (price, scope, required deposit). The owner can adjust sliders or text natively.
  3. **Approve & Send**: A floating, translucent glass "Send Quote & Payment Link" button (minimum 44x44px touch target) dispatches the proposal.
  4. **Pending Payment State**: The card in the feed transitions to a "Waiting for Deposit" status token.
  5. **Success Confirmation**: Upon webhook receipt, a celebration animation plays, and the task automatically moves to the "Operations/Fulfillment" queue.

  ### AI Agent Integration Points
  - **Trigger**: New message with intent to buy/book.
  - **Action**: The *Sales & Revenue Assistant* extracts parameters (items, dates, constraints) and calls a structured internal tool `draft_quote_and_deposit`.
  - **Memory**: The agent stores the customer's budget and preferences in the tenant-scoped memory for future upselling.

  ### Key Design Decisions
  - **Zero-Setup Gateway Abstraction**: We will hide Stripe/Payment complexities behind an "OHC Payments" UI abstraction, allowing the owner to start sending quotes before fully passing KYC, capturing the intent first.
  - **Stateful Quote Cards**: Instead of separate tabs for "Messages" and "Invoices," quotes are embedded directly into the conversational timeline or feed.
  - **Optimistic UI with Background Sync**: Creating a quote must feel instantaneous on mobile, utilizing background job queues (PostgreSQL `SKIP LOCKED`) to eventually sync with external payment providers.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to build the unified Quote-to-Cash (Q2C) feature within the OHC platform.
  - **Objective**: Allow the owner to review an AI-drafted quote, edit it on a 375px mobile screen, and send a deposit request link to the customer with one tap.
  - **CUJ**:
    1. Owner opens the app and sees a pending quote drafted by the AI from a recent customer inquiry.
    2. Owner modifies the deposit amount and taps "Send to Customer".
    3. The system generates a payment link and updates the work item state to "Awaiting Deposit".
  - **Acceptance Criteria**:
    - Build the mobile-first UI components using OHC Premium Tokens (macOS Translucent Glass style).
    - Ensure 100% usability on a 375px viewport with no horizontal scrolling.
    - Integrate the UI with the existing AI job queue and backend API for quote state management.
    - All interactive elements must pass the "grandmother test" and have >44px touch targets.
    - Must include full Playwright E2E test coverage verifying the quote creation and state transition.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
