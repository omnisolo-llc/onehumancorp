issue_title: "Implement Native Rust Omnichannel Inbox to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems or incomplete integrations for customer communication, previously utilizing Chatwoot which has since been removed from the architecture. Business owners (like Maya the Baker or Carlos the Handyman) need a unified, zero-configuration omnichannel inbox directly built into OHC. They shouldn't have to manage a separate SaaS tool, configure webhooks, or deal with multi-tenant data leakage risks to respond to Instagram DMs, WhatsApp messages, and website chats.

  ## Research Report
  - **Context:** `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` explicitly mandates the replacement of Chatwoot with a native, tenant-safe omnichannel support platform written in Rust.
  - **Codebase Audit:** Chatwoot has been fully removed. We cloned and analyzed the open-source Chatwoot repository (`https://github.com/chatwoot/chatwoot`) to benchmark feature parity. Key Chatwoot capabilities we must replicate natively:
    - `Inbox`, `Conversation`, `Message`, and `Contact` data models.
    - Channel Adapters (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Whatsapp`).
    - WebSocket real-time messaging for immediate UI updates.
    - Automation rules, SLA policies, and canned responses.
  - **Competitor Analysis:** Shopify Inbox and Zendesk provide integrated chat, but they are often complex to set up. Our native Rust implementation must be frictionless, instantly available for all tenants, and highly performant (low latency) by leveraging Rust's concurrency model and PostgreSQL RLS.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    A[Customer Widgets / Channels] -->|WebSockets / Webhooks| B(Rust Edge Gateway - Ingress)
    B --> C{Rust Omnichannel Router}
    C --> D[PostgreSQL - Conversations/Messages RLS]
    C --> E[Redis - Live Presence & PubSub]
    C --> F[AI Agents - Auto-reply]
    D --> G(Rust API - Egress)
    E --> G
    G -->|WebSockets / REST| H[OHC Owner Mobile/Web App]
  ```

  ### Mobile UX Flow (375px first)
  1. **Triage Feed:** The owner opens the app and sees a unified list of conversations (Instagram, Web, Email) sorted by priority/SLA.
  2. **Conversation View:** Tapping a thread opens a chat interface. It clearly shows the channel origin (e.g., small Instagram icon).
  3. **AI Assistance:** A floating "Draft Reply" button allows the AI to suggest a response based on tenant context.
  4. **Action-Oriented:** The owner can reply, assign, resolve, or convert the conversation into a Booking/Quote directly from the chat screen without horizontal scrolling.

  ### AI Agent Integration
  - **Customer Success Agent:** Listens to the `conversation.created` and `message.created` events via the internal event bus.
  - Generates auto-replies or drafts for human approval based on the tenant's knowledge base.
  - Respects bounded execution (loop prevention) and tenant capabilities.

  ### Key Design Decisions
  - **Language:** Rust for the backend messaging services to ensure high throughput, low latency, and memory safety for concurrent WebSocket connections.
  - **Data Isolation:** Strict PostgreSQL Row-Level Security (RLS) using `tenant_id` on all tables (`conversations`, `messages`, `contacts`).
  - **Event-Driven:** Utilize Redis Pub/Sub for realtime delivery to connected clients and to trigger background AI jobs.

  ## Implementation Prompt
  Implement the core backend data structures and API endpoints for the native OHC omnichannel inbox in Rust, replicating the essential features of Chatwoot.
  1. Define the core entity schemas (Contacts, Inboxes, Conversations, Messages) with strict multi-tenant RLS.
  2. Create the Rust gRPC/REST APIs for creating and fetching conversations and messages.
  3. Implement a WebSocket gateway in Rust for real-time message delivery to the frontend.
  4. Build a mobile-first (375px) Flutter UI for the unified inbox, allowing the owner to view threads and send replies.
  5. Ensure all tests (`bazel test //...` and Playwright E2E) pass and verify that a message sent via the API appears instantly in the UI via WebSockets.
  Do not prescribe specific database crate choices (e.g., Diesel vs SQLx); focus on the domain logic and tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
