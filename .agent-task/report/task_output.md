issue_title: "Implement Intelligent Omni-Channel Work Triage Engine"
issue_description: |
  # Intelligent Omni-Channel Work Triage Engine

  ## Problem Statement
  Small business owners like Maya (the baker) and Nora (the agency principal) receive customer inquiries, booking requests, and support issues across multiple disconnected channels: Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Managing these manually leads to missed opportunities, dropped context, and constant context switching, preventing the owner from focusing on their actual work. They need a single, prioritized "Work Feed" where all inbound communications are normalized, enriched with customer context, and pre-triaged by an AI assistant proposing the next best action.

  ## Research Report
  ### Market Context
  - **Traditional Inboxes (e.g., Zendesk, Front):** Built for customer support teams with ticketing concepts, too heavy and complex for a solo operator or small team.
  - **Unified Social Inboxes (e.g., Meta Business Suite):** Limited strictly to Meta properties (FB/IG) and lacks deep business context (e.g., orders, bookings).
  - **Shopify Inbox:** Good for e-commerce, but limited primarily to web chat and basic shopify order status, missing services, bookings, and custom quotes.
  - **Tencent Workbuddy / WeCom:** The gold standard for integrated operations and chat in Asia, deeply linking customer interactions with business transactions.

  ### Opportunity
  OHC can differentiate by providing an "Assistant-First" unified inbox. It doesn't just display messages; it acts as a "Work Triage" engine. It reads the message, links it to existing customer records and past transactions, and generates a drafted reply or action (like "Send Deposit Link") for the owner to approve with one tap.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **The Triage Feed (Home Screen):** The owner opens the app and sees a unified list of "Items Needing Attention", not just raw messages.
  2. **Triage Item Card:** Clean, UniFi-style card showing:
     - Avatar/Name & Channel Icon (e.g., WhatsApp).
     - Brief summary of the request (e.g., "Wants a quote for a 2-tier wedding cake").
     - An AI-proposed action button (e.g., `[Review Draft]`, `[Create Quote]`, `[Check Calendar]`).
  3. **Interaction Detail View:** Tapping a card opens the full conversation context.
     - Top half: The chat thread.
     - Bottom half: The Assistant's proposed draft or action panel.
     - Swipe right to approve/send, swipe left to dismiss/edit.

  ### Architecture (High-Level)
  - **Channel Adapters:** Webhooks for external channels (IG, WhatsApp, Mailgun/Sendgrid) normalized into an internal `InboundMessage` event.
  - **Identity Resolution:** Matches inbound sender details (phone, email, social handle) to an existing `Customer` record within the `tenant`.
  - **AI Triage Worker (Job Queue):**
    - Triggered by `InboundMessage`.
    - Retrieves customer history and recent business context (e.g., active orders).
    - Classifies intent (Lead, Support, Booking).
    - Generates a `TriageTask` with a proposed `NextAction` (e.g., draft reply).
  - **Data Model:**
    - `conversations` (tenant_id, id, customer_id, channel)
    - `messages` (tenant_id, id, conversation_id, content, direction)
    - `triage_tasks` (tenant_id, id, message_id, status, proposed_action, ai_summary)

  ### Architecture Diagram

  ```mermaid
  flowchart TD
      A[Inbound Webhooks: IG, WhatsApp, Email] -->|Normalize| B(InboundMessage Event)
      B --> C{Identity Resolution}
      C -->|Match/Create| D[Customer Record]
      C --> E[Insert Message to DB]
      E --> F[AI Triage Job Queue: SKIP LOCKED]
      F --> G(AI Agent: Intent & Context)
      G -->|Fetch Context| H[(PostgreSQL: Past Orders/Memory)]
      G -->|Draft Action| I[Create TriageTask]
      I --> J[Owner Mobile App: Triage Feed UI]
      J --> K{Owner Action}
      K -->|Approve| L[Send Reply via Adapter]
      K -->|Dismiss| M[Archive Task]
  ```

  ### AI Agent Integration
  - **Customer & Relationship Assistant:** Acts as the processing layer. Uses an LLM to analyze the incoming message, summarize the intent, and draft a context-aware response. The drafted response is stored securely and presented to the user.
  - **Context Retrieval:** Uses RAG/Memory to fetch shop policies or past customer interactions to inform the draft.

  ## Implementation Prompt
  **Target Persona:** Maya (Home Baker) & Nora (Agency Principal)
  **Objective:** Build the core backend processing pipeline and the unified mobile-first UI for the Work Triage Engine.

  **CUJ (Critical User Journey):**
  1. A webhook simulates receiving an Instagram DM from a new customer asking "Do you make vegan cakes for this Saturday?"
  2. The backend creates a new customer record, conversation, and triggers the AI Triage Worker.
  3. The Triage Worker generates a summary and drafts a reply based on Maya's business context.
  4. Maya opens the OHC mobile app (375px view) and sees the new triage item on her dashboard.
  5. She taps the item, reviews the AI-drafted reply, and clicks "Send".

  **Acceptance Criteria:**
  - Build the necessary database tables with strict tenant-level isolation (Row Level Security).
  - Implement a PostgreSQL `SKIP LOCKED` based background job for the AI Triage Worker.
  - Create the Flutter frontend components for the Triage Feed and Detail View using translucent, premium design tokens and 44x44px touch targets.
  - E2E Playwright test must fully cover receiving the inbound message, UI presentation, and owner approval flow without mocking internal API calls.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
