issue_title: "Build Native Rust Omnichannel Inbox to Replace Chatwoot"
issue_description: |
  ### Title
  Build Native Rust Omnichannel Inbox to Replace Chatwoot

  ### Problem Statement
  Business owners (like Maya the Baker or Carlos the Handyman) receive customer inquiries across many channels: Instagram DMs, WhatsApp, SMS, and website chat. Keeping track of all these messages is chaotic, leading to dropped leads and slow responses. Previously, we relied on an external tool (Chatwoot), but it introduced too much complexity, wasn't perfectly integrated with our AI agents, and didn't feel native to our multi-tenant owner assistant. Owners need a single, unified inbox right inside OHC that seamlessly handles all their conversations, allows AI agents to draft replies automatically, and works flawlessly on a 375px mobile screen without relying on external third-party messaging services.

  ### Research Report
  - **Chatwoot Source Code Audit**: Chatwoot’s architecture relies on core entities: `Inboxes`, `Conversations`, `Messages`, and `Contacts`. Channels (e.g., WhatsApp, Email, Web Widget) use adapters to normalize messages into a standard format. It heavily utilizes WebSockets for real-time updates and background jobs (like SLA policies and webhooks). As per OHC requirements, Chatwoot is 100% retired, and we must rebuild these capabilities natively in Rust.
  - **Competitor Benchmarking**:
    - **Shopify Inbox**: Highly integrated into the commerce flow, focusing on turning conversations into sales by easily sending product links and discount codes directly in the chat.
    - **Wix Inbox**: Provides a unified view of site interactions, forms, and chats, serving as a centralized communication hub.
  - **Opportunity**: By bringing this natively into OHC (Rust), we perfectly align the messaging system with our Zero-Trust multi-tenancy model. We can deeply embed our AI assistants directly into the event stream, allowing for instant draft generation, sentiment analysis, and intent extraction without relying on fragile third-party webhooks.

  ### Design Doc

  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : involves
      TENANT ||--o{ CONTACT : owns
      MESSAGE ||--o{ ATTACHMENT : includes

      %% AI Integration
      CONVERSATION ||--o{ AI_DRAFT_REPLY : generated_by
  ```
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Channel (e.g. WhatsApp)
      participant OHC_Webhooks
      participant Rust_Omnichannel_Engine
      participant AI_Customer_Assistant
      participant Owner_App (Tauri/Mobile)

      Customer->>Channel: Send message
      Channel->>OHC_Webhooks: POST payload
      OHC_Webhooks->>Rust_Omnichannel_Engine: Normalize & Save Message
      Rust_Omnichannel_Engine->>Owner_App: WebSocket (New Message Event)
      Rust_Omnichannel_Engine->>AI_Customer_Assistant: Trigger Draft Generation
      AI_Customer_Assistant-->>Rust_Omnichannel_Engine: Save Draft Reply
      Rust_Omnichannel_Engine->>Owner_App: WebSocket (Draft Ready)
      Owner_App->>Owner_App: Owner Reviews Draft
      Owner_App->>Rust_Omnichannel_Engine: Approve & Send
      Rust_Omnichannel_Engine->>Channel: Deliver Message
      Channel->>Customer: Message Delivered
  ```

  #### UI Wireframes & 375px UX Flow
  1. **Inbox List View**: A scrollable list of active conversations, sorted by recency and priority. Unread dots clearly indicate new activity. Touch targets are at least 44x44px.
  2. **Conversation View**: Tapping a conversation opens the chat timeline. The AI agent's drafted reply is prominently displayed above the keyboard input area in a floating glass panel, with distinct "Approve", "Edit", or "Discard" actions.
  3. **Contact Context Sheet**: Tapping the customer's avatar slides up a bottom sheet showing their past orders, tags, and preferences (e.g. Maya's vegan cake customer).
  4. **macOS-style Translucent Glass**: The UI utilizes clean UniFi-like card layouts, restrained translucent materials, and strong typographic hierarchy.

  #### AI Agent Integration Points
  - **Drafting**: Upon receiving a new message, the `AI_Customer_Assistant` automatically reads the thread context and business data to draft a reply.
  - **Summarization**: Long threads are summarized by the agent to quickly catch the owner up.
  - **Action Extraction**: The agent detects if the customer wants to book a service or buy a product, surfacing a quick-action button for the owner (e.g., "Send Deposit Link") directly in the chat UI.

  #### Key Design Decisions
  - **Native Rust & Local WebSockets**: To ensure real-time responsiveness and tight multi-tenant isolation, the messaging engine is built as native Rust microservices within `onehumancorp/mono`.
  - **Abstract Channel Adapters**: The core engine deals only with normalized messages. External platforms (WhatsApp, Instagram) are implemented as distinct channel adapters that map to the unified schema.
  - **Offline-Tolerant Mobile Design**: The mobile shell caches recent conversations and queues outgoing messages if the network drops, reconciling gracefully when back online.

  ### Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational Rust backend and mobile-first UI for the new Native Omnichannel Inbox, completely replacing any legacy Chatwoot dependencies.

  **CUJ (Critical User Journey)**:
  1. Maya (the owner) opens the OHC app on her phone (375px viewport).
  2. She navigates to the "Inbox" tab.
  3. She sees a list of conversations and taps one from a new customer asking about cakes.
  4. The conversation view loads, showing the customer's message history.
  5. An AI-drafted reply ("Hi! Yes, we do vegan cakes. What flavor are you looking for?") is waiting for her approval.
  6. She taps "Approve & Send". The message is sent, and the UI instantly updates to reflect the sent status.

  **Acceptance Criteria**:
  - The feature must be completely usable on a 375px width mobile layout without horizontal scroll.
  - The UI must contain ZERO mock data; all data must flow from the real backend.
  - The backend messaging engine must be implemented natively in Rust.
  - Every interactive element (buttons to approve/send) must function end-to-end and be verified.
  - Multi-tenancy must be strictly enforced (row-level security and Rust service scoping).
  - You MUST include at least 5 Playwright E2E tests verifying this CUJ from login to message sent.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
