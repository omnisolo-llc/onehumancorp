issue_title: "Implement Agentic Auto-Negotiator & Dynamic Quoting Engine"
issue_description: |
  # Mission Queue Protocol: Agentic Auto-Negotiator & Dynamic Quoting Engine

  ## Problem Statement
  Carlos (Field Service) and Nora (Agency Principal) lose up to 30% of their leads because they cannot instantly respond to service inquiries with accurate, customized quotes while on the job or in meetings. Traditional quoting tools require manual data entry, complex CRM workflows, and desktop interfaces. Business owners need an invisible AI agent that can intercept incoming DMs, forms, or calls, instantly calculate a quote based on availability and project parameters, and negotiate or secure a deposit autonomously.

  ## Research Report & Competitive Analysis
  Our market research into AI-native competitors (e.g., 11x.ai, Hubspot Breeze, Lindy) and traditional tools (Jobber, HoneyBook) reveals a significant gap:
  - **Traditional Service CRMs (Jobber, HoneyBook):** Provide structured form builders and PDF quote generators, but require the owner to manually review the request, build the quote, and send it. Time-to-quote is measured in hours or days.
  - **AI Chatbots:** Can answer basic questions but lack the capability to lock in pricing or handle complex availability rules.
  - **OHC Opportunity:** By leveraging the `Operations Agent` and `Sales Assistant`, OHC can intercept an Instagram DM or WhatsApp message, ask qualifying questions (e.g., "How many rooms need painting?"), calculate the price, draft the proposal, and present a Stripe Payment Link for the deposit.

  ## Architecture Design Doc
  ### Data Model & Sync Protocol
  - **Quote Ledger (PostgreSQL):** A multi-tenant `quotes` table linked to `customers` and `tenant_id`. Uses row-level security (RLS). Statuses include `draft`, `proposed`, `accepted`, `rejected`.
  - **Pricing Engine:** A dynamic rules engine where the AI accesses structured service catalog data (base price, hourly rate, material cost modifiers).
  - **Locking & Availability:** Redis Redlock is used to temporarily reserve calendar slots during the negotiation phase to prevent double-booking.

  ### AI Agent Integration
  - **Sales & Revenue Assistant ("The Negotiator"):**
    - Triggers on an incoming `Work Intake` event (DM, Form).
    - Queries the RAG context for tenant pricing guidelines.
    - Uses a `generate_quote` tool to produce a structured JSON quote and natural language response.
  - **Finance Assistant:** Generates the Stripe Payment Intent for the deposit.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      A[Customer DM / Web Form] -->|Webhook| B(Work Triage Event Bus);
      B --> C{Intent Classifier};
      C -->|Request for Quote| D[Sales Assistant Agent];
      D --> E[Query DB: Pricing Rules & Availability];
      D --> F[Draft Quote & Stripe Link];
      F --> G[Unified Agent Feed];
      G -->|Owner Approves| H[Send to Customer];
      G -->|Auto-Approve Enabled| H;
      H --> I[Redis: Lock Calendar Slot];
  ```

  ### Mobile-First UX Flow
  1. **Notification:** Carlos receives a push notification: "New Quote Drafted: $450 for 3-room painting."
  2. **Feed Card (375px viewport):** The Unified Agent Feed displays an Action Card summarizing the customer's request and the AI-generated quote.
  3. **Interaction:** The card has a minimum 44x44px touch target for "Approve & Send", "Edit", or "Reject".
  4. **Customer View:** The customer receives a mobile-optimized, glassmorphism-styled web link displaying the quote and an Apple Pay/Google Pay button for the deposit.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the Agentic Auto-Negotiator end-to-end.
  - **User-Facing Outcome:** The business owner receives actionable quote cards in their mobile feed for incoming service requests, ready for 1-tap approval.
  - **CUJ:**
    1. A simulated webhook posts a customer request (e.g., "Need a website redesign proposal") to the tenant's intake endpoint.
    2. The backend Sales Assistant processes the request, checks the tenant's base pricing, and generates a quote record in PostgreSQL.
    3. The owner sees a new quote card in the UI feed.
    4. The owner taps "Approve". The system generates a Stripe checkout link and marks the quote as sent.
  - **Acceptance Criteria:**
    - Zero mock data; use PostgreSQL for quote persistence.
    - Secure multi-tenant row-level isolation must be enforced for quotes.
    - Playwright E2E test must cover the webhook intake -> UI card approval -> status update flow.
    - Mobile UI must be 100% usable at 375px width.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
