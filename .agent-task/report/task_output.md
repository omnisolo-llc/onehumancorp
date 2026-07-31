issue_title: "[Architecture] Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a fully native Rust omnichannel chat system to serve our non-technical owner/operator personas (Maya, Carlos, Priya, Leo, Fatima). Chatwoot as an external dependency is 100% RETIRED. Our owners need a cohesive work assistant where conversations (Instagram DMs, WhatsApp, SMS, Web Widget) are inherently integrated with their tasks, quotes, and deposits without navigating a separate CRM tool. The new system must handle multi-tenant data isolation natively via our PostgreSQL infrastructure and seamlessly interface with OHC's AI agents.

  ## Research Report
  Based on a deep audit of the Chatwoot source code (`chatwoot/app/models` and `chatwoot/app/models/channel`), the core architecture requires replicating and improving several key domains in Rust:

  **1. Omnichannel Adapters:**
  Chatwoot supports a variety of channels (WhatsApp, Facebook Page, Instagram, SMS, Line, Telegram, TikTok, Web Widget, API). Our Rust implementation needs a unified trait-based `ChannelAdapter` system to normalize incoming webhook payloads from these platforms into a single internal representation.

  **2. Core Data Entities:**
  - **Account/Tenant:** Multi-tenant row-level security (RLS) must be strictly enforced.
  - **Inbox:** Logical grouping of channels.
  - **Conversation & Messages:** The core entities connecting Contacts and Users/Agents.
  - **Contacts:** Omni-channel identity resolution (e.g., matching a WhatsApp phone number to a Web Widget email).
  - **Automation & Macros:** Canned responses, SLA policies, and automation rules that AI agents can utilize.

  **3. High-Performance Real-Time Engine:**
  Chatwoot relies on ActionCable (Ruby on Rails). OHC will leverage native Rust (e.g., Tokio, Axum, or similar) with WebSocket support and Redis Pub/Sub for scalable, low-latency message delivery.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      CHANNEL_ADAPTER ||--o{ CONVERSATION : spawns
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| AI_WORKER : processed_by
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Triage Feed:** The owner (e.g., Maya) opens the app and sees a combined feed of Instagram DMs, web inquiries, and WhatsApp messages in a simple list without technical CRM clutter.
  2. **Conversation View:** Tapping a message opens a clean, macOS-style translucent chat interface.
  3. **AI Integration:** The Customer & Relationship Assistant agent suggests a draft reply with a quote/deposit link directly in the chat view.
  4. **Actionable Outcomes:** A single button to "Send & Create Task" turns the conversation into a booked order.

  ### AI Agent Integration Points
  - **Customer Assistant:** Listens to the Redis stream of new `MESSAGE` events, analyzes intent, and pushes draft replies.
  - **Operations Assistant:** Parses messages for scheduling intents and coordinates with the Booking system to offer timeslots in the chat.

  ### Key Design Decisions
  - **Language:** Rust for the backend chat microservices to ensure memory safety and high concurrency.
  - **Database:** PostgreSQL with row-level security (`tenant_id`) across all tables.
  - **Real-time:** WebSockets driven by Rust and Redis for pub/sub across instances.

  ## Implementation Prompt
  **Goal:** Implement the foundational Native Rust Omnichannel Chat backend and Flutter mobile-first unified inbox UI.

  **CUJ (Critical User Journey):**
  Maya, the home baker, receives an Instagram DM asking about vegan cakes. The message flows through the Rust webhook receiver, gets stored in PostgreSQL with Maya's `tenant_id`, and appears in real-time in her Flutter app's Unified Triage Feed. The AI Assistant generates a draft reply. Maya taps "Approve & Send" on her 375px wide mobile screen, routing the message back out through the Rust channel adapter to Instagram.

  **Acceptance Criteria:**
  1. **Rust Microservice:** Implement a core Rust service with gRPC/REST APIs for `Inbox`, `Conversation`, and `Message` entities, backed by PostgreSQL.
  2. **Webhook Receiver:** Implement a generic webhook intake system that can route to a dummy/test channel adapter.
  3. **WebSocket Pub/Sub:** Implement a real-time event stream that pushes new messages to connected clients.
  4. **Flutter UI:** Build a mobile-first (375px) unified inbox screen with translucent glass styling that displays the real-time messages.
  5. **Automated Tests:** 100% unit test coverage in Rust, and Playwright E2E tests simulating an inbound message appearing in the UI. ZERO mock data in the UI; data must flow end-to-end.

  ## Priority & Scope
  - **Priority:** P0
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
