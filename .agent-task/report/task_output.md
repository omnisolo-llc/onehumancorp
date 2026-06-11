issue_title: "Agentic Autonomous Quoting & Proposal Generation Engine"
issue_description: |
  # Research Report: Agentic Autonomous Quoting & Proposal Generation Engine

  ## 1. Problem Statement
  Service-based and project-based small business owners—such as Nora (Agency Principal) and Carlos (Handyman)—lose substantial time and potential revenue due to the friction of manual quoting. Currently, turning an inbound customer request into a professional, legally sound proposal with integrated payment or deposit links requires juggling multiple disconnected tools (email, Word/Docs, specialized CRM like HubSpot or Jobber, and payment processors like Stripe). This friction delays response times, drops conversion rates, and increases the administrative burden on the owner.

  ## 2. Research Report
  - **Market Context**: Traditional CRMs like HubSpot or specialized field service software like Jobber offer quoting tools, but they require significant manual data entry and configuration. Platforms like Wix and Shopify are product-first; their service quoting tools feel like bolted-on forms rather than core workflows.
  - **The OHC Opportunity**: By natively integrating an AI-driven Quoting & Proposal Engine into the OHC unified inbox, OHC can eliminate the "proposal bottleneck." The AI can instantly draft quotes based on historical pricing, service catalog, and conversational context, allowing the owner to simply review and send.
  - **Competitor Gaps**:
    - *Jobber / ServiceTitan*: Excellent for field services but highly specialized, complex, and lack deep AI agentic workflows that draft proposals from casual social media DMs or text messages.
    - *Shopify / Wix*: E-commerce focused; quoting for custom services is an afterthought requiring third-party plugins.
    - *HubSpot*: Powerful but targets enterprise/mid-market, creating overwhelming complexity for micro-SMEs.

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ QuoteTemplate : has
      Tenant ||--o{ Quote : issues
      Tenant ||--o{ Customer : manages
      Customer ||--o{ Quote : receives
      Quote ||--o{ QuoteLineItem : contains
      Quote ||--|| DepositRequirement : requires

      Quote {
          uuid id
          uuid tenant_id
          uuid customer_id
          string status
          decimal total_amount
          datetime validity_period
      }

      QuoteLineItem {
          uuid id
          uuid quote_id
          string service_name
          decimal amount
          int quantity
      }
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      actor Owner
      participant Webhook as Social/Email Webhook
      participant SalesAgent as Sales Agent (LLM)
      participant OHC as OHC Platform
      participant Stripe as Stripe Checkout

      Customer->>Webhook: "I need a quote for a logo design"
      Webhook->>SalesAgent: Trigger Inquiry
      SalesAgent->>OHC: Fetch Service Catalog & Pricing
      SalesAgent-->>SalesAgent: Draft Quote & Proposal Message
      SalesAgent->>Owner: Push Notification: "Drafted Quote"
      Owner->>OHC: Review & Adjust on Mobile App
      Owner->>OHC: Tap "Approve & Send"
      OHC->>Customer: Send SMS/Email with Proposal Link
      Customer->>Stripe: Review and Pay Deposit
      Stripe->>OHC: Webhook: Payment Success
      OHC->>Owner: Push Notification: "Deposit Paid. Task created."
  ```

  ### AI Integration
  - **Sales Agent ("The Closer")**: Analyzes inbound inquiries (e.g., "I need a new logo and branding kit" or "My sink is leaking"). It queries the `Service` catalog and historical pricing, drafts a `Quote` complete with line items, and generates a personalized proposal message.
  - **Operations Agent ("The Manager")**: Once a quote is accepted and the deposit paid, it automatically creates the corresponding `Task` or `Project` in the owner's workflow and blocks the required time on the calendar.

  ### Mobile UX Flow (375px)
  1. **Owner View (Work Feed)**: A new inquiry appears in the triage feed. The Sales Agent attaches a "Drafted Quote" card.
  2. **Review & Approve**: The owner taps the card to view the quote in a clean, scrollable interface. Line items can be adjusted with native numerical keypads.
  3. **Send & Collect**: The owner taps "Send." The customer receives a mobile-optimized link (SMS or email) to view the proposal.
  4. **Customer View**: A responsive web view where the customer can accept the quote and immediately pay the deposit via Stripe Checkout (Apple Pay/Google Pay supported).

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Agentic Quoting & Proposal Engine
  **Target Persona**: Nora (Agency Principal) & Carlos (Handyman)
  **Outcome**: When an inquiry arrives, the Sales Agent drafts a professional quote. Nora or Carlos can review, adjust, and send it from their phone in under 30 seconds. Upon customer acceptance, a deposit is automatically collected and the project is logged.

  **Next Actions**:
  1. Implement the core Data Models (`Quote`, `QuoteLineItem`, `QuoteTemplate`) with strict multi-tenant isolation in PostgreSQL.
  2. Develop the Sales Agent capability to parse natural language inquiries, match them against the service catalog, and draft a structured quote object.
  3. Create the Mobile-First Owner UX for reviewing and editing drafted quotes within the Agent Feed.
  4. Develop the public-facing customer quote acceptance page, integrated directly with Stripe Checkout for seamless deposit collection.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
