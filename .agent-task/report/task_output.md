issue_title: "[Architectural Design] Native Rust Omnichannel Inbox & Chat Engine (Replacing Chatwoot)"
issue_description: |
  # Native Rust Omnichannel Inbox & Chat Engine

  ## Problem Statement
  Currently, OHC relies on Chatwoot as an external third-party service for handling omnichannel conversations, web chat widgets, and inbox management. This introduces a heavy external dependency, breaks our zero-trust multi-tenancy model (SPIFFE/SPIRE), limits our AI agents' ability to directly interface with streaming conversation events natively, and violates the platform's architectural mandate to keep core infrastructure native and high-performance. Maya (the baker) and Carlos (the handyman) need their customer communications (Instagram DMs, SMS, Web Chat) fully integrated into their OHC Assistant feed instantly, without relying on a disconnected external SaaS that fails to meet our latency and mobile-first offline targets.

  ## Research Report
  Based on an exhaustive audit of the Chatwoot source code (\`https://github.com/chatwoot/chatwoot\`), we identified the core architectural components required to build a native, high-performance omnichannel chat system in Rust:

  **Chatwoot Core Capabilities to Replicate:**
  1.  **Omnichannel Models:** `Accounts` (Tenants), `Inboxes` (Channel Containers), `Conversations` (Threads), `Messages` (Payloads), `Contacts` (Customers).
  2.  **Channel Adapters:** Interfaces for Web Widget, API, Email, Line, SMS (Twilio/Bandwidth), WhatsApp (Cloud API), Instagram/Messenger, etc. (Implemented via polymorphic associations in Rails; requires trait-based adapters in Rust).
  3.  **Real-time PubSub:** ActionCable/Redis based WebSocket broadcasting for instant message delivery and typing indicators.
  4.  **Agent Workflows:** Assignment, Round-robin routing, Macros, Canned Responses, SLA policies.
  5.  **Extensibility:** Webhooks for external bot integrations (which we will replace with native direct AI Agent hooks).

  **Competitive Context:**
  Modern unified inboxes (like Shopify Inbox, Front, and Intercom) leverage edge-caching and persistent WebSocket connections to deliver sub-50ms message latency. By bringing this natively into OHC (Rust + gRPC + Redis PubSub + PostgreSQL), we eliminate API boundary overhead between the "Chat system" and the "AI Assistant system," allowing our `Operations` and `Customer Service` agents to process inbound webhook events (like an IG DM) instantly, draft replies, and push them to the owner's mobile feed in real-time.

  ## Design Doc

  ### Architecture Overview

  We will implement a Native Rust Chat Engine within the `onehumancorp/mono` repository, structured around a high-performance asynchronous event loop.

  **Core Components (Rust):**
  1.  **`ohc-chat-gateway` (WebSocket Edge):** Manages persistent WebSocket connections with the Flutter mobile/web clients and the public web widget. Authenticated via SPIFFE/SPIRE JWTs.
  2.  **`ohc-chat-core` (gRPC Service):** The business logic layer. Handles REST API (for webhooks from Meta/Twilio) and gRPC for internal OHC services. Manages the database models.
  3.  **`ohc-chat-router` (Event Bus/Redis):** Uses Redis PubSub for cross-node message broadcasting and Kafka/NATS for durable event sourcing (e.g., triggering AI agents).

  **Data Model & Multi-Tenancy (PostgreSQL):**
  *   **Strict Row-Level Security (RLS)** using `tenant_id` on every table.
  *   `inboxes` (tenant_id, name, channel_type, settings)
  *   `contacts` (tenant_id, identifier, name, avatar_url, metadata)
  *   `conversations` (tenant_id, inbox_id, contact_id, status: open/snoozed/resolved, assignee_id)
  *   `messages` (tenant_id, conversation_id, content, message_type: incoming/outgoing/template/activity, sender_type, sender_id)
  *   `channel_configs` (tenant_id, inbox_id, provider_credentials_encrypted)

  ### Mobile UX Flow (375px First)
  1.  **The Unified Feed (Home):** Carlos opens the app. The home feed merges tasks and unread messages.
  2.  **Conversation View:** Tapping a message opens a native-feeling chat view. The UI uses Translucent Glass headers.
  3.  **AI Assistant Overlay:** Above the keyboard, a prominent "Assistant" area shows suggested replies (e.g., "Draft a quote for $150"). Carlos taps the suggestion to populate the input field, edits, and sends.
  4.  **Offline Tolerance:** Messages sent while Carlos is in a basement (offline) are queued locally (SQLite/Hive in Flutter) and sync automatically upon reconnection.

  ### AI Agent Integration Points
  *   **Inbound Hook:** When a new `Message` is created in PostgreSQL, a CDC (Change Data Capture) or application-level event is published to the `ai_job_queue`.
  *   **Agent Processing:** The `Customer Service Agent` wakes up, loads the conversation history + contact context, drafts a reply, and saves it as an `AgentDraft` linked to the conversation.
  *   **Owner Approval:** The UI subscribes via WebSocket to the `AgentDraft` creation and displays the suggestion to the owner.

  ### Mermaid Diagram
  ```mermaid
  graph TD
      Client(Flutter Mobile App/Web)
      Widget(Public Web Widget)
      Meta(Instagram / WhatsApp API)
      Twilio(SMS / Voice)

      WS[ohc-chat-gateway <br> WebSocket Server]
      Core[ohc-chat-core <br> gRPC / REST API]
      Redis[(Redis PubSub)]
      DB[(PostgreSQL + RLS)]
      AgentQueue[AI Job Queue]
      AI[OHC Customer Service Agent]

      Client <--> |WebSocket| WS
      Widget <--> |WebSocket| WS
      WS <--> |PubSub| Redis

      Meta --> |Webhooks| Core
      Twilio --> |Webhooks| Core
      Client --> |REST/gRPC| Core

      Core --> |Read/Write| DB
      Core --> |Publish Event| Redis
      Core --> |Enqueue Job| AgentQueue

      Redis <--> Core

      AgentQueue --> AI
      AI --> |Generate Draft/Reply| Core
  ```

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement Phase 1 of the Native Rust Omnichannel Chat System, establishing the foundational data models and the internal gRPC/REST API boundaries to replace Chatwoot.

  **Required Outcomes (Phase 1):**
  1.  Create the Rust domain structs and database schemas (with strict RLS `tenant_id` isolation) for `Inboxes`, `Contacts`, `Conversations`, and `Messages`.
  2.  Implement a generic trait-based `ChannelAdapter` system in Rust, and create a mock/dummy `TestChannelAdapter` for immediate unit testing.
  3.  Implement a REST API endpoint to receive incoming messages (simulating a webhook) and a gRPC endpoint to fetch a conversation's message history.
  4.  Ensure that creating a message successfully fires a localized event (or logs an event intended for the Redis PubSub layer).
  5.  **Verification:** Write 100% unit test coverage for the models and adapter logic. Create a Playwright E2E test that simulates an owner logging into the OHC UI, opening a specific Inbox, and viewing a newly injected message (using the API to simulate the inbound webhook). Do not use mocked UI state; the UI must read from the actual database via the new API.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
