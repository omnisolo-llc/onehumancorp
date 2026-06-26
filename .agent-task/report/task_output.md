issue_title: "Implement the Ambassador Agent: Native Social Inbox Auto-Responder"
issue_description: |
  # Title: Implement the Ambassador Agent - Native Social Inbox Auto-Responder

  ## Problem Statement
  Solopreneurs and small business owners (e.g., Maya the Home Baker) miss critical sales because they cannot actively monitor and respond to social media DMs (like Instagram or WhatsApp) while running physical operations such as baking or deliveries. Existing solutions require configuring complex logic builders (like ManyChat) which are far too technical for non-technical users. They need an assistant that automatically drafts contextual replies to customer inquiries based on their specific business data (inventory, policies) and presents them for a simple 1-tap approval on a mobile device.

  ## Research Report
  Based on our market analysis of SMB platforms, existing tools are mostly reactive:
  - **Shopify**: Sidekick is a reactive chatbot that requires the user to initiate action. Third-party apps are required for social auto-responders, adding to the "App Tax".
  - **Wix & Squarespace**: Offer basic CRM and inbox features, but lack proactive, agent-driven auto-responders that understand inventory context.
  - **GoDaddy**: Focuses on simple setup but lacks deep operational or conversational AI tools.

  There is a massive white space for a proactive agent that acts as a "Customer Success" department. The "Ambassador Agent" will ingest incoming messages, classify the intent, query the tenant's specific business context via a RAG pipeline, and generate a drafted response. This drafted response is pushed to the owner's mobile device as an Action Card, where they can approve, edit, or discard the message. This transforms the owner from an operator constantly typing out the same answers to an approver of AI-generated work.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WebhookIngestion
      participant EventBus
      participant AmbassadorAgent
      participant RAG_Pipeline
      participant MobileClient

      Customer->>WebhookIngestion: Sends DM (e.g., "Do you have vegan cakes?")
      WebhookIngestion->>EventBus: Publishes NewMessageEvent
      EventBus->>AmbassadorAgent: Triggers Agent
      AmbassadorAgent->>RAG_Pipeline: Query tenant context (Inventory/Policies)
      RAG_Pipeline-->>AmbassadorAgent: Returns context (Vegan cakes in stock)
      AmbassadorAgent->>AmbassadorAgent: Drafts reply
      AmbassadorAgent->>EventBus: Publishes DraftReadyEvent
      EventBus->>MobileClient: Pushes Notification & Action Card
      MobileClient->>Owner: "Drafted reply to Customer. Tap to review."
      Owner->>MobileClient: Taps "Approve"
      MobileClient->>WebhookIngestion: Sends Approval
      WebhookIngestion->>Customer: Sends Reply
  ```

  ### UI Wireframes & Screen Flow Description (375px first)
  1. **Push Notification Screen**: A standard native push notification stating "Ambassador Agent drafted a reply to @customer. Tap to review."
  2. **Action Card Feed (Home Screen)**: The owner opens the app and sees a feed of pending items. The top item is an Action Card for the DM.
  3. **Action Card Detail**:
     - Top section: Customer avatar, name, and their message bubble (e.g., "Do you have vegan cakes?").
     - Middle section: AI-drafted reply bubble in a distinct color/style (e.g., "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?").
     - Bottom section: Contextual tags used by the AI (e.g., "Inventory: 3 Vegan Cakes").
     - Footer: Three large, touch-friendly buttons side-by-side or stacked: "Approve & Send" (Primary), "Edit" (Secondary), "Discard" (Tertiary).
  4. **Edit State**: Tapping "Edit" transforms the drafted reply bubble into an active text area with the native keyboard opened.

  ### Mobile UX Flow
  - **Notification**: The owner receives a push notification on their phone.
  - **Action Card**: Tapping the notification opens the OHC app to an Action Card.
  - **Interactions**: Primary interactions must have touch targets >= 44x44px. The flow should require no horizontal scrolling and fit perfectly within a 375px width screen. The user reviews the draft, taps "Approve", and receives a satisfying, subtle haptic/visual confirmation that the task is complete and the card is removed from the feed.

  ### AI Agent Integration Points
  - **LLM Provider**: Utilize the existing abstraction layer (Gemini Pro primary, with fallback).
  - **System Prompt**: The Ambassador Agent will need a specific system prompt defining its role, tone, and constraints (e.g., "You are a customer success assistant for [Business Name]. Keep responses concise and friendly. Do not invent inventory or policies.").
  - **Tenant Memory**: Utilize tenant-scoped memory to maintain conversation history with specific customers to ensure continuity.

  ### Key Design Decisions and Why
  - **Action Cards over Chat Interface**: We chose an Action Card feed instead of a traditional chat interface for the owner because the goal is to approve work, not engage in a conversation with the AI. Action Cards reduce cognitive load and clearly present the proposed action.
  - **Event Bus Architecture**: Using a central event bus (Redis Pub/Sub or Postgres SKIP LOCKED) ensures reliability. If the mobile client is offline, the draft remains pending until they reconnect. It also decouples ingestion from generation.
  - **No Complex Rules Engine**: We explicitly avoided a drag-and-drop logic builder (like Zapier or ManyChat). The LLM handles intent and context matching, minimizing setup friction for the non-technical owner.

  ## Implementation Prompt
  **Objective**: Implement the core backend infrastructure and mobile UX for the Ambassador Agent's auto-responder workflow.

  **Persona**: Maya the Baker

  **Critical User Journey (CUJ)**:
  1. Maya connects her social account (simulated via webhook integration for MVP).
  2. A customer sends a message asking about availability.
  3. The system ingests the message, classifies the intent, and checks inventory.
  4. The Ambassador Agent drafts a reply confirming availability.
  5. Maya receives a notification and views the drafted reply on an Action Card in her 375px mobile feed.
  6. Maya taps "Approve & Send," and the response is dispatched.

  **Acceptance Criteria**:
  - Implement the webhook listener and event routing for incoming messages.
  - Implement the LLM intent classification and draft generation logic for the Ambassador Agent.
  - Build the mobile-first (375px) Action Card UI in the Flutter/Frontend application, including the Approve, Edit, and Discard actions.
  - Ensure the drafted message is correctly stored and linked to the tenant and customer.
  - Verify the entire flow via end-to-end (E2E) Playwright tests, demonstrating the receipt of a message, draft generation, and successful approval.
  - **Do NOT prescribe specific database schemas or API endpoints.** Design those as needed to fulfill the CUJ, ensuring strict multi-tenant isolation.
  - The UI must use real data (no mock data in the frontend components).

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
