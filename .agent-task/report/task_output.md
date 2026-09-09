issue_title: "Architectural Gap: Native Rust Omnichannel Customer Support & Chat Engine"
issue_description: |
  **Problem Statement:**
  Currently, OneHumanCorp (OHC) lacks a unified, high-performance omnichannel inbox capable of handling real-time customer communications (Instagram DMs, WhatsApp, Web Chat, Email, SMS) seamlessly. Our personas, such as Maya the home baker and Carlos the handyman, are bogged down trying to switch between multiple apps to triage custom orders or service requests. They need a single, real-time command center where an AI assistant unified with a "Native Rust Omnichannel Customer Support & Chat Engine" can draft replies, manage deposits, and coordinate bookings without leaving the OHC platform.

  **Research Report:**
  - **Shopify Ping / Inbox:** Centralizes conversations but is tightly coupled with Shopify products.
  - **Tencent Workbuddy / WeCom:** Provides excellent omnichannel capability but at an enterprise scale with high technical overhead.
  - **Zendesk / Intercom:** Too heavy, enterprise-focused, and non-intuitive for a 375px mobile-first solo operator.
  - **Modern AI-Native Tools:** High-performance systems rely on Rust-based WebSocket handling and native actor models for concurrency.
  - **Opportunity:** A natively built Rust shared communication layer within OHC using a Universal Provider Facade for AI drafts, unified with OHC's multi-tenant SaaS architecture. This ensures high throughput, Zero-Trust isolation, and absolute mobile parity.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ Conversation : contains
      Conversation ||--o{ Message : includes
      ChannelAdapter ||--o{ Conversation : routes
      Customer ||--o{ Conversation : participates
      Tenant {
        uuid tenant_id
      }
      Inbox {
        uuid inbox_id
      }
      Conversation {
        uuid conversation_id
        string status
      }
      Message {
        uuid message_id
        text content
      }
    ```
    ```mermaid
    sequenceDiagram
      participant C as Customer (IG/WhatsApp)
      participant A as Channel Adapter (Rust)
      participant E as Real-Time Engine (Rust/WS)
      participant I as Inbox (Mobile 375px)
      participant AI as Universal Provider Facade
      C->>A: Inbound Message
      A->>E: Route to Tenant Inbox
      E->>I: WS Push (Real-Time)
      E->>AI: Trigger Auto-Draft
      AI-->>I: Propose Draft Reply
      I->>E: Owner Approves & Sends
      E->>A: Dispatch
      A->>C: Outbound Message
    ```
  - **Mobile UX Flow (375px first):**
    1. **Home Command Center:** Unified "Inbox" notification bubble highlights urgent pending messages across all channels.
    2. **Triage View:** A consolidated list showing conversations. Each row features a channel icon (e.g., IG, Email), customer name, snippet, and AI-suggested "Next Action" (e.g., "Draft ready", "Needs deposit").
    3. **Conversation View:** iMessage-like chat interface. Bottom action bar has a prominent "AI Draft" button, alongside quick actions like "Send Quote" or "Request Payment".
    4. **Resolution:** Owner taps "Approve" on the AI draft, message sends, and the conversation is auto-archived.
  - **AI Agent Integration Points:**
    - AI "Customer & Relationship Assistant" listens to the message queue via PostgreSQL `SKIP LOCKED`.
    - Auto-generates drafts using tenant-scoped memory (e.g., previous orders, preferences).
    - Can inject "Operations" tools (e.g., fetching a booking link) into the conversation context.
  - **Key Design Decisions:**
    - Natively built in Rust (`omnisolo-llc/onehumancorp`) using WebSocket/SSE for real-time messaging to minimize latency (< 250ms UI motion).
    - Multi-tenant isolation at the DB level (`ENABLE ROW LEVEL SECURITY`) and memory boundary.
    - AI drafts are proposed, not auto-sent (unless explicitly configured by an advanced user), maintaining Owner Control.

  **Implementation Prompt:**
  **Context:** We need to implement the Native Rust Omnichannel Customer Support & Chat Engine for OneHumanCorp.
  **Goal:** Build the backend Rust services, data models, and the Flutter PWA mobile-first (375px) inbox UI that unifies IG DMs, WhatsApp, and Web Chat into a single triage feed.
  **CUJ:** Maya receives an IG DM asking for a vegan cake quote. The message hits the Rust channel adapter, flows to her OHC Mobile Inbox via WebSockets, and the AI pre-drafts a reply with a quote link. Maya taps "Approve" to send the reply back to IG.
  **Acceptance Criteria:**
  1. Rust backend handles inbound webhooks and routes to tenant inboxes.
  2. WebSocket/SSE real-time push to the UI.
  3. UI perfectly usable on 375px screens with touch targets >= 44x44px.
  4. AI Assistant auto-draft generation integrated.
  5. Multi-tenant data isolation strictly enforced.
  6. E2E tests written using browser/Playwright covering the full inbox triage CUJ.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
