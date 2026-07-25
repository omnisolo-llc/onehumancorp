issue_title: "Implement Native Rust Omnichannel Chat System (Retire Chatwoot)"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System (Chatwoot Replacement)

  **Problem Statement**:
  Currently, Maya the baker and Carlos the handyman struggle to manage customer communications across different channels (Instagram DMs, WhatsApp, Email, Web Chat). The existing external tool (Chatwoot) feels disjointed from their core business workflows (bookings, quotes, inventory). They need a single, unified "Inbox" directly inside the OneHumanCorp (OHC) app where they can reply to a customer, see their past orders, generate a new quote, or have the AI draft a response—all without leaving the app or setting up a third-party integration. We need a native solution built into OHC that handles all these channels seamlessly.

  **Research Report**:
  - **Chatwoot Source Analysis**: Reviewing the `chatwoot/chatwoot` repository, we observed a comprehensive structure utilizing `Account`, `Conversation`, `Message`, `Contact`, `Inbox`, and `Channel` adapters. Its strengths lie in omnichannel ingestion and WebSocket-based real-time delivery. However, its architecture is decoupled from native commerce primitives (inventory, services, quotes).
  - **Competitor Analysis**:
    - **Shopify Inbox**: Extremely successful because it natively connects chat with storefront data. A merchant can share a product link or create a discount code directly in the chat.
    - **Wix Inbox**: Consolidates site chat, Facebook, and form submissions, deeply integrated with Wix CRM.
    - **WeCom/Tencent Workbuddy**: The gold standard for integrated operations; chat is just one module in a unified work interface.
  - **Gap**: OHC relies on Chatwoot which introduces latency, disjointed multi-tenant boundaries, and poor cohesion with OHC's native AI assistants (e.g., generating an invoice based on chat context). A native Rust implementation will provide memory-safe, high-concurrency message handling with direct access to OHC's tenant-scoped database and Agent queues.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
        Tenant ||--o{ Inbox : configures
        Inbox ||--o{ ChannelAdapter : connects
        Inbox ||--o{ Conversation : contains
        Conversation ||--o{ Message : has
        Conversation }o--|| Contact : belongs_to
        Message ||--o{ Attachment : includes
    ```
  - **Key Design Decisions**:
    - **Native Rust Services**: High-performance, concurrent ingestion of webhooks (WhatsApp, Instagram) using Rust async channels.
    - **Unified Data Model**: Direct integration into OHC's PostgreSQL database with row-level security for tenant isolation.
    - **AI Integration**: The `Customer & Relationship Assistant` will subscribe to the message event stream to automatically draft replies or suggest actions based on the conversation context.
  - **UI Wireframes & Screen Flow (375px first)**:
    - **Inbox List (Home)**: Clean, Unifi-style list of recent conversations, sorted by urgency/AI priority. Translucent glass app bar.
    - **Conversation View**:
      - Sticky header with customer name and key business context (e.g., "Maya's Bakery - Last order: 2 days ago").
      - Scrollable message thread.
      - Input area with native mobile keyboard support and an "AI Draft" button prominently displayed.
    - **Mobile UX Flow**: Tap Inbox -> Tap Conversation -> Review AI draft -> Tap Send -> Return to Inbox. Zero horizontal scrolling, minimum 44x44px touch targets.

  **Implementation Prompt**:
  As an Implementer agent, your task is to build the foundational data model, API layer, and 375px mobile-first UI for the new Native Omnichannel Inbox, replacing the external Chatwoot dependency.
  - **User-facing outcome**: The owner (e.g., Maya) opens the OHC app, taps "Inbox", and sees a unified list of messages. She can tap a message, see a real-time thread, and reply.
  - **Critical User Journey (CUJ)**:
    1. Owner logs in.
    2. Owner navigates to Inbox.
    3. Owner views a unified conversation thread containing messages.
    4. Owner sends a reply, which is persisted and displayed instantly.
  - **Acceptance Criteria**:
    - The external Chatwoot dependency must be fully removed or bypassed for this flow.
    - The feature must be implemented in Rust (backend) and Flutter/web UI.
    - The UI must perfectly fit a 375px viewport with no horizontal scrolling.
    - All backend records must strictly enforce multi-tenant isolation via `tenant_id`.
    - Zero mock data in the UI; data must flow end-to-end from the database.
    - Comprehensive Playwright E2E tests must verify the entire flow from navigation to sending a message.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
