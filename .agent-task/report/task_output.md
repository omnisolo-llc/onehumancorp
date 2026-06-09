issue_title: "Implement the AI-Native Quoting & Proposal Generator (The Sales Assistant)"
issue_description: |
  # Research Report: AI-Native Quoting & Proposal Generator (The Sales Assistant)

  ## Problem Statement
  Service-based and custom-order SMBs (like Nora the Agency Principal or Carlos the Field Service Owner) spend an inordinate amount of time drafting quotes, estimating costs, and chasing approvals. They are often out in the field or dealing with other tasks, making it difficult to respond to inquiries promptly with professional, accurate estimates. Traditional tools require them to manually type line items, calculate totals, and attach separate PDFs. They need an AI assistant that can take a brief description of the job (or a forward from a customer message), draft a complete, professional quote, and present it for a 1-tap mobile approval.

  ## Research Report
  - **Market Context**: Platforms like Shopify do not handle custom service quoting well; they rely on clunky draft orders or expensive B2B apps. CRM tools like HubSpot or specialized tools like Jobber offer quoting, but they are manual forms. There is no major SMB platform that offers *agentic* quoting where the AI builds the quote from a conversational prompt or customer inquiry.
  - **The OHC Opportunity**: By integrating quoting natively and powering it with a Sales AI Agent, OHC can reduce the time-to-quote from hours/days to minutes. This dramatically increases the win rate for SMBs.
  - **Competitor Gaps**:
    - *Jobber*: Manual entry, requires navigating complex forms.
    - *Shopify*: Focuses on pre-priced products, not custom quotes.
    - *Stripe Invoicing*: Great for billing, but lacks the "proposal/estimate" AI generation step based on service context.

  ## Design Doc
  ### Data Model (PostgreSQL)
  The core `quotes` and `quote_line_items` schema exists but needs to be fully integrated with the AI layer and the front-end.

  ```mermaid
  erDiagram
      Tenant ||--o{ Quote : has
      Customer ||--o{ Quote : requests
      Quote ||--|{ QuoteLineItem : contains

      Quote {
          ID id PK
          ID tenant_id FK
          ID customer_id FK
          String status
          DateTime valid_until
          Amount total_amount
          Text notes
      }

      QuoteLineItem {
          ID id PK
          ID quote_id FK
          String description
          Amount unit_price
          Integer quantity
          Boolean is_optional
      }
  ```

  ### AI Integration (Sales Assistant)
  - **The Sales Assistant (Agent)**: Given a customer request (e.g., "Customer needs 5 custom vegan cakes for next Saturday"), the agent will query the product catalog/service list for pricing constraints, estimate labor, and draft a JSON payload representing the `Quote` and `QuoteLineItem`s.
  - **Handoff**: The drafted quote is pushed to the owner's Agent Feed for review.

  ```mermaid
  sequenceDiagram
      actor Customer
      actor Owner
      participant AgentFeed as Agent Feed (Mobile)
      participant SalesAgent as Sales Assistant (AI)
      participant Catalog as Product/Service Catalog
      participant QuotingService as Quoting API

      Customer->>AgentFeed: Sends inquiry via DM/SMS
      Owner->>AgentFeed: Taps "Draft Quote"
      AgentFeed->>SalesAgent: Natural language request context
      SalesAgent->>Catalog: Fetch pricing/constraints
      Catalog-->>SalesAgent: Returns context
      SalesAgent->>SalesAgent: Drafts Quote & Line Items JSON
      SalesAgent->>QuotingService: Request to create Quote
      QuotingService-->>SalesAgent: Returns drafted Quote ID
      SalesAgent-->>AgentFeed: Pushes "Review Drafted Quote" Action Card
      Owner->>AgentFeed: Reviews & Edits Quote Card
      Owner->>QuotingService: Taps "Approve & Send"
      QuotingService-->>Customer: Sends SMS/Email link to accept
  ```

  ### Mobile UX Flow (375px)
  1. **Triage/Intake**: Owner sees a new inquiry in the Agent Feed.
  2. **Agent Drafts**: Owner taps "Draft Quote". The Sales Assistant generates the quote.
  3. **Review & Approve**: Owner sees a mobile-optimized card summarizing the quote. They can tap to edit line items (using native mobile keyboards/steppers for quantities) or tap "Approve & Send".
  4. **Customer View**: Customer receives an SMS/Email link to a beautiful, translucent glass styled OHC-hosted page to view and "Accept" the quote (optionally transitioning to a deposit payment flow).

  ## Implementation Prompt
  **Target Persona**: Carlos (Field Service Owner)

  **Outcome**: Carlos receives a text about a repair job. He opens OHC, uses the Sales Assistant to instantly draft a professional quote based on standard labor rates and parts, reviews it on his Android phone, and sends it to the customer with one tap.

  **Next Actions for Engineering**:
  1. **Backend Integration**: Connect the existing `quoting` service to the LLM layer. Create an agent capability that takes natural language input and outputs a structured quote request.
  2. **Agent Feed UX**: Implement the "Draft Quote" action card in the mobile-first UI. Ensure it uses the premium translucent glass design system.
  3. **Customer Proposal Page**: Build the public-facing route where customers can view the quote and tap "Accept".
  4. **E2E Testing**: Write a Playwright E2E test verifying that a user can generate a quote via the agent, approve it, and the customer can accept it.

  **Acceptance Criteria**:
  - The quote generation must happen automatically from a prompt.
  - The review screen must be perfectly usable on a 375px screen (no horizontal scrolling for line items).
  - All new code must have 100% unit test coverage.
  - Playwright tests must run against the real backend without mocked data.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
