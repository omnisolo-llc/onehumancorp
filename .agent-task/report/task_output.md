issue_title: "Build Native Rust Web Widget Chat Integration for OHC"
issue_description: |
  ## Problem Statement
  Currently, small-business operators (like our personas Maya, Carlos, Priya, and Nora) need a seamless, real-time way to interact with their customers directly from their websites without forcing them into a WhatsApp or email flow immediately. A live web widget allows them to answer questions, capture leads, and provide customer support, all directly funneling into their OHC dashboard for triage by the AI assistant or human operators. The legacy external third-party Chatwoot integration is 100% retired, and OHC requires a native, high-performance, and multi-tenant solution built in Rust to serve this critical business need. Without it, owners lose visibility into live website traffic and risk missing direct sales opportunities.

  ## Research Report
  Based on the source code benchmarking of the legacy Chatwoot repository (e.g., `app/models/channel/web_widget.rb`), the core functionality required for a web widget includes:
  *   **Real-time Communication:** WebSocket-based event streams for real-time messaging between the website visitor and the OHC backend.
  *   **Session Persistence:** Ability to tie a chat session to a unique visitor even if they navigate across different pages on the owner's website.
  *   **Customization:** The widget needs a `provider_config` or similar structure to store widget color, welcome message, and positioning to match the owner's brand.
  *   **Unified Inbox Integration:** Messages generated from the widget must flow into the same unified `messages` table and UI as other channels (like WhatsApp), ensuring a single source of truth for the owner.
  *   **Performance & Scale:** By moving this to a native Rust implementation, we avoid the overhead and operational complexity of a separate Ruby on Rails application, reducing latency and infrastructure costs while integrating directly with OHC's multi-tenant architecture and AI agent triage system.
  *   **Market Need:** Tools like Intercom, Zendesk, and Chatwoot offer this, but they are separate silos. OHC's unique value proposition is having this integrated natively, where the AI assistant can immediately draft replies, create tasks, or schedule bookings based on the chat.

  ## Design Doc
  **Integration Strategy:**
  1.  **Data Model:** Define the database schema for the web widget channel. This will likely involve a `channel_web_widget` table linked to a generic `channels` table, capturing settings like widget color, welcome tagline, and domain whitelisting. It must enforce `tenant_id` for row-level security.
  2.  **API Endpoints:** Create a set of public-facing API endpoints (REST) for the initial widget initialization (fetching settings) and starting a conversation session.
  3.  **WebSocket Gateway:** Implement a WebSocket handler (using Axum's WebSocket support) to stream real-time events (new message, typing indicator, presence) between the frontend widget script and the OHC backend.
  4.  **Message Routing:** Connect the incoming WebSocket messages to the internal OHC message bus. This should trigger the `Work Triage` AI capability to evaluate the message, draft a reply, or notify the owner if human intervention is required.
  5.  **Widget Script:** (Future scope, but needs backend support) A lightweight JavaScript bundle (`widget.js`) that owners can embed on their sites, which communicates with these new APIs.

  **User Experience (Owner's Perspective):**
  *   The owner navigates to their OHC settings, clicks "Add Web Chat", and receives a small JavaScript snippet to paste onto their website.
  *   They can customize the widget's appearance (color, welcome message) directly within OHC.
  *   When a customer chats on their website, the message immediately appears in the OHC unified inbox, alongside Instagram DMs and emails.
  *   The AI assistant drafts a contextual reply based on the owner's knowledge base and past interactions.

  ## Implementation Prompt
  Implement the backend infrastructure for the native Rust Web Widget chat channel.
  *   Define the data models and database migrations required for a web widget channel, ensuring strict multi-tenant isolation.
  *   Create the Rust backend services and Axum routes for widget initialization and WebSocket connections.
  *   Ensure the implementation allows for storing widget customization settings (color, welcome text).
  *   Connect the incoming messages to the core OHC message handling system so they appear in the unified inbox.
  *   Write comprehensive unit and integration tests verifying the API behavior, WebSocket connectivity, and multi-tenant data isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
