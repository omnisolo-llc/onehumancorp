issue_title: "AI-Driven Dynamic Quotation & Proposal Engine"
issue_description: |
  # Research Report: AI-Driven Dynamic Quotation & Proposal Engine

  ## 1. Problem Statement
  Service-based and project-based small business owners (e.g., Carlos the Handyman, Nora the Agency Principal) spend hours manually drafting quotes, proposals, and estimates for clients. They often rely on fragmented tools (Word, Excel, basic CRM templates) that are disconnected from their core operations, pricing catalogs, and invoicing. This manual process is slow, prone to errors, and delays revenue capture.

  ## 2. Research Report
  - **Market Context**: Platforms like Quickbooks or Freshbooks offer basic estimate generation, but they are static forms requiring manual data entry. Specialized tools like Proposify are powerful but too complex and expensive for a single operator or micro-agency.
  - **The OHC Opportunity**: By integrating quotation generation directly into the OHC event mesh and powering it with the Sales & Revenue Assistant (The Salesperson) and Operations Assistant (The Manager), we can turn a raw customer inquiry into a fully fleshed-out, interactive quote in seconds.
  - **Competitor Gaps**:
    - *Freshbooks/Quickbooks*: Static templates, no AI context synthesis from previous client chats, completely detached from project management.
    - *HoneyBook*: Good for creatives, but setup is heavy. Does not autonomously draft proposals based on an omnichannel inbox feed.
    - *Shopify/Wix*: Inherently product-focused; service quoting is unnatural and requires heavy app workarounds.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via Inbox] -->|Webhook/Event| B(Omnichannel Gateway)
      B --> C{The Salesperson Agent}
      C -->|Query Catalog & Availability| D[Operations Agent / The Manager]
      D -->|Return Pricing & Calendar| C
      C -->|Draft Quote JSON| E[Quote Engine]
      E -->|Render Glassmorphism UI| F[Owner Dashboard Feed 375px]
      F -->|Owner 1-Tap Approve| G[Quote Delivery Service]
      G -->|Email/SMS Link| H[Customer Interactive Web View]
      H -->|Accept & Pay Deposit| I[Stripe Checkout]
      I -->|Webhook| J[Invoice & Project Auto-Creation]
  ```

  ### Data Model (PostgreSQL)
  - `Quote`: The overarching entity containing `tenant_id`, `customer_id`, `status` (draft, sent, accepted, rejected, expired), and `valid_until`.
  - `QuoteLineItem`: Granular services/products included, with `unit_price`, `quantity`, and `description`.
  - `QuoteTerm`: Specific contractual or operational terms appended by the AI based on the service type (e.g., "50% deposit required").

  ### Mobile UX Flow (375px First)
  1. **Owner View (Triage Feed)**: A card appears: "Carlos, a new inquiry from John for 'Deck Repair'. I have drafted a $1,200 estimate based on your standard rate."
  2. **Draft Review**: Tapping the card opens a clean, translucent glass UI showing the line items. The owner can tap to edit amounts or add a line item via native keyboard.
  3. **One-Tap Send**: The owner taps "Approve & Send".
  4. **Customer View**: The customer receives an SMS link to a mobile-optimized web page showing the quote. A prominent "Accept & Pay Deposit" button initiates a Stripe checkout session.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Listens to the unified inbox. When an inquiry implies a request for pricing, it extracts the scope, queries the `Service` catalog, and drafts the `Quote` record.
  - **Operations Assistant**: Verifies if the requested timeline is feasible on the calendar before the quote is drafted, adding a note if scheduling is tight.

  ### Key Design Decisions
  - **Approval-Gate**: The AI *never* sends a binding quote autonomously. It drafts it and places it in the owner's feed for one-tap approval.
  - **Interactive Customer View**: Quotes are not static PDFs. They are dynamic web views that seamlessly transition into payment and booking.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: As Carlos, when a customer texts me asking "How much to fix my fence?", I open OHC to find a drafted quote based on my standard hourly rate and material markup. I tap "Approve," and the customer gets a link to pay the deposit instantly.
  **CUJ & Acceptance Criteria**:
  1. Create the `Quote` and `QuoteLineItem` schemas with RLS for multi-tenant isolation.
  2. Implement an API endpoint that the Sales Agent can call to generate a draft quote from JSON output.
  3. Build the 375px mobile UI card for the owner feed to review and edit the drafted quote.
  4. Build the customer-facing public quote view that integrates with a Stripe Payment Link for deposit collection.
  5. Provide Playwright E2E tests: A test script mimics an agent creating a draft quote, the owner approving it via the UI, and the customer navigating to the public link to accept it.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
