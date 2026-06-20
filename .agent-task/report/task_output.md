issue_title: "Design: The Estimator Agent - Automated Interactive Proposals & Multi-Stage Deposits"
issue_description: |
  # Research Report: The Estimator Agent (Automated Interactive Proposals & Multi-Stage Deposits)

  ## 1. Problem Statement
  For service-based and custom-order small businesses (e.g., Carlos the handyman, Maya the custom baker, Nora the agency principal), turning an inquiry into a paid job is highly manual. They receive DMs or forms, manually calculate costs, draft a PDF or text message quote, and manually request a deposit via a generic payment link. This fragmented process leads to high drop-off rates, double-entry errors, and significant time wasted on leads that don't convert. Existing platforms like Shopify are built for fixed-price carts, not custom quotes. Tools like HoneyBook or QuickBooks are too complex for a mobile-first user.

  ## 2. Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Primarily built for static SKUs. Custom orders require complex third-party apps or draft orders which are clunky on mobile.
  - **Wix/Squarespace:** Offer basic form builders, but turning a form submission into an interactive, payable quote requires manual intervention or expensive plugins.
  - **HoneyBook/Dubsado:** Excellent for proposals and contracts, but overly complex. They feel like enterprise software, requiring desktop setup and significant onboarding.
  - **OHC Opportunity:** Leverage our AI framework to build "The Estimator Agent". When an inquiry comes in via Work Triage, the Estimator Agent drafts an interactive proposal (Quote + Contract + Deposit Link) directly in the mobile feed. The owner just reviews and taps "Approve & Send".

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry/Form] -->|Event| B(Work Triage / Inbox)
      B --> C{The Estimator Agent}
      C -->|Query| D[Tenant Service/Product Catalog & Pricing Rules]
      C -->|Query| E[Operations Agent - Calendar Availability]
      C -->|Draft| F[Interactive Proposal Object]
      F --> G[Owner Mobile Feed - 375px]
      G -->|Owner Approves| H[Dispatch via SMS/Email/DM]
      H --> I[Customer Opens WebPWA Quote]
      I -->|Accepts & Pays Deposit| J(Stripe Checkout/Payment Intent)
      J --> K[Operations Agent - Confirms Booking & Updates Ledger]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Owner Feed Card:** "Carlos, new repair request from John. I drafted a quote for $150 with a $50 deposit based on your standard rates."
  - **Interaction:** Owner taps the card. A clean, Apple-style translucent modal opens showing the line items, the required deposit amount, and the generated message.
  - **Actions:** "Approve & Send" (primary button), "Edit Items" (secondary).
  - **Customer View:** The customer receives a secure, edge-cached dynamic link. Clicking it opens a beautiful, mobile-optimized (375px) proposal with an integrated Stripe Payment Link to pay the deposit.

  ### AI Agent Integration Points
  - **The Estimator Agent:** Uses RAG against the owner's past quotes, pricing sheets, and catalog. Automatically parses customer intent (e.g., "I need a 2-tier vegan wedding cake") into structured line items and required deposit rules.
  - **Operations Agent:** Ensures the requested date/time in the quote is held tentatively or blocked upon deposit payment.

  ### Key Design Decisions
  - **Interactive Proposal Object:** A new data entity in PostgreSQL that represents a multi-state document (Draft -> Sent -> Viewed -> Accepted -> Paid).
  - **Mobile-First Quoting:** Complex CPQ (Configure, Price, Quote) reduced to an AI-drafted card that requires only one tap to approve.
  - **Deposit-First Logic:** Deeply integrated multi-stage payments (Deposit + Final Invoice) using Stripe Payment Intents, automatically tracked in the central ledger.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** As an owner (like Carlos or Maya), when a customer requests a custom service, my OHC app automatically presents a drafted quote with the correct price and deposit amount. I tap "Approve", the customer gets a link, pays the deposit, and the job is booked—all from my phone in under 10 seconds.

  **Acceptance Criteria:**
  - Create the PostgreSQL schema for the `interactive_proposals` and `proposal_line_items` tables with RLS and tenant isolation.
  - Implement The Estimator Agent logic to parse raw text inquiries into structured proposals.
  - Build the 375px mobile feed card for owner approval.
  - Generate a secure, web-facing customer proposal view with integrated Stripe deposit checkout.
  - Write Playwright E2E tests covering the journey from owner approval to customer deposit payment.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
