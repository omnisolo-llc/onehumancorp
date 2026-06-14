issue_title: "OHC Unified Inbox & AI Communication Auto-Drafting"
issue_description: |
  # Research Report: OHC Unified Inbox & AI Communication Auto-Drafting

  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by fragmented communication channels. They receive inquiries via Instagram DMs, WhatsApp, SMS, email, and website contact forms. Responding promptly is critical for conversion, but managing multiple apps while running the physical business leads to missed opportunities and lost revenue. Existing solutions (like Zendesk or Intercom) are too complex, expensive, and geared towards enterprise support teams, not solopreneurs operating from a mobile phone. Furthermore, these tools are reactive; they don't actively help the owner draft the response based on business context.

  ## Research Report
  - **Competitor Analysis:**
    - *Shopify:* Offers "Shopify Inbox", which aggregates some channels but lacks deep, proactive AI drafting based on real-time inventory or scheduling data. It still requires the user to do the heavy lifting of composing the reply.
    - *Zendesk/Intercom:* Enterprise-grade, complex setup, high cost. Overkill for a sole operator.
    - *ManyChat/Chatbots:* Rely on rigid, user-built decision trees. If a customer asks a question outside the flow, the bot fails gracefully. This requires technical setup that the target persona cannot manage.
  - **The OHC Opportunity:**
    - OHC's value proposition is the "Invisible AI Automation." By centralizing all communication into a single feed, the Customer Success Agent ("The Ambassador") can intercept every message, classify its intent using an LLM, RAG against the business's data (inventory, FAQs, policies), and present the owner with a pre-drafted, contextually accurate reply in a simple "Approve/Edit/Discard" mobile card.

  ## Design Doc
  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
      A[Instagram API] -->|Webhook| E(Message Ingestion Queue)
      B[WhatsApp API] -->|Webhook| E
      C[Website Form] -->|API| E
      D[Email/SMS] -->|Webhook| E
      E --> F[Intent Classification LLM]
      F --> G{RAG Pipeline}
      G -->|Inventory/FAQ/Policy Data| H[Draft Generation LLM]
      H --> I[Unified Inbox DB Table]
      I --> J[Push Notification to Owner]
      J --> K[Mobile App: Approve/Edit Card]
  ```

  ### Mobile UX Flow (375px)
  1. **Notification:** Owner receives a push: "New inquiry from @user about Vegan Cakes. Agent has drafted a reply."
  2. **Unified Feed:** Tapping the notification opens the OHC app to the Unified Inbox feed. The UI is a clean, vertically scrolling list of conversation cards.
  3. **Action Card:** The specific card shows:
     - The customer's message.
     - The channel icon (e.g., Instagram).
     - The AI-drafted reply (e.g., "Yes, we have 2 vegan chocolate cakes left! Here is the link to reserve one: [Link]").
     - Large, touch-friendly (min 44x44px) buttons: **Approve & Send**, **Edit**, **Discard**.
  4. **Execution:** Tapping "Approve" instantly sends the reply back through the native channel via the backend integration.

  ### AI Agent Integration
  - **The Ambassador (Customer Success Agent):** The core intelligence behind this feature. It must:
    1. Parse incoming unstructured text.
    2. Determine if the message requires a response (e.g., ignoring "Thanks!").
    3. Query the necessary data (e.g., checking PostgreSQL inventory for "Vegan Cake").
    4. Generate a polite, brand-aligned response.
    5. Learn from owner edits to improve future drafts.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Inbox & AI Communication Auto-Drafting
  **Target Persona:** Maya the Baker

  **Outcome:** Maya connects her Instagram and WhatsApp to OHC. When a customer messages her asking about product availability, the OHC Ambassador Agent automatically drafts a response checking real-time inventory and presents it to Maya in the OHC mobile app for a one-tap approval.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  1. Implement a unified `Message` data model in PostgreSQL that can store interactions from multiple channels, ensuring strict multi-tenant isolation.
  2. Create an ingestion endpoint (webhook handler) that can receive simulated incoming messages.
  3. Integrate the LLM (Gemini/MiniMax) and a basic RAG flow to process the incoming message, query mock inventory/FAQ data, and generate a draft reply.
  4. Build the mobile-first (375px) "Unified Inbox" UI in the app. It must display the incoming message and the AI-drafted reply within a translucent glassmorphism card.
  5. The card must have functional "Approve", "Edit", and "Discard" actions. "Approve" should update the message state in the DB.
  6. **Zero mock data in UI:** The UI must be driven entirely by data from the backend DB.
  7. Provide Playwright E2E tests covering the complete flow: A message is ingested -> the backend drafts a reply -> the UI displays the draft -> the user approves it -> the state is updated.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
