issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chat system"
issue_description: |
  # Replace Chat system with Native Rust Omnichannel Chat System

  ## Problem Statement
  Currently, OneHumanCorp (OHC) relies on Chat system as an external third-party service for its omnichannel customer support and chat functionality. The OHC Engineering Standards strictly mandate that Chat system must be 100% retired. Relying on an external service creates friction for a small-business owner who expects a seamlessly integrated, assistant-first work platform. We need our own high-performance, multi-tenant chat engine natively built in Rust inside `onehumancorp/mono`. This new system must achieve 100% feature parity with Chat system, managing omnichannel communications (web widget, email, SMS, Instagram DMs, etc.) invisibly, so our users—like Maya the baker and Carlos the handyman—can triage customer inquiries effortlessly without juggling external admin portals.

  ## Research Report
  - **Codebase & Architecture Gap:** The project instructions indicate that Chat system is being retired in favor of a native Rust implementation. Our current discovery found placeholders and settings for a chat service (`src/server/services/chat`) but not a fully replicated omnichannel architecture matching Chat system's capabilities.
  - **Chat system Source Code Audit:** Chat system's core architecture relies on:
    - **Omnichannel Models:** Concepts like `Inbox`, `Conversation`, `Contact`, `Message`, and `ChannelAdapter`.
    - **Real-time Messaging:** WebSocket connections for live chat and notifications.
    - **Web Widget:** A lightweight frontend chat interface embeddable on websites.
    - **Advanced Support Features:** SLA policies, macros, canned responses, and agent routing algorithms.
  - **Competitor Insights:** Systems like Intercom and Shopify Inbox leverage highly concurrent, event-driven architectures to process multi-channel messages in real time. Building this in Rust allows OHC to benefit from memory safety, high concurrency, and low latency—critical for maintaining our strict performance and offline targets.

  ## Design Doc
  ### High-Level Architecture (Native Rust Chat Engine)
  1.  **Data Model & Persistence:**
      -   **PostgreSQL:** Store multi-tenant entities with row-level security (`tenant_id`). Entities: `Conversation`, `Message` (with polymorphic `channel_type`), `Inbox`, `Contact`, `SlaPolicy`, `CannedResponse`.
      -   **Redis:** Pub/Sub for real-time message distribution across multiple service instances. Cache for active WebSocket connections and conversation states.
  2.  **Service Layer (Rust Microservices):**
      -   **Channel Adapters:** Modular Rust crates handling specific integrations (e.g., `InstagramDMAdapter`, `WebWidgetAdapter`, `EmailAdapter`).
      -   **Routing Engine:** Assigns conversations to specific AI Agents (or human operators, if configured) based on load and context.
      -   **WebSocket Server:** Natively implemented in Rust (using libraries like `tokio-tungstenite` or `axum` WebSockets) to push updates to the OHC Frontend and the Web Widget.
  3.  **Frontend (Flutter + Next.js Web Widget):**
      -   **Owner Inbox UI (Flutter PWA):** Clean, translucent glass UI integrating with the existing "Work Triage" view.
      -   **Web Chat Widget:** A lightweight, embeddable React/Next.js widget for the owner's storefront.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          string sender_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }
  ```

  ### Mobile UX Flow (375px First)
  -   **Triage Integration:** The owner opens the app and sees incoming messages from all channels unified in the existing "Work Triage" feed.
  -   **Conversation View:** Tapping a message opens a clean, chat-style interface. AI drafts are presented inline as "suggested replies" (Action Buttons: "Approve as-is", "Edit", "Dismiss").
  -   **Context Drawer:** A swipe from the right reveals the customer's CRM profile, past orders, and notes—all fitting within a 375px width without horizontal scrolling.

  ### AI Agent Integration
  -   **Customer Assistant Agent:** Listens to the incoming message stream via internal pub/sub. When a new message arrives, it retrieves context from the `CustomerMemoryGraph` and drafts a reply.
  -   **Operations Assistant:** If a message implies a task (e.g., "Cancel my order"), the agent creates a draft operational task alongside the message reply.

  ## Implementation Prompt
  Implement a native Rust omnichannel chat engine to replace Chat system.
  1.  **Data Models:** Create Rust structs and PostgreSQL schemas (with Row Level Security by `tenant_id`) for `Inbox`, `Conversation`, `Contact`, and `Message`.
  2.  **API & WebSockets:** Implement REST endpoints for managing inboxes/conversations and a WebSocket handler for real-time messaging using Axum.
  3.  **Channel Extensibility:** Define a Rust trait `ChannelAdapter` and implement a basic `WebWidgetAdapter`.
  4.  **UI Updates:** Update the Next.js and Flutter frontends to connect to the new native WebSocket endpoints instead of Chat system, ensuring the UI adheres to the macOS-style translucent glass design system and works flawlessly on 375px viewports.
  5.  **Testing:** Provide comprehensive unit tests for the Rust services and Playwright E2E tests for the new UI flows, ensuring 100% test pass rate (`bazel test //...`).

  **Estimated Scope:** Large

  **Acceptance Criteria:**
  - The external Chat system dependency is entirely removed.
  - Owners can receive and reply to messages from a native web widget through the unified Triage UI.
  - Real-time updates function correctly across browser tabs using the new Rust WebSocket server.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
