issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  OHC aims to replace external dependencies like Chatwoot with a 100% native Rust omnichannel chat system. Relying on an external Chatwoot system violates our core architectural mandate, slows down multi-tenant integrations, and introduces security boundaries that cannot be fully secured with SPIFFE/SPIRE. To provide Maya, Carlos, and Priya a seamless, unified inbox experience directly in their 375px mobile screens, OHC needs a native, high-performance omnichannel inbox that handles real-time WebSockets, agent routing, and unified conversational data natively in Rust.

  ## Research Report
  - **Chatwoot Architecture Audit**: Analyzed `chatwoot/chatwoot`. Chatwoot uses an Inbox/Channel model where each channel (Email, WhatsApp, Facebook, Web Widget) maps to a distinct integration adapter, funneling into unified Conversations, Messages, and Contacts. Real-time delivery is powered by ActionCable (WebSockets).
  - **Competitor Systems**: Shopify Inbox and Wix Inbox both centralize multi-channel communications into a single interface tailored for mobile.
  - **Proposed Rust Model**: We will implement the Inbox, Channel, Conversation, Message, and Contact models using native Rust (Axum/Tonic). Real-time updates will be pushed via a scalable WebSocket layer (e.g., Tokio-Tungstenite).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : holds
  ```
  ### Mobile UX Flow (375px)
  1. **Unified Inbox List**: Owner opens the app to see a unified list of conversations across Instagram, Email, and Web Widget.
  2. **Conversation View**: Tapping a conversation opens a Translucent Glass-styled chat view.
  3. **AI Reply Draft**: The AI Work Assistant automatically drafts a response at the bottom, which the owner can send with one tap.
  ### AI Agent Integration
  - The **Customer & Relationship Assistant** is subscribed to the `ConversationCreated` and `MessageReceived` events. It analyzes intent and drafts replies directly into the unified inbox.
  ### Key Decisions
  - **Native Rust**: Axum + WebSockets for high throughput.
  - **SPIFFE/SPIRE**: Ensure strictly verified multi-tenant Row-Level Security for every API call and WebSocket frame.

  ## Implementation Prompt
  Implementer Agent: You are tasked with building the core Rust backend services for the Native Omnichannel Chat System in OHC.
  1. Define the multi-tenant data models (Inbox, Conversation, Message, Contact) with strictly enforced Row-Level Security.
  2. Build the REST/gRPC API endpoints to create/read conversations and messages.
  3. Implement the WebSocket server utilizing Tokio to emit real-time `message_created` events to connected clients.
  Ensure complete unit test coverage (100%) and integrate with the AI Assistant event stream so it can draft replies.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
