issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (the baker taking custom orders via Instagram) and Carlos (the handyman fielding SMS quote requests) suffer from severe operational fatigue managing multiple messaging platforms. Previously, OHC relied on an external dependency (Chatwoot) which introduced complexity, multi-tenancy risk, and latency. Chatwoot as an external service is now 100% RETIRED.
  We must implement a native Rust high-performance, multi-tenant omnichannel customer support and chat engine inside `onehumancorp/mono` to achieve 100% feature parity with Chatwoot, ensuring seamless mobile-first operations for owners, zero-trust security, and real-time AI Ambassador capabilities.

  ## Research Report
  *   **External Dependency Retirement:** The mandate is clear: "Complete Chatwoot Retirement." Chatwoot source code (`https://github.com/chatwoot/chatwoot`) was audited (specifically models, channel adapters like IG/FB/WhatsApp/SMS, WebSocket real-time messaging, and inbox architecture).
  *   **Target Architecture:** The new native Rust system must replicate core features like omnichannel data models, controllers, channels, WebSocket real-time messaging, inboxes, SLA policies, macros, canned responses, and agent routing natively inside the OHC ecosystem.
  *   **Competitor Systems Audit:** Unlike Shopify, Wix, or Squarespace that rely on 3rd party apps or basic contact forms, OHC’s native Rust chat system will empower the **Ambassador Agent**—an invisible, always-on AI representative that understands the business context and drafts replies directly within the unified inbox.
  *   **Scale and Performance:** Building in Rust allows for massive concurrency and minimal latency required for a high-scale event mesh (NATS) that manages inbound/outbound messaging.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ UNIFIED_INBOX : manages
      UNIFIED_INBOX ||--o{ INBOX_MESSAGE : contains
      CUSTOMER_CHANNEL ||--o{ INBOX_MESSAGE : "Ingests/Dispatches"

      UNIFIED_INBOX {
          string tenant_id PK
          string inbox_id PK
          string name
          boolean ai_ambassador_active
      }

      INBOX_MESSAGE {
          string message_id PK
          string tenant_id FK
          string channel_type
          string status
          datetime created_at
          boolean requires_human_escalation
      }

      INBOX_MESSAGE ||--o| AI_DRAFT : generates

      AI_DRAFT {
          string draft_id PK
          string message_id FK
          string proposed_content
          string approval_status
      }

      UNIFIED_INBOX ||--o{ MOBILE_UI : "Syncs to"
  ```

  ### Mobile UX Flow (375px Baseline)
  *   **Core Layout:** macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards.
  *   **Feed View (The Queue):** A vertically scrolling list of cards representing active conversations. No horizontal scrolling.
  *   **Badging & Status:** Icons indicate source (Instagram, SMS, Email). Green spark for AI handled, red pulse for Human Required.
  *   **Thread View:** Translucent gradient bubbles. AI-generated drafts appear in a subtle yellow-tinted glass bubble with 1-tap "Approve" or "Edit" buttons.
  *   **Notifications:** "✨ AI booked a $150 cake order from Instagram. No action needed." Or "⚠️ Instagram DM: Custom 5-tier wedding cake. Human input required."

  ### AI Agent Integration Points
  *   **Ambassador Agent (The Silent Ambassador):** Hooks natively into the Rust inbox service (listening to NATS events). It consults CS, Ops, and Finance departments to check inventory, calendar, and pricing before generating drafts.
  *   **Context-Aware Drafting Engine:** Saves drafts to the Inbox Ledger as `status: PENDING_APPROVAL` for the owner to 1-tap approve.

  ### Key Design Decisions (Why, not How)
  *   **Native Rust Ownership:** Eliminates the latency, operational overhead, and security risks of integrating an external service like Chatwoot.
  *   **Unified Thread Model:** Normalizes all channels (IG, WhatsApp, SMS) into a single "Customer Profile" and "Thread." Small business owners care about the customer and the request, not the channel.
  *   **Zero-Trust Isolation:** Strict multi-tenant row-level security in PostgreSQL and SPIFFE identities at the router level guarantee no cross-tenant message contamination.
  *   **Invisible by Default:** The AI acts as a middleware interceptor, drafting responses in the background without manual invocation per message.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your objective is to implement the "Native Rust Omnichannel Chat System" to fully replace Chatwoot.
  1.  **Backend Services:** Build the native Rust microservices inside `onehumancorp/mono` that replicate Chatwoot's omnichannel data models, channel adapters (e.g., WhatsApp, Instagram, SMS), WebSocket real-time messaging, and inbox architecture.
  2.  **AI Integration:** Integrate the Ambassador Agent to listen to incoming messages via NATS, fetch tenant context, and draft replies labeled as `PENDING_APPROVAL`.
  3.  **UI/UX:** Build the Mobile-First (375px) Unified Inbox UI using the OHC Premium Token library (Translucent Glass + UniFi modular cards) where owners can view threads and 1-tap approve AI drafts.
  4.  **Acceptance Criteria:**
      *   Full removal/retirement of any external Chatwoot dependencies.
      *   Successful end-to-end processing of a mocked SMS and IG DM into a unified thread.
      *   AI agent successfully generates a draft reply for a mocked inquiry.
      *   Strict multi-tenant isolation enforced.
      *   100% unit and E2E test coverage for all new Rust services and UI flows.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust]
assignees: []