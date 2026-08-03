issue_title: "Design Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OmniSolo/OHC currently relies on external third-party services like Chatwoot for omnichannel customer support and chat functionality. Relying on external dependencies introduces latency, security risks (multi-tenant data sharing), and breaks the unified AI-agent-driven work assistant experience. Non-technical owners (e.g., Maya the Baker, Carlos the Handyman) need an integrated customer communication channel directly within OHC that coordinates seamlessly with the rest of their operations.

  ## Research Report
  - **Chatwoot Source Code Audit:**
    - Analyzed Chatwoot's omnichannel data models (`https://github.com/chatwoot/chatwoot`), which include `Conversations`, `Messages`, `Inboxes`, `Contacts`, `ChannelAdapters` (Web, API, Email, SMS, WhatsApp), and `Users` (Agents/Admins).
    - Chatwoot handles real-time sync through ActionCable (WebSocket).
    - It uses SLA policies, auto-assignment algorithms, macros, and canned responses to speed up agent workflows.
  - **OHC Competitive Analysis:**
    - Competitors like Shopify Inbox, WeCom, and DingTalk provide native, deeply integrated chat systems that share context with orders, products, and customer records.
  - **Proposed Integration:**
    - OHC will build its own high-performance, native Rust omnichannel chat system to fully retire Chatwoot.
    - Our system will handle multi-tenant isolation rigorously via tenant IDs and Row-Level Security (RLS) equivalents at the database level.
    - WebSockets will be handled via Actix-Web or Axum in Rust, ensuring low-latency communication.

  ## Design Doc
  - **Architecture Diagram (Conceptual):**
    - Client (Web/Mobile) <--> Rust WebSocket Server <--> Message Processor <--> PostgreSQL (Conversations, Messages)
    - Integrations (Email, SMS, WhatsApp, IG) <--> Rust Webhook Handlers <--> Inbox Router <--> PostgreSQL
    - AI Agent Hooks <--> Events Queue (Redis/Kafka) <--> Background Workers (Rust)
  - **Data Model (Core Entities):**
    - `Tenant` (Isolated workspace)
    - `Contact` (Customer entity, cross-channel)
    - `Inbox` (Channel group)
    - `ChannelAdapter` (Specific integrations like Web Widget, WhatsApp)
    - `Conversation` (Thread of messages)
    - `Message` (Individual message, attachment, agent note)
  - **Multi-Tenant & Security Strategy:**
    - Strict row-level isolation for every query using `tenant_id`.
    - Zero-trust SPIFFE/SPIRE identity for service-to-service calls.
  - **Mobile UX Flow (375px First):**
    - Tab bar icon for "Inbox".
    - Conversations list with unread indicators, tags, and AI summaries.
    - Chat screen with message bubbles, input area, attachment button, and inline AI suggested replies.
    - Premium translucent glass styling and UniFi-style card layouts.

  ## Implementation Prompt
  - **User Journey (CUJ):**
    - As a business owner (e.g., Maya the Baker), I want to receive Instagram DMs and Web Chat inquiries in a single native OHC inbox, so I don't have to switch apps.
    - When I open the Inbox on my 375px mobile screen, I see a unified list of messages.
    - I can read a message, see AI-suggested replies based on the customer's history and my bakery context, and tap to send or edit the reply.
    - The message is sent back via the native channel adapter (e.g., IG or Web Widget) instantly.
  - **Acceptance Criteria:**
    - Implement the native Rust omnichannel core data models (Contact, Inbox, Conversation, Message) with strict multi-tenant isolation.
    - Implement a WebSocket connection manager for real-time client updates.
    - Develop the API layer (REST/gRPC) for message sending, receiving, and conversation management.
    - Ensure 100% unit test coverage for the new Rust module.
    - Create at least 5 Playwright E2E tests verifying the UI integration of the new inbox from a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
