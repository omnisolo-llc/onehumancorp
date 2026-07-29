issue_title: "Native Rust Omnichannel Chat System: WhatsApp & Web Widget Integration"
issue_description: |
  # Problem Statement
  Small business owners receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  We are replacing external dependencies with a native Rust implementation inside `onehumancorp/mono` to achieve full feature parity, better performance, and zero-trust security.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone".
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **Source Code Audit:** I have cloned and audited the legacy external chat engine source code. I analyzed its omnichannel data models (`app/models/inbox.rb`, `app/models/conversation.rb`, `app/models/message.rb`, `contact.rb`, etc.), channel adapters, WebSocket real-time messaging, and inbox architecture. We need to replicate these robust models (with `tenant_id` isolation instead of its `account_id`) natively in Rust, along with its webhook parsing and omnichannel routing concepts.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) reads messages, queries the customer's omnichannel identity graph, and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp/Insta DM/Email Webhooks] -->|Ingest| B(Omnichannel Gateway - Rust)
      C[Web Widget] -->|WebSocket| B
      B --> D{Customer Identity Resolution}
      D -->|Lookup| E[(PostgreSQL - RLS Tenant Isolated)]
      D --> F[Event Mesh]
      F --> G[The Ambassador Agent]
      G -->|Query Context| E
      G -->|Draft Reply| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J --> A/C
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (WhatsApp)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Translucent Glass cards, blurred background, UniFi modular dashboard layout.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via event mesh. Uses RAG against the tenant's product catalog and customer history to draft personalized replies.

  ### Key Design Decisions
  - **Native Rust:** Build controllers, channel adapters, and models natively in Rust based on our source audit.
  - **Proactive Drafting:** Move from read-reply to read-approve.
  - **Tenant Isolation:** Strict RLS in PostgreSQL (`tenant_id`).

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer sends a message on WhatsApp or the web widget, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. Implement a native Rust Omnichannel Gateway (ChannelAdapters) that accepts WhatsApp webhooks and Web Widget WebSocket connections.
  2. Implement native Rust data models reflecting core entities (Inbox, Conversation, Message, Contact) but strictly mapped to `tenant_id` for RLS.
  3. Implement Customer Identity Resolution to match incoming external identifiers to internal contact/customer records.
  4. Ensure RLS by `tenant_id` is strictly enforced on all new database models (Inboxes, Conversations, Messages).
  5. Integrate with the existing event mesh to trigger The Ambassador Agent upon new message ingestion.
  6. The Agent drafts a reply and queues it for owner approval.
  7. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve", and the system dispatches the message.
  8. Achieve 100% unit test coverage for new Rust code.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
