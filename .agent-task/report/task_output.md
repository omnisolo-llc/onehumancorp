issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**
  OHC currently relies on external Chatwoot for customer support and chat functionality. However, Chatwoot as an external service is being retired because it introduces multi-tenancy risks, third-party dependency complexity, and is not perfectly tailored to the OHC owner/operator workflows (e.g., Maya, Carlos, Fatima). The platform requires a fully integrated, high-performance, native Rust omnichannel chat system inside `onehumancorp/mono` that aligns with OHC's Zero Trust architecture and Mobile-First constraints, enabling owners to manage all customer communications seamlessly from a unified triage feed.

  **Research Report**
  After reviewing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and leading omnichannel architectures (e.g., Shopify Inbox, Stripe, Wix), the essential components needed in OHC are:
  - Scalable WebSockets for real-time messaging.
  - Multi-tenant conversation and inbox data models.
  - Channel adapters for web widget, Instagram, WhatsApp, and Email.
  - AI Agent handoff and integration points.
  The native implementation must enforce strict tenant isolation using SPIFFE/SPIRE identity and PostgreSQL row-level security (`ENABLE ROW LEVEL SECURITY`). This approach guarantees that data for different owners is securely segmented while providing the high-speed concurrent processing capabilities native to Rust.

  **Design Doc**
  - **Architecture Diagram (Mermaid.js)**
    ```mermaid
    graph TD;
      Client(Mobile/Web Widget) -->|WebSocket/REST| RustAPI(Rust Chat API)
      RustAPI -->|Row Level Security| PG(PostgreSQL: Inbox, Conversations, Messages)
      RustAPI -->|Pub/Sub| Redis(Redis for Real-time events)
      RustAPI --> AIAgent(AI Customer Assistant)
      AIAgent -->|Auto-reply/Draft| RustAPI
    ```

  - **Mobile UX Flow (375px first)**
    The owner opens OHC on their phone and immediately sees a consolidated "Work Triage" feed. Tapping on an unread message transitions to a native-feeling chat view without any horizontal scrolling (minimum 44x44px touch targets). The AI-drafted reply is visibly distinct, adopting translucent glass styling (macOS-style) with a "Send Draft" button. Advanced settings and technical jargon are completely hidden.

  - **AI Agent Integration Points**
    The Rust Chat API emits `ConversationUpdated` events to the Redis event bus. The "Customer & Relationship Assistant" listens to these events, accesses tenant-scoped memory to understand the customer context (e.g., past orders, preferences), and pushes draft replies back via gRPC/REST endpoints. These drafts are then streamed to the client via WebSockets, awaiting the owner's one-tap approval.

  **Implementation Prompt**
  **Goal:** Build the foundational native Rust omnichannel chat models, WebSocket server, and API endpoints to replace Chatwoot, integrated seamlessly with the Flutter frontend.
  **CUJ:** Maya receives a new Instagram DM asking about a vegan cake. It appears instantly in her OHC Work Triage feed via WebSockets. She taps it and sees an AI-drafted reply based on her previous vegan cake orders, which she approves and sends with a single tap.
  **Acceptance Criteria:**
  - Create native Rust entities: `Inbox`, `Conversation`, `Message`, and `ChannelAdapter`.
  - Implement a secure WebSocket server in Rust for real-time message streaming.
  - Ensure 100% row-level multi-tenant isolation (`tenant_id` on every table).
  - Develop a Flutter frontend chat view responsive at 375px using OHC design tokens.
  - Entirely remove external Chatwoot dependencies.
  - Add at least five E2E Playwright tests simulating inbound messages and owner responses.
  - Achieve 100% Unit Test coverage for all new Rust and Flutter code.

  **Scope:** Large
  **Priority:** P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
