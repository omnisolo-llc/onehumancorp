issue_title: "Architecture & Implementation: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC previously relied on Chatwoot as an external service for omnichannel inbox management. Chatwoot is now 100% retired. OHC requires a native, high-performance, multi-tenant Rust chat system to manage WhatsApp Business and Web Widget interactions. Non-technical owners (like Maya the baker or Carlos the handyman) need a unified, real-time inbox that seamlessly coordinates with AI agents to draft responses and triage incoming requests without them navigating technical menus.

  ## Research Report
  Based on our codebase audit (`src/server/services/chat` and `chatwoot/app/models`):
  - Chatwoot utilizes complex conversational models (Inboxes, Conversations, Messages, Contacts, and Channels) built on Rails and PostgreSQL.
  - We have initiated basic Rust models (`ChatInbox`, `ChatConversation`, `ChatMessage`, `ChatChannel`, `ChatContact`) in `src/server/services/chat/models.rs`.
  - Migrations (`1009_native_omnichannel_chat.sql`) enforce Row Level Security (RLS) via `tenant_id`.
  - Competitors like Shopify Inbox, WeCom, and Wix Inbox emphasize lightweight mobile accessibility and AI drafting. OHC differentiates by instantly triaging messages using AI triage agents (using `SemanticRouter`) and automating drafts for review.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ ChatInbox : owns
      ChatInbox ||--o{ ChatChannel : has
      ChatInbox ||--o{ ChatConversation : contains
      ChatContact ||--o{ ChatConversation : participates
      ChatConversation ||--o{ ChatMessage : holds
  ```
  **System Components**:
  1. **Core Data Models (Rust/PostgreSQL)**: Extend `ChatService` to support webhook events for WhatsApp and WebSocket connections for Web Widget.
  2. **WebSocket & Webhooks**: Actix/Axum-based WebSocket handlers to broadcast real-time updates to connected mobile/web clients.
  3. **AI Agent Integration**: Wire incoming messages to `SemanticRouter` (currently seen in `src/server/api/agents/chat.rs`) to classify intent (Operations, Customer Success) and automatically invoke `DepartmentOrchestrator` to generate a `DraftForReview` action.
  4. **Multi-Tenancy**: All DB interactions must continue to bind `tenant_id` properly to maintain strict RLS isolation.

  ### Mobile UX Flow (375px)
  - **Screen 1 (Unified Inbox)**: Bottom nav tab. Shows a list of recent conversations (WhatsApp + Web) with AI-generated summary badges.
  - **Screen 2 (Conversation View)**: Standard chat interface. Messages from customers appear on the left. The bottom input area includes a "Suggested Reply" button (glassmorphism style) pulsing if the AI has drafted a response.
  - **Interaction**: Maya receives a custom cake inquiry on WhatsApp. The AI triage agent drafts a quote response. Maya taps the pulsing "Review Draft" button, edits the price if needed, and taps "Send".

  ### AI Agent Integration Points
  - **Triage**: Every incoming message enqueues a lightweight background job (via the existing Pg/Redis queue) for the Semantic Gateway.
  - **Drafting**: If intent requires a quote (e.g., Sales), the `SalesAgent` drafts a response and attaches it as a system message with `status='pending_approval'`.

  ## Implementation Prompt
  **To the Implementer**:
  1. Complete the Rust `ChatService` in `src/server/services/chat/service.rs` to include fetching conversations and sending messages via WebSocket/WhatsApp adapters.
  2. Implement the frontend unified inbox UI in Flutter/Next.js adhering to the OHC Premium Token library (macOS Translucent Glass, UniFi layout). Ensure perfect functionality on 375px screens.
  3. Integrate the `SemanticRouter` to process incoming messages and surface AI-drafted replies (like the "Needs Approval" action cards seen in `Team Chat`) directly in the conversation view.
  4. Ensure complete coverage using Playwright E2E tests for the new Inbox CUJ (no mocked data; use real DB seeds and endpoints).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
