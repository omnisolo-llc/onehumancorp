issue_title: "Build Agentic Smart Quoting & Autonomous Invoicing Engine"
issue_description: |
  # Research Report: Agentic Smart Quoting & Autonomous Invoicing Engine

  ## 1. Problem Statement & Dogfooding Evidence
  Service-based businesses (e.g., Carlos the Field Service Owner, Nora the Agency Principal) often spend hours each week manually drafting quotes and converting them into invoices. The typical process is highly fragmented.

  **Dogfooding Evidence:**
  During a simulated walkthrough using a Playwright script targeting the local `server` instance (started via `docker compose up`), the following gap was observed in the Critical User Journey (CUJ):
  - **Persona Attempted:** Carlos (Handyman).
  - **Action:** Attempted to convert an incoming message inquiry ("How much to fix a leaky pipe this Friday?") into a formal, structured quote.
  - **Observed Gap:** The system provided a unified inbox view but lacked any button or automated flow to generate a quote directly from the message context. I had to manually navigate away from the message, create a new document, and type out the customer details and line items, breaking the "Agentic" promise of the platform.

  ## 2. Research Report
  ### Market Context & Competitor Discovery
  - **Traditional CRM/Invoicing Tools (Quickbooks, Freshbooks, HubSpot):** These tools require the user to manually enter line items. While they offer workflow automations (like recurring invoices), they do not autonomously *draft* the initial quote based on unstructured customer inquiry (e.g., an Instagram DM).
  - **Square / Stripe Invoicing:** Excellent payment collection, but still relies on manual quote creation.
  - **AI-Native Solutions:** Tools like 11x.ai or relevanceai.com are exploring autonomous digital workers, but they are often enterprise-focused and too complex/expensive for a single-person operation like Carlos.

  ### The OHC Gap & Opportunity
  OHC currently lacks an end-to-end autonomous quoting engine that seamlessly bridges the gap between the "Customer Success Agent" (The Ambassador) and the "Finance Agent" (The Accountant). By implementing the **Agentic Smart Quoting & Autonomous Invoicing Engine**, OHC can capture unstructured demand (messages) and transform it into structured, actionable financial documents with zero manual data entry.

  ## 3. Design Doc
  ### Architecture & Data Model (PostgreSQL)

  #### Entity Relationship Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Customer : manages
      Tenant ||--o{ Quote : issues
      Customer ||--o{ Quote : receives
      Quote ||--o{ QuoteLineItem : contains
      Quote ||--o| Invoice : generates

      Tenant {
          uuid id PK
          string name
      }
      Customer {
          uuid id PK
          uuid tenant_id FK
          string contact_info
      }
      Quote {
          uuid id PK
          uuid tenant_id FK
          uuid customer_id FK
          string status
          string unstructured_context
          timestamp created_at
      }
      QuoteLineItem {
          uuid id PK
          uuid quote_id FK
          string description
          int quantity
          decimal unit_price
      }
      Invoice {
          uuid id PK
          uuid quote_id FK
          string payment_link
          string status
      }
  ```
  - **Multi-Tenant Isolation:** All tables will strictly use `tenant_id` with Row Level Security (RLS) enabled.

  ### AI Agent Integration & Coordination (Sequence Diagram)
  ```mermaid
  sequenceDiagram
      actor Customer
      participant Gateway as Omnichannel Gateway
      participant Ambassador as Customer Success Agent
      participant QuotingAgent as Quoting Agent (Negotiator)
      participant DB as PostgreSQL (Quote)
      participant Feed as Mobile Agent Feed (Owner)
      participant Accountant as Finance Agent
      participant Stripe as Stripe API

      Customer->>Gateway: "How much to fix a leaky pipe?"
      Gateway->>Ambassador: Ingest Message
      Ambassador->>QuotingAgent: Route to Quoting/Sales
      QuotingAgent->>DB: Query Historical Quotes & Pricing
      QuotingAgent->>DB: Draft Quote & Line Items
      QuotingAgent->>Feed: Push "Action Required: Review Quote"
      Note over Feed: Owner reviews on 375px mobile UI
      Feed->>Customer: Owner Approves -> Send Quote Link
      Customer->>Feed: Customer Accepts Quote
      Feed->>Accountant: Trigger Invoice Generation
      Accountant->>Stripe: Create Payment Link
      Stripe-->>Accountant: Return Link URL
      Accountant->>DB: Save Invoice Record
      Accountant->>Customer: Send Invoice/Payment Link
  ```

  ### Mobile UX Flow (375px First)
  1. **Agent Feed (Owner View):** Carlos receives a high-priority card: *"Drafted Quote for John: Leaky Pipe Repair - $150. [Review & Send] [Discard]"*
  2. **Quote Review Screen:** Tapping the card opens a clean, thumb-friendly view. Carlos sees the parsed details, suggested price (derived from historical data), and a 1-tap "Send Quote" button.
  3. **Client View:** The client receives an SMS/Email link leading to an OHC-hosted, mobile-optimized Quote page with an "Accept & Pay Deposit" button powered by Stripe Checkout.
  4. **Post-Acceptance:** Upon acceptance, the OHC system seamlessly converts the quote to a scheduled invoice.

  ### Key Design Decisions
  - **Proactive Drafting:** The system drafts the quote *before* the owner opens the app.
  - **Mobile-First Approval:** The owner's primary interaction is tapping "Approve" on a pre-filled card, not typing out line items on a phone keyboard.
  - **Historical Pricing RAG:** The quoting agent will use embeddings of past accepted quotes to learn and suggest accurate pricing over time, eliminating the need for rigid price books if the owner's pricing is dynamic.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** When a customer sends a message requesting a service, the business owner opens the OHC app to find a fully drafted, accurately priced quote waiting in their Agent Feed. With one tap, they approve and send the quote. Once the customer accepts, it automatically becomes a payable invoice.

  **CUJ & Acceptance Criteria:**
  1. An external inquiry (e.g., simulated webhook) detailing a service request is ingested.
  2. The Quoting Agent successfully parses the intent, queries the product/service catalog or historical data, and drafts a `Quote` record.
  3. An Action Card appears in the Agent Feed on a simulated 375px mobile viewport.
  4. The user (owner) taps "Approve" via Playwright E2E tests, which triggers the system to send the quote link to the mocked customer.
  5. When the mocked customer accepts the quote, the system automatically generates an `Invoice` and readies a Stripe Payment Link.
  6. **Testing:** Provide comprehensive unit tests for the quoting logic and Playwright E2E tests verifying the 375px mobile review-and-approve flow. No mocked UI state or fake internal network calls are allowed.

  ## 5. Priority & Scope
  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
