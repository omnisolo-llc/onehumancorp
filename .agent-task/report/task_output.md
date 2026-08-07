issue_title: "Native Rust Omnichannel Web Chat Widget Engine (Chatwoot Alternative)"
issue_description: |
  ## Title: Native Rust Omnichannel Web Chat Widget & Inbox Engine

  ## Problem Statement
  Owners like Maya (Home Baker) and Priya (Boutique Operator) need a unified way to communicate with their customers across different channels (Website, WhatsApp, Instagram). Relying on external third-party services like Chatwoot introduces latency, privacy concerns, and additional subscription costs. We need a native, fast, multi-tenant omnichannel chat engine directly inside OHC, so owners can immediately reply to website visitors, capture leads, and triage messages from a single inbox without leaving the OHC platform.

  ## Research Report
  **Competitive Analysis & Benchmarking:**
  - **Chatwoot**: Chatwoot's architecture relies on Rails and PostgreSQL with specific models for different channels (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::FacebookPage`). Messages are normalized into a unified `Message` and `Conversation` table, allowing operators to reply from a single inbox regardless of the source. It also provides a robust web widget (Vue.js/React) that authenticates via a `widget_token` and establishes WebSocket connections for real-time updates.
  - **Shopify Inbox & Wix Inbox**: Both provide native chat solutions that are tightly integrated with the store's inventory and customer profiles. This gives operators instant context on the customer's cart or previous orders.
  - **Why Native in OHC?**: By implementing the web widget and omnichannel inbox natively in Rust within OHC, we eliminate the need for a separate third-party Chatwoot integration. This aligns with OHC's architecture (multi-tenant, row-level security in PostgreSQL). The Rust engine will provide high-performance WebSocket handling for real-time messaging, low memory footprint, and tight integration with OHC's AI agents (e.g., automatic draft replies).

  **Key findings from Chatwoot Source Code (`https://github.com/chatwoot/chatwoot`):**
  - Channels are defined polymorphically. `Inbox` belongs to a channel (e.g., `Channel::WebWidget`, `Channel::Whatsapp`).
  - Messages have a unified schema: `account_id`, `conversation_id`, `inbox_id`, `content`, `message_type` (incoming/outgoing), and `content_type` (text/image).
  - Contacts are unique per account and can have multiple `ContactInbox` records linking them to different channel identities (e.g., a phone number for WhatsApp, a browser cookie/token for Web Widget).

  ## Design Doc
  **Integration with OHC:**
  - **Data Model**: Introduce multi-tenant `inboxes`, `conversations`, `messages`, and `contacts` tables in PostgreSQL, guarded by our row-level security (`tenant_id`). Implement polymorphic channel configurations (e.g., `web_widget_config`, `whatsapp_config`).
  - **Web Widget**: Build a lightweight, embeddable web component (JS/CSS) that owners can drop into their external sites. The widget authenticates using a tenant-specific public token and establishes a WebSocket connection to the OHC Rust backend.
  - **Unified Inbox UI**: Inside the OHC Flutter/PWA app, introduce a "Work Triage" Inbox view. This view aggregates `conversations` across all active channels.
  - **Real-Time Engine**: The Rust backend will manage WebSocket connections from both the Web Widget (customers) and the OHC App (owners). When a customer sends a message, the Rust backend persists it and broadcasts it to the owner's active sessions via Redis Pub/Sub (for cross-node scaling) and WebSockets.
  - **AI Coordination**: When a message arrives, the OHC AI Job Queue will evaluate it to generate a suggested reply or categorize the intent (e.g., order inquiry, support) before the owner even reads it.

  ## Implementation Prompt
  **User-Facing Outcome:**
  As an owner (e.g., Priya), I can navigate to my OHC settings and generate a snippet of code for a Web Chat Widget to place on my boutique's website. When a customer visits my site and types a message, it instantly appears in my OHC Work Triage feed. I can read their message, see an AI-drafted suggested reply, and respond. The customer receives my reply in real-time on the website.

  **Acceptance Criteria:**
  - An owner can create a "Web Widget" inbox and get an embeddable HTML/JS snippet.
  - A customer can open the widget on an external site, start a conversation, and send text messages.
  - Messages are stored securely with tenant isolation.
  - The owner receives the message in their unified OHC Inbox UI in real-time via WebSockets.
  - The owner can reply from OHC, and the customer sees the response in the widget instantly.
  - The feature operates entirely natively within OHC (no external Chatwoot dependencies).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
