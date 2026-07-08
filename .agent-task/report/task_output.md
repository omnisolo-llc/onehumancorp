issue_title: "Implement 'The Ambassador' Agent: Native Social Inbox Auto-Responder"
issue_description: |
  # Research Report: The Ambassador Agent

  ## Executive Summary
  This report details the need for an automated DM response system for non-technical small business owners like Maya the Baker. The Ambassador Agent will intercept social media DMs, classify intents, build context via RAG, and propose drafted replies for the owner to approve on their mobile device.

  ## 1. Problem Statement
  Solopreneurs like Maya miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience.

  ## 2. Architecture & Design Flow

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Instagram API
      participant Ambassador Agent
      participant OHC Database
      participant Owner Mobile App

      Customer->>Instagram API: Send DM (e.g., "Vegan cake availability?")
      Instagram API->>Ambassador Agent: Webhook Event
      Ambassador Agent->>OHC Database: Retrieve RAG Context (Inventory, FAQs)
      Ambassador Agent->>Ambassador Agent: LLM Intent Classification & Draft Generation
      Ambassador Agent->>Owner Mobile App: Push Notification & Draft Card
      Owner Mobile App->>Owner Mobile App: Tap "Approve & Send"
      Owner Mobile App->>Ambassador Agent: Approval Confirmation
      Ambassador Agent->>Instagram API: Send Reply DM
      Instagram API->>Customer: Reply DM
  ```

  - **Data Ingestion**: Webhooks connected to Instagram Graph API.
  - **Processing Layer**: LLM intent classification (Is this a pricing inquiry, availability check, or general support?).
  - **Context Generation**: RAG pipeline retrieving Maya's inventory count, store policies, and FAQ embeddings.
  - **Draft Generation**: Agent generates a contextually accurate reply.
  - **Mobile UX**: Pushes a notification to Maya. The OHC mobile app displays a 375px card showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.

  ## 3. Implementation Prompt
  **Feature Name:** The Ambassador - Native Social Inbox Auto-Responder
  **Target Persona:** Maya the Baker (relies on Instagram DMs, overwhelmed by volume).

  **Outcome:** An automated DM response system where the AI agent drafts replies based on inventory and business rules. Maya can review and approve them directly from her iPhone.

  **Critical User Journey (CUJ):**
  1. Maya logs into the OHC mobile web app (375px view).
  2. Maya connects her Instagram Business account via the Integrations tab.
  3. A customer DMs Maya on Instagram: "Do you have vegan chocolate cake available for Saturday?"
  4. The Ambassador Agent queries Maya's OHC inventory, confirms availability, and drafts: "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?"
  5. Maya receives a push notification on her phone: "Agent drafted a reply to @customer. Tap to review."
  6. Maya taps the notification, sees the draft, and clicks "Approve". The message is sent.

  **Next Actions for Engineering:**
  - Integrate Instagram Graph API for message receiving/sending.
  - Implement intent classification using Gemini Pro.
  - Implement RAG retrieval for context building.
  - Build the mobile-first (375px) notification card UX for approval.
  - Do NOT prescribe database schemas here. Focus on the seamless connection between the webhook, the LLM, and the user's mobile feed.

  **Priority:** P0
  **Estimated Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
