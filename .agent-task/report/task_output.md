issue_title: "[Architecture] Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. OHC is migrating away from a third-party Chatwoot dependency to a native Rust implementation to guarantee zero-trust multi-tenancy, tighter AI integration ("The Ambassador" agent), and a seamless mobile-first 375px experience.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses a robust `Conversation`, `Message`, `Inbox`, `Channel`, and `Contact` data model. It features dedicated channel adapters (Web Widget, API, Email, WhatsApp, Line, SMS), powerful webhook systems for events, macros, and SLA policies.
  - **OHC Gap:** Currently, OHC lacks this omnichannel infrastructure natively. To retire external Chatwoot, OHC must replicate the core Chatwoot data models, WebSocket real-time event distribution, and channel webhook ingestion securely in Rust.
  - **Shopify Inbox & Wix Inbox:** Aggregates chat but relies heavily on manual responses. OHC’s native Rust inbox will natively query the customer's full multi-tenant graph (orders, bookings, preferences) to empower the "Ambassador" AI to draft highly contextual replies automatically.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph External Channels
          A[Instagram DM]
          B[WhatsApp]
          C[Web Chat Widget]
          D[Email / SMS]
      end

      subgraph OHC Native Rust Chat Engine
          E[Channel Webhook Ingestion API]
          F[Conversation & Message Router]
          G[(Omnichannel PostgreSQL - Multi-Tenant RLS)]
          H[Redis Pub/Sub & WebSockets]
      end

      subgraph AI & Operations
          I[Customer Identity Resolution]
          J[The Ambassador AI Agent]
          K[Action Required Feed / 375px Mobile UI]
      end

      A -->|Webhook| E
      B -->|Webhook| E
      C -->|WS/API| E
      D -->|Webhook| E

      E --> F
      F -->|Save Message| G
      F -->|Publish Event| H
      F --> I

      I -->|Contextualize| J
      J -->|Draft AI Reply| G
      J -->|Push to Owner Feed| K
      H -->|Real-time update| K
  ```

  ### Mobile UX Flow (375px)
  1. **Notification/Feed:** Owner (Maya) sees an "Action Required: Draft Reply" card on her 375px home feed indicating a new Instagram DM.
  2. **Unified Inbox View:** Tapping the card opens the chat. The UI displays the DM alongside the customer's past cake orders (pulled via Identity Resolution).
  3. **AI Draft:** A pre-drafted response from "The Ambassador" AI sits in the composer ("Hi, yes we can do a vegan version! It requires a $20 deposit. Here is the link...").
  4. **Action:** Maya taps "Send" (or edits). The system dispatches the message back via the Instagram channel adapter.
  5. **Material Design:** The UI utilizes translucent glass styling, ensuring sticky headers and non-obtrusive virtual keyboard behavior.

  ### AI Agent Integration
  - **The Ambassador:** Triggered automatically upon new incoming messages via the Redis event bus. Queries the tenant's unified database (using strict RLS) to understand past transactions and preferences.
  - **Drafting:** The AI generates a `draft` status message in the `Messages` table. The owner reviews this draft on the mobile app before it transitions to `sent` and hits the external channel API.

  ### Data Model Invariants (Native Rust Replication)
  - `Inbox`: Represents a configured channel (e.g., Maya's Instagram account).
  - `Contact`: A unified customer profile across channels.
  - `Conversation`: A thread between a `Contact` and an `Inbox`.
  - `Message`: Individual payloads (text, attachments).
  - **Multi-Tenancy:** ALL tables must enforce row-level security (RLS) using `tenant_id`.
  - **Real-time:** Updates pushed to the frontend via WebSockets authorized by SPIFFE/SPIRE identity tokens.

  # Implementation Prompt
  **Role:** Backend/Frontend Implementer
  **Objective:** Implement the core Native Rust Omnichannel Inbox data models, API endpoints, and mobile-first 375px unified inbox UI, retiring external Chatwoot.
  **CUJ:** An owner receives a webhook payload from a channel (simulated API call), a new `Conversation` and `Message` are created in the database, "The Ambassador" AI drafts a reply, and the owner sees this in their mobile-first Unified Inbox UI and approves it.
  **Acceptance Criteria:**
  - Rust API implemented for `Inbox`, `Conversation`, and `Message` creation with robust multi-tenant RLS.
  - Webhook ingestion endpoint implemented to receive standard message payloads.
  - Mobile-first React/Next.js UI (usable on 375px width) displaying the conversation and AI draft.
  - No external Chatwoot dependencies used.
  - 100% unit test coverage and at least 5 Playwright E2E tests validating the complete flow from webhook ingestion to owner approval in the UI.

  # Priority & Scope
  **Priority:** P0 (Critical for platform independence and core AI functionality)
  **Scope:** Large (Involves database, Rust backend, and mobile UI)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat, rust, mobile-first]
assignees: []
