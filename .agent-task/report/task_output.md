issue_title: "[Architecture] Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC currently lacks a unified, native omnichannel messaging system that adheres to our strict multi-tenant architecture and mobile-first, AI-driven work assistant design. Relying on an external service like Chatwoot introduces latency, breaks zero-trust data isolation boundaries, and disrupts the seamless AI-assistant experience for our core personas (like Maya the baker and Carlos the handyman). Per our technical mandate, Chatwoot as a third-party dependency is fully retired. We need to implement a high-performance, native Rust omnichannel chat system within `onehumancorp/mono`.

  ## Research Report
  **Findings:**
  - **Market Context:** Integrated communication is a cornerstone of modern SMB platforms. Systems like Shopify Inbox and WeCom provide centralized command centers for merchants to manage customer interactions (DMs, SMS, Web Chat) directly alongside operations.
  - **Chatwoot Source Code Audit:** A review of the Chatwoot architecture (https://github.com/chatwoot/chatwoot) reveals essential models we must replicate: `Account` (Tenant), `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`. Their architecture heavily relies on ActionCable for WebSockets and background workers for agent routing and SLA enforcement.
  - **Native Rust Advantage:** Building this in Rust allows us to guarantee strict `tenant_id` row-level security (RLS) enforcement at the compiler level, minimize memory footprint for persistent WebSocket connections, and deeply integrate with our AI Job Queue (PostgreSQL `SKIP LOCKED` pattern).

  ## Design Doc
  **Architecture (High-Level):**
  - **Core Entities:**
    - `tenant_id` must strictly partition all records.
    - `Inbox`: Aggregates different channels (e.g., WhatsApp, Instagram DM, Web Chat).
    - `Conversation`: Represents a thread between a `Contact` and the business.
    - `Message`: Individual messages, supporting rich attachments (stored via GCS/MinIO) and AI drafts.
  - **Real-Time Delivery:** Utilize a Rust-based WebSocket server (e.g., Axum + Tokio-Tungstenite) authenticated via SPIFFE/SPIRE for bidirectional event streaming (`message.created`, `conversation.status_updated`).
  - **AI Agent Integration:**
    - New messages trigger a webhook to the AI Job Queue.
    - The **Customer & Relationship Assistant** contextually analyzes the message, retrieves business data (e.g., inventory, policies), and generates a drafted reply.
    - The draft is persisted as a pending `Message` and pushed via WebSocket to the owner's UI for approval.

  **Mobile UX Flow (375px First):**
  1. **Work Triage Dashboard:** The unified inbox is represented as a priority list on the home screen. Unread messages with AI-drafted replies are highlighted with a distinct premium translucent glass card.
  2. **Conversation View:** Tapping a card opens a chat UI. The keyboard uses native mobile behavior. The AI-drafted reply sits immediately above the text input with "Approve", "Edit", and "Discard" actions.
  3. **Offline Tolerance:** The UI must handle temporary network loss gracefully, queuing approved messages locally and syncing when back online.

  ## Implementation Prompt
  Implement the backend foundational layer for the Native Rust Omnichannel Chat System.
  **Target Persona & CUJ:** Maya (the home baker) receives a custom cake inquiry on Instagram. She opens OHC on her 375px iPhone, sees the inquiry in her Work Triage feed, taps it, and finds an AI-drafted reply ("Yes, we do vegan cakes! Here is a deposit link."). She taps "Approve" and the message is sent instantly.
  **Acceptance Criteria:**
  - Implement Rust data models and PostgreSQL schemas for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` Row Level Security.
  - Expose REST/gRPC endpoints for CRUD operations on these entities.
  - Implement a basic WebSocket endpoint in Rust that can accept connections and broadcast events.
  - Provide a test-only local adapter for external channels (mocking Instagram/WhatsApp).
  - **Test Requirements:** 100% unit test coverage for the new Rust module. At least one E2E Playwright test simulating the unified inbox flow using truthful backend state (ZERO mock data in UI).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
