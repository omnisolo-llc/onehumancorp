issue_title: "Native Rust Chat Engine Architecture"
issue_description: |
  # Native Rust Chat Engine Architecture

  ## Problem Statement
  Small business owners currently face a fragmented communication landscape, interacting with customers across Instagram DMs, WhatsApp, Email, and SMS. The requirement states that "Chat woot as an external third-party service, dependency, or integration is 100% RETIRED." Our application must implement its own high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust to achieve 100% feature parity with that retired service, including omnichannel data models, controllers, channels, WebSocket real-time messaging, and inbox architecture. This provides context-aware AI drafting capability ("The Ambassador") within an autonomous workspace.

  ## Research Report
  ### Findings & Competitive Analysis
  - **Prior Architecture Assessment:** Based on a source code audit of `https://github.com/chat` `woot/chat` `woot`, the legacy system utilizes concepts of `Account` (tenant), `Inbox`, `Channel` (adapter), `Conversation`, `Message`, and `Contact`. It heavily leverages ActionCable for real-time WebSockets and relies on PostgreSQL for persistence with Redis for background jobs and Pub/Sub.
  - **Current OHC State:** OHC has some rudimentary omnichannel tables (e.g. `omni_inbox_messages`, `work_item`) but lacks a cohesive native Rust chat engine, scalable WebSocket messaging system, and the robust Channel abstraction found in the prior architecture.
  - **The OHC Opportunity:** By replacing the legacy service with a native Rust implementation embedded within the OHC platform, we can integrate it directly with our Agentic systems. "The Ambassador" agent can be triggered directly by Rust channel webhooks, query the local context, and proactively draft replies seamlessly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: Insta/WA/SMS] -->|Webhook| B(Omnichannel Gateway - Rust)
      B --> C{Channel Adapter Registry}
      C --> D[Unified Conversation DB - Postgres]
      D --> E[Event Mesh / Redis PubSub]
      E --> F[WebSocket Hub - Rust/Tokio]
      F -->|Real-time Updates| G[OHC Mobile App 375px]
      E --> H[The Ambassador Agent]
      H -->|Query| I[Customer Identity Graph]
      H -->|Drafts Reply| D
      G -->|Approves| B
      B -->|Sends| A
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Feed:** Users land on an aggregated, prioritized feed of incoming conversations, irrespective of source. Each card prominently displays the customer's intent and platform icon.
  - **Proactive Draft Cards:** AI drafts ("The Ambassador") are styled distinctively, requiring a single tap ("Approve") or swipe action.
  - **Zero-Friction Context:** Tapping a conversation expands a pane showing past interactions and order history pulled from the identity graph.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by `ConversationCreated` or `MessageCreated` events published by the Omnichannel Gateway to the Event Mesh. It drafts a reply directly to the `Message` table with a status `draft`.

  ### Key Design Decisions
  - **Native Rust Axum/Tokio WebSocket Server:** Replace Ruby ActionCable with a high-performance Rust WebSocket server.
  - **Channel Adapter Pattern:** Implement traits in Rust for each channel (WhatsApp, Instagram, Email) to normalize incoming payloads into the unified `Message` and `Conversation` models.
  - **Strict Multi-Tenancy:** All new models (`Conversation`, `Message`, `Inbox`, `Channel`) must include `tenant_id` and enforce RLS policies strictly aligned with OHC standards.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya, a baker, receives an Instagram DM. Her OHC mobile app instantly shows a notification in her Unified Inbox. Opening it reveals the customer's message alongside a perfectly drafted AI response considering past orders. Maya taps "Approve" and the message is dispatched back to Instagram. All of this operates on OHC's internal native infrastructure without relying on external chat services.

  **CUJ & Acceptance Criteria:**
  1. Define and migrate the core omnichannel PostgreSQL schema in Rust/SQLx: `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Ensure RLS is active.
  2. Implement the `ChannelAdapter` trait and a concrete webhook handler for at least one channel (e.g., a dummy/test webhook).
  3. Implement the native Rust WebSocket server (e.g., using `axum::extract::ws`) that broadcasts `MessageCreated` events to connected clients authenticated via tenant.
  4. Integrate "The Ambassador" agent to automatically listen to new messages and generate a draft reply.
  5. Playwright E2E Test: Simulate an incoming webhook message, verify the WebSocket updates the UI, verify the AI draft appears, tap "Approve", and verify the dispatch logic is triggered.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
