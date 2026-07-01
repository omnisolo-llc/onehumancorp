issue_title: "Architectural Implementation: Unified Omnichannel Work Triage & AI Coordination Engine"
issue_description: |
  # Mission Queue Protocol: Unified Omnichannel Work Triage Architecture

  ## 1. Problem Statement
  Small business owners and operators (our core personas like Maya the Baker, Carlos the Handyman, and Priya the Boutique Owner) are drowning in fragmented communication channels (Instagram DMs, SMS, WhatsApp, Webchat, Email). They miss critical revenue opportunities because they are busy doing the actual work and cannot monitor 5 different inboxes. Current market solutions (Shopify Inbox, HubSpot) aggregate messages but still require manual human triage and reading.

  The core gap: OHC lacks a unified, highly scalable, multi-tenant architecture to ingest omnichannel events, normalize them into a single standard, and automatically coordinate AI agents (Operations, Customer Service, Sales) to triage the work and propose an action card (e.g., "Drafted Reply", "Booking Proposal") to the owner.

  ## 2. Research Report
  - **Codebase & Docs Audit:** The `agent_feed_deep_dive.md` outlines the vision of an Agent Feed but lacks the concrete, scalable data model, multi-agent coordination protocol (Redlock/Job Queue), and strict RLS multi-tenant schema required to make this production-ready for thousands of tenants.
  - **Competitive Analysis:**
    - **Shopify Inbox:** Good aggregation, but purely reactive. AI only does basic auto-replies based on rigid rules.
    - **WeCom / DingTalk:** Excellent enterprise routing, but too complex for a 1-person baker.
    - **OHC Opportunity:** Shift from "Read your messages" to "Approve this work". When an IG DM asks for a custom cake, OHC doesn't just show the message; it checks inventory (Operations Agent), drafts a quote (Sales Agent), and presents an "Approve & Send Quote" button.

  ## 3. Design Doc (Architecture)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (IG/SMS)
      participant G as API Gateway (Ingest)
      participant Q as AI Job Queue (Postgres SKIP LOCKED)
      participant T as Work Triage Agent
      participant O as Ops/Sales Agents
      participant DB as Postgres (RLS Isolated)
      participant M as Mobile App (375px)

      C->>G: Inbound Message Webhook
      G->>Q: Enqueue Raw Event
      Q->>T: Dequeue & Normalize Message
      T->>DB: Save to `unified_messages` (tenant_id)
      T->>O: Request Context (Inventory/Calendar)
      O-->>T: Context Provided
      T->>DB: Generate `action_card` (Drafted Reply/Quote)
      T->>M: Push Notification / WebSocket
      M->>DB: Owner taps "Approve"
      DB->>G: Dispatch Response to Customer
  ```

  ### Data Model & Multi-Tenancy Invariants
  - **`unified_messages`**: `id`, `tenant_id`, `channel` (ig, sms, email), `sender_id`, `raw_payload`, `normalized_text`, `status`.
  - **`action_cards`**: `id`, `tenant_id`, `message_id`, `card_type` (reply_draft, quote_approval, task), `content_json`, `status` (pending, approved, discarded).
  - **Security**: PostgreSQL Row Level Security (RLS) mandated on `tenant_id`. Redis distributed locks (`ohc:lock:{tenant_id}:triage:{conversation_id}`) to prevent duplicate AI agent processing.

  ### Mobile UX Flow (375px First)
  - **Home Screen (The Feed):** Stack of unread `Action Cards` resembling a clean, premium macOS translucent glass interface.
  - **Interaction:** Owner sees card: *"Customer Maya asked about Vegan Cakes for Friday."* Below it, the AI draft: *"Yes, we can do that! It will be $50. [Payment Link]"*.
  - **Actions:** Large, thumb-friendly 44x44px touch targets for "Approve & Send", "Edit Draft", or "Swipe to Dismiss".
  - **Offline/Flaky Network:** Tap "Approve", the UI immediately optimistic-updates and transitions the card away. Background sync queue ensures delivery when network returns.

  ## 4. Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to implement the Unified Work Triage Core Engine.
  1. **Database Schema:** Create the PostgreSQL migrations for `unified_messages` and `action_cards` with strict `tenant_id` RLS policies.
  2. **Job Queue Foundation:** Implement the Postgres `SKIP LOCKED` job queue for processing inbound webhook events into normalized messages.
  3. **Agent Coordination:** Build the Go/Rust service layer that reads the normalized message, requests context, and generates an `Action Card` record.
  4. **Mobile API:** Expose REST/gRPC endpoints for the Flutter mobile app to fetch pending `action_cards` and submit approvals.
  5. **Verification:** Implement 100% unit test coverage for the queue and at least 2 Playwright E2E tests simulating the owner logging in, seeing an Action Card, and approving it.

  Do NOT prescribe specific internal variable names. Focus on fulfilling the CUJ: A webhook comes in, and the owner sees an actionable card on their mobile feed.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical path for "AI Assistant" value prop).
  - **Estimated Scope:** Large (Requires DB, Backend, and API definitions).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
