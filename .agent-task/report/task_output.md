issue_title: "Architectural Gap: Unified Multi-Channel Work Triage & AI Inbox Engine"
issue_description: |
  ## Title
  Architectural Gap: Multi-Channel Work Triage & AI Inbox Engine

  ## Problem Statement
  Owners like Maya (Baker) and Carlos (Handyman) are drowning in inquiries spread across Instagram DMs, SMS, WhatsApp, and emails. They miss leads because they cannot monitor 5 different apps while working. The gap: OHC lacks a unified, multi-tenant inbox architecture where all incoming messages are ingested into a single feed, triaged by an AI Work Triage Agent, and grouped into actionable work tasks (e.g., "Draft quote," "Schedule delivery").

  ## Research Report
  - **Shopify Inbox**: Good for store chats, but poor integration with SMS and third-party social DMs without paid apps.
  - **Wix Inbox**: Better unified approach but lacks AI autonomous execution (only suggests replies).
  - **GoDaddy Conversations**: Basic aggregation, no contextual memory of past orders.
  - **OHC Opportunity**: OHC can differentiate by not just aggregating messages, but actively parsing them via an AI Triage Agent that turns a DM like "Can you fix my sink tomorrow?" into a proposed Calendar Booking and Drafted Quote, pending owner approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      IG[Instagram DMs] --> Ingest[Webhook Ingestion API]
      SMS[Twilio SMS] --> Ingest
      Email[Email Provider] --> Ingest
      Ingest --> DB[(Postgres: Unified Messages)]
      DB --> Triage[AI Work Triage Agent]
      Triage --> Memory[(Tenant Vector Memory)]
      Triage --> Action[Proposed Work Actions]
      Action --> UI[Mobile Flutter Shell]
      UI --> Owner[Owner Approval]
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Triage Feed (Home)**: A unified list of cards using clean Ubiquiti UniFi modular layout and translucent glass materials. Card: "Carlos: Instagram DM - Sink Repair. [Review Quote]".
  2. **Detail View**: Tapping the card shows the chat history and an AI-generated Quote Draft at the bottom. 44x44px touch targets.
  3. **Action Button**: A prominent full-width button to "Approve & Send Quote" or "Edit".

  ### AI Agent Integration Points
  - **Ingestion Hook**: Triggers the AI Work Triage Agent on every new message via PostgreSQL SKIP LOCKED job queue.
  - **Context Retrieval**: Agent queries tenant's vector memory for customer history and current pricing/availability.
  - **Action Proposal**: Agent creates a pending `WorkAction` record (draft reply, draft quote) instead of sending immediately.

  ### Key Design Decisions
  - **Single Message Table**: All channels map to a unified `messages` table with `channel_type` to simplify query logic. Row-level tenant isolation enforced via `tenant_id`.
  - **Pending Actions**: AI never auto-sends quotes; it creates drafts to maintain owner trust.

  ## Implementation Prompt
  **For the Implementer Agent**:
  Implement the backend ingestion API for the Unified AI Inbox. Create the database migration for a multi-tenant `unified_messages` table and a `pending_work_actions` table ensuring row-level security. Create a Rust REST/gRPC endpoint that accepts incoming webhook payloads, normalizes them, and triggers a background job (via the existing PG job queue) for the AI Work Triage Agent to process the message. Expose a query endpoint for the mobile app to fetch the triage feed.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
