issue_title: "Implement Universal Agentic Omnichannel Inbox & Auto-Negotiator"
issue_description: |
  ## 1. Problem Statement
  Small business owners and field service operators (like Carlos the Handyman or Maya the Baker) frequently lose up to 30% of potential leads because they are busy "on the job" or away from their desks. They cannot continuously monitor Instagram DMs, SMS, WhatsApp, and Web Chat. Current CRMs require manual triage, which defeats the purpose for a non-technical owner who needs an assistant, not another dashboard to check. The platform lacks a unified event ingestion pipeline and an autonomous agent that not only categorizes incoming inquiries but actively negotiates prices, schedules bookings, and securely collects deposits on behalf of the owner.

  ## 2. Research Report
  ### Competitor Analysis
  - **HubSpot Breeze / Intercom Fin:** Powerful but aimed at enterprise support teams, requiring extensive pre-configuration of decision trees.
  - **Shopify Sidekick:** Excellent at advising on commerce metrics but does not actively negotiate with inbound customers over external channels like Instagram DMs or SMS.
  - **11x.ai (Alice/Julian) & Lindy.ai:** Autonomous workers that handle inbound/outbound calls and scheduling, but they lack native integration into a hybrid commerce+booking multi-tenant POS layer.

  ### Pain Points (OHC Personas)
  - **Carlos (Handyman):** Needs a system to respond to an SMS, quote a standard repair price, and take a $50 deposit to secure a slot while he is fixing a sink.
  - **Maya (Baker):** Gets DMs overnight about custom cakes. Needs an agent to confirm availability, collect details, and send a Stripe Payment Link before she wakes up.

  ### Opportunity
  Implement an "Ambassador" Auto-Negotiator Agent that intercepts omnichannel messages, uses RAG (Retrieval-Augmented Generation) against the tenant's inventory/calendar, drafts a response, and executes state changes (creating quotes/bookings/payment links) autonomously or via a 1-tap approval from the owner's Agent Feed.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (IG/SMS)
      participant WH as Omnichannel Webhook Gateway
      participant Q as AI Job Queue (PgSQL SKIP LOCKED)
      participant A as Ambassador Agent (LLM + RAG)
      participant DB as Central Ledger (PgSQL)
      participant O as Owner (OHC Mobile App)

      C->>WH: "Can you fix a pipe tomorrow?"
      WH->>Q: Enqueue Message Event
      Q->>A: Dequeue & Process
      A->>DB: Query Availability & Pricing
      A-->>A: Classify Intent (Booking Request)
      A->>DB: Generate Quote & Payment Link
      A->>O: Push "Action Card" to Agent Feed
      O->>O: Reviews drafted reply & Quote (1 tap)
      O->>A: Approve Action
      A->>C: "Yes, I can. It's $150. Here is the link to deposit."
  ```

  ### Mobile UX Flow (375px First)
  1. **Agent Feed (Home Screen):** The owner opens the app and sees a Translucent Glass Action Card: "New Lead: Pipe Repair. Agent drafted quote for $150. [Approve & Send] [Edit]".
  2. **Approval:** Tapping "Approve & Send" instantly dispatches the omnichannel reply with a Stripe Payment link.
  3. **Auto-Pilot Toggle:** For trusted standard services, the owner can enable "Auto-Pilot" allowing the Ambassador agent to respond and book without explicit approval.

  ### AI Agent Integration Points
  - **Intent Classifier:** LLM pipeline to categorize messages (`booking_inquiry`, `support_issue`, `spam`, `pricing_question`).
  - **Memory Graph:** Updates the `customer_memory_graph` with context (e.g., "Customer needs plumbing help").
  - **RAG Sync:** Vector search against tenant services, pricing tables, and scheduling availability.

  ### Key Design Decisions
  - Use **PostgreSQL `SKIP LOCKED`** for the AI Job Queue to ensure robust, transactional webhook processing without introducing heavy dependencies like Kafka.
  - Maintain strict **row-level security (RLS)** in PostgreSQL using `tenant_id` to isolate conversational memory and business logic.
  - Design the Agent Feed cards using **macOS-style Translucent Glass** (`rgba(255,255,255,0.65)`, `backdrop-filter: blur(30px)`) to provide a premium, clear UI hierarchy.

  ## 4. Implementation Prompt
  **User Outcome:** As an owner, I want my OHC assistant to automatically read incoming DMs or SMS, check my calendar/pricing, and draft a response with a payment link in my Agent Feed, so I can secure new business with one tap while working.

  **Tasks for Implementer:**
  1. Create the `omnichannel_webhook` endpoint to ingest incoming messages and enqueue them securely into the PostgreSQL-backed AI Job Queue.
  2. Implement the `Ambassador` agent logic to dequeue messages, run RAG against tenant data (pricing, schedule), and generate a drafted reply payload.
  3. Design the Mobile-First (375px) Agent Feed UI in the frontend (using Translucent Glass tokens) to display these drafted replies as actionable cards (Approve / Edit / Discard).
  4. Ensure end-to-end integration: Approving the card dispatches the message back out and records the interaction in the `customer_memory_graph`.

  **Acceptance Criteria:**
  - Full E2E Playwright test simulating an incoming message, processing by the agent, and approval via the Agent Feed UI.
  - The UI must render correctly on a 375px viewport with >= 44x44px touch targets.
  - 100% unit test coverage for the webhook and agent intent classification logic.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical path for "AI Assistant" value proposition)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
