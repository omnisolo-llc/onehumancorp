issue_title: "Implement 'The Ambassador' AI Customer Success Agent"
issue_description: |
  ## Title
  The Ambassador: AI Unified Inbox & Proactive Contextual Response Agent

  ## Problem Statement
  Small business owners (Maya the baker, Carlos the handyman) lose potential revenue daily because they cannot manage fragmented communication channels (Instagram DMs, WhatsApp, SMS, Web Chat) while actively performing their work. Current "unified inboxes" simply aggregate messages; they still require the owner to manually read, gather context (e.g., checking order history or calendar availability), and type a response. This creates an unscalable bottleneck, delays responses, and frustrates customers.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional Aggregators (Shopify Inbox, Wix Inbox):** Combine channels but rely entirely on manual human replies or rigid, keyword-based auto-responses. They do not autonomously generate contextual responses based on full customer history.
  - **Enterprise Solutions (Zendesk AI, Intercom):** Offer powerful AI but are prohibitively complex to set up and too expensive for solopreneurs.
  - **AI-Native Competitors (Lindy.ai, Relevance AI):** Demonstrate the demand for autonomous agents but are decoupled from the core business data (inventory, orders, calendar).
  - **OHC Opportunity:** Leverage OHC's position as the central operating system. "The Ambassador" agent doesn't just aggregate; it deeply understands the OHC data model. It reads incoming messages, queries the unified customer graph (past orders, active bookings, preferences), checks inventory/availability, and proactively drafts a complete, accurate response. The owner simply taps "Approve" in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[SMS] -->|Webhook| B
      B --> E{Customer Identity Engine}
      E -->|Resolve Identity| F[(PostgreSQL: Unified Customer Graph)]
      E --> G[Event Bus: New Message]
      G --> H[The Ambassador Agent (Gemini Pro)]
      H -->|Query Context| F
      H -->|Query Operations| I[(PostgreSQL: Orders/Inventory/Calendar)]
      H -->|Draft Generated| J[Action Queue]
      J --> K[Mobile App: Owner Feed 375px]
      K -->|1-Tap Approve/Edit| L[Omnichannel Dispatcher]
      L -->|Send Reply| B
  ```

  ### Mobile UX Flow (375px)
  1. Maya is baking and receives an Instagram DM: "Can I get the same vegan cake I ordered last month for this Saturday?"
  2. The Omnichannel Gateway receives the webhook.
  3. The Ambassador Agent queries the Customer Graph (matches IG handle to phone number to past order #1042), queries Inventory/Schedule (checks Saturday capacity).
  4. The Ambassador drafts: "Hi Sarah! I'd love to make another Vegan Chocolate Delight for you. I have an opening this Saturday. Shall I send over the $40 deposit link to confirm?"
  5. Maya opens the OHC app. At the top of her "Action Required" feed is a card: **[Draft Reply] Sarah (IG)** with the drafted text.
  6. Maya taps "Approve & Send."

  ### AI Agent Integration Points
  - **Agent:** The Ambassador (Customer Success Agent).
  - **Triggers:** Webhooks from messaging channels via Omnichannel Gateway.
  - **Capabilities:** Requires read-only access to customer profiles, order history, and schedules. Requires write access to draft responses in the Action Queue.

  ### Key Design Decisions
  - **Human-in-the-Loop Default:** For v1, all agent-generated replies MUST be routed to the Owner Feed as drafts for approval. Full autonomy is a future opt-in setting.
  - **Tenant-Scoped Memory:** The LLM context window must be strictly populated only with data belonging to the specific `tenant_id` associated with the incoming message.
  - **Identity Resolution:** The system must merge customer identities (e.g., if a known customer with an email address sends a WhatsApp message for the first time, it should link them if identifiable, otherwise treat as a new lead).

  ## Implementation Prompt
  **Role:** Backend & AI Agent Implementer
  **Task:** Build the core logic for "The Ambassador" agent to receive simulated omnichannel messages, query customer context, and generate draft responses.
  **CUJ:**
  1. An external system posts a simulated incoming message to the `OmnichannelGateway` API.
  2. The system identifies the customer and retrieves their recent order history.
  3. The Ambassador Agent (using the configured LLM) generates a contextual draft response based on the message and history.
  4. The draft is persisted to the database and appears in the owner's Action Feed API endpoint.
  5. The owner calls the `ApproveDraft` API endpoint, triggering the simulated dispatch.
  **Acceptance Criteria:**
  - Implement the `OmnichannelGateway` to receive generic message payloads.
  - Implement The Ambassador agent using the existing LLM provider abstraction, injecting customer context into the system prompt.
  - Ensure all database queries strictly enforce `tenant_id` isolation.
  - Provide unit tests mocking the LLM response.
  - Provide a Playwright E2E test (or API integration test) demonstrating the full flow from incoming message to approved draft, verifying the final response content.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
