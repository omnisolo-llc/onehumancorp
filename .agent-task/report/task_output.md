issue_title: "Native Rust Omnichannel Chat Inbox System Implementation"
issue_description: |
  # Native Omnichannel Chat System Research Report & Design Doc

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a fully native omnichannel customer support system built in Rust. Currently, the repository relies on a deprecated `server_integrations_chatwoot` outbound client and legacy Chatwoot integration patterns, but it lacks the core Chatwoot-equivalent functionality needed by our personas (e.g. Maya managing DMs, Carlos receiving SMS inquiries). The native infrastructure for multi-channel messaging is partially present but lacks a unified operational structure, comprehensive delivery tracking, resilient state management, and an admin interface that mirrors Chatwoot's power while being simple enough for an SMB owner to use on a mobile device.

  ## Research Report
  Based on our analysis of the Chatwoot open-source repository (`app/models/*`, omni-channel routing, inbox structures) and OHC's internal architecture docs (e.g., `2026-07-13-native-omnichannel-chat-design.md`):
  - **Chatwoot Disassembly:** Chatwoot relies on core models such as `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`. Its architecture routes incoming messages through channel-specific controllers, creates or updates conversations, and broadcasts real-time events via WebSockets to operator inboxes.
  - **OHC Gaps:** OHC currently has fragmented persistence models (`inbox_messages`, `omni_inbox_messages`), incomplete AI triage integrations, and insufficient multi-tenant data boundaries compared to Chatwoot's robust `Account` (Tenant) scoping.
  - **Competitor Systems:** Stripe and Shopify have strong unified inbox models where a message from SMS, email, or chat widget resolves to a single user profile. We need to implement a similar unified contact resolution in Rust.

  ## Design Doc

  ### Architecture Diagram (Mental Model & Entity Rel)
  - **Tenant (`tenant_id`)** -> The absolute boundary.
  - **Contact** -> A customer profile unified across channels (e.g., Phone, Email, Meta ID).
  - **Inbox** -> A logical grouping (e.g., "Sales", "Support", "General").
  - **Conversation** -> A thread between a Contact and the business, tied to an Inbox.
  - **Message** -> Individual entries in a Conversation, typed by channel.
  - **ChannelAdapter** -> Rust traits for Meta, Twilio, Email, and generic Webhooks, processing ingestion and outbound delivery.

  ### Mobile UX Flow (375px First)
  - **Inbox View:** A unified list of active conversations with clear badges for unread messages, AI drafts, and source channel icons (WhatsApp, SMS, Email).
  - **Thread View:** A chat interface where the owner can see the customer's history, current message, and an AI-suggested draft response.
  - **Action Bar:** Floating controls to "Send", "Edit Draft", or "Resolve" the conversation.
  - **Real-time:** The UI must update instantly via WebSocket/SSE without manual refresh.

  ### AI Agent Integration Points
  - **Work Triage:** Analyzes incoming messages to determine urgency and route to the correct Inbox or agent.
  - **Customer & Relationship Assistant:** Drafts suggested responses based on the business's knowledge base and previous interactions with the contact.

  ### Key Design Decisions
  - **Zero Chatwoot:** We will completely remove all references to Chatwoot integration.
  - **Rust Native:** The core message processing, channel adapters, and real-time broadcasting will be implemented in the `onehumancorp/mono` Rust backend.
  - **Unified Persistence:** We will migrate away from the competing inbox schemas to a single, robust, tenant-isolated `messages` and `conversations` schema.

  ## Implementation Prompt
  **Goal:** Implement the core Rust backend services and PostgreSQL database schema for the native omnichannel chat system, replacing Chatwoot.
  **CUJ:** A small business owner (Maya) receives a new Instagram DM. The system ingests it, creates a unified conversation, the AI drafts a reply, and Maya reviews/sends it from her 375px mobile screen.
  **Acceptance Criteria:**
  1. Define and implement the Rust data structures for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` isolation.
  2. Implement a `ChannelAdapter` trait and at least one concrete adapter (e.g., generic webhook or mock SMS) for ingestion.
  3. Ensure the backend can ingest a message and store it correctly in the unified schema.
  4. Create the necessary REST/gRPC endpoints for the frontend to fetch conversations and send replies.
  5. Remove legacy Chatwoot integration code where it conflicts.
  6. Achieve 100% test coverage for the new Rust modules.
  7. Write Playwright E2E tests verifying the inbox ingestion and response flow via the web UI.

  **Priority:** P0 (Critical)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
