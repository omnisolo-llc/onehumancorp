issue_title: "Architecture Design: Native Rust Omnichannel Customer Support & Chat Engine"
issue_description: |
  # MISSION QUEUE PROTOCOL: ARCHITECTURAL SPECIFICATION

  ## 1. Title
  Architecture Design: Native Rust Omnichannel Customer Support & Chat Engine (Chatwoot Replacement)

  ---

  ## 2. Problem Statement
  Communication fragmentation is a major blocker for small business owners and operators (such as Maya the baker, Carlos the handyman, Fatima the food cart operator, and Priya the boutique owner). Customers reach out through multiple disconnected platforms: Instagram DMs, WhatsApp messages, emails, SMS, and website chat widgets.

  Managing these disconnected channels causes:
  - Switching between multiple apps, which leads to missed leads and slow responses.
  - Complete lack of unified customer context (e.g., replying to an Instagram DM without knowing that the customer bought a vegan cake last week).
  - High manual overhead in typing repetitive responses.

  Existing customer support solutions like Chatwoot, Zendesk, or Intercom are bloated, expensive, require complex integration setups, and do not fit a mobile-first, AI-assisted solopreneur workflow. Small business owners do not want an administrative portal; they need an invisible, highly secure assistant that proactively prepares accurate context-aware responses and lets them approve and dispatch with 1-tap.

  ---

  ## 3. Research Report & Chatwoot Benchmarking

  ### Benchmarking Chatwoot
  Chatwoot is an open-source customer communication platform. An audit of Chatwoot's core architecture (`github.com/chatwoot/chatwoot`) reveals several major components:
  1. **Omnichannel Inboxes**: Routing boundaries associated with channels (e.g., Web Widget, WhatsApp Business, Facebook Messenger, Twilio, Email/IMAP).
  2. **Contacts & Contact Identities**: A Contact represents a customer. A Contact Identity links third-party identifiers (e.g., social handles, phone numbers, email addresses) to the single Contact.
  3. **Conversations**: Channel-scoped threads containing status, priority, SLAs, and assignment state.
  4. **Messages & Attachments**: Rich message data, private agent notes, and media pointers.
  5. **Real-time Gateway**: Rails ActionCable-based WebSockets carrying typing indicators, presence, and new message dispatches.

  ### OHC Opportunity
  To achieve 100% native performance, high security, and offline support, OHC retires Chatwoot completely. We replace it with a native, high-performance, multi-tenant Omnichannel Chat & Support Engine written in Rust within `onehumancorp/mono`.

  This engine replicates Chatwoot's primary features while enhancing them in three ways:
  - **First-Class AI Drafting**: Pre-drafts replies using Gemini Pro/GPT-4o before the operator opens the inbox, presenting them in a 1-tap action queue.
  - **Seamless Multi-Tenancy**: Guaranteed row-level security (RLS) on PostgreSQL and automatic predicate injection on SQLite (desktop Tauri).
  - **Local-First Sync**: PowerSync-enabled replication with local-first SQLite databases, maintaining sub-100ms responsiveness even on low-data or offline connections.

  ---

  ## 4. High-Level System Architecture & Design

  ### Architecture Diagram
  The diagram below illustrates the end-to-end data flow from incoming provider events to the transactional outbox delivery.

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant Provider as Meta/Twilio Webhook
      participant Gateway as Inbound Ingestion Gate (Rust)
      participant CRM as Identity Resolution Engine
      participant DB as Multi-Tenant Database (RLS)
      participant Agent as AI Ambassador / Manager
      participant Outbox as Transactional Outbox Worker

      Customer->>Provider: Sends Instagram DM / WhatsApp
      Provider->>Gateway: Signed Webhook Delivery
      Note over Gateway: Verifies cryptographic signature & freshness
      Gateway->>CRM: Normalize Input & Resolve Identity
      CRM->>DB: Query Contact Identity & Customer History
      DB-->>CRM: Resolved Contact ID & Context
      Gateway->>DB: Write Inbound Message & Conversation Status (Tx)
      Gateway-->>Provider: HTTP 200 OK (Idempotent Commit)

      Note over Agent: Triggered by Tx Outbox event
      Agent->>DB: Query Product Catalog & Order Ledger
      Agent->>Agent: Draft context-aware reply
      Agent->>DB: Write proposed AI draft state (status: draft)

      Note over Outbox: Outbox processes Operator Approved Draft
      Outbox->>Provider: Dispatches outgoing message payload
      Provider-->>Outbox: Accepted by Provider
      Outbox->>DB: Update Message Delivery State (Tx)
  ```

  ---

  ## 5. Data Model & Invariants

  To prevent cross-tenant leakage, every table carries a non-empty `tenant_id` string as part of its primary key or as a foreign key. PostgreSQL uses `ENABLE ROW LEVEL SECURITY` with tenant-based isolation rules. SQLite (desktop Tauri) enforces matching tenant predicates in all repository queries.

  ### Entity-Relationship Diagram (ERD)

  ```mermaid
  erDiagram
      TENANT {
          string id PK
          string name
          string display_name
      }
      INBOX {
          string id PK
          string tenant_id FK
          string name
          string channel_type
          string routing_policy
      }
      CHANNEL_CONNECTION {
          string id PK
          string tenant_id FK
          string inbox_id FK
          string provider_type
          string encrypted_credentials
          string webhook_secret
      }
      CONTACT {
          string id PK
          string tenant_id FK
          string first_name
          string last_name
          string avatar_url
          jsonb custom_attributes
      }
      CONTACT_IDENTITY {
          string id PK
          string tenant_id FK
          string contact_id FK
          string provider_type
          string identity_key
      }
      CONVERSATION {
          string id PK
          string tenant_id FK
          string inbox_id FK
          string contact_id FK
          string status
          string priority
          timestamp sla_deadline
          string assigned_agent_id
      }
      MESSAGE {
          string id PK
          string tenant_id FK
          string conversation_id FK
          string sender_type
          string sender_id
          string content
          string content_type
          string delivery_state
          timestamp created_at
      }
      ATTACHMENT {
          string id PK
          string tenant_id FK
          string message_id FK
          string storage_url
          string content_type
          string status
      }
      TRANSACTIONAL_OUTBOX {
          string id PK
          string tenant_id FK
          string event_type
          jsonb payload
          string lease_owner
          timestamp lease_expires
          string status
      }

      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_CONNECTION : configures
      TENANT ||--o{ CONTACT : manages
      CONTACT ||--o{ CONTACT_IDENTITY : links
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : groups
      MESSAGE ||--o{ ATTACHMENT : contains
      TENANT ||--o{ TRANSACTIONAL_OUTBOX : queues
  ```

  ---

  ## 6. AI Department Coordination

  AI agents operate as a cohesive team to serve the owner/operator:
  1. **Customer Success Agent (The Ambassador)**: Triggered by new incoming messages. It performs semantic search (RAG) against the tenant's Product Catalog, Knowledge base, and past conversations to draft a natural, context-aware reply.
  2. **Operations Agent (The Manager)**: If a message implies a booking, delivery, inventory check, or task reschedule, The Ambassador delegates to The Manager. The Manager checks calendar slots, calculates delivery routing constraints, or checks inventory availability before passing the structured slots back to The Ambassador.
  3. **Finance Agent (The CFO)**: If a custom-order deposit or quote is requested, The CFO agent compiles the quote details and creates a secure Stripe payment link, which is automatically appended to the draft reply.

  ---

  ## 7. Mobile-First UX Flow & Visual Excellence (375px First)

  The interface adapts the macOS Translucent Glass styling combined with clean Ubiquiti UniFi modular card layouts.

  ### Mobile Wireframe Flows
  - **Screen 1: Priority Action Queue (Home)**:
    - Restrained, translucent header.
    - Status Indicators: "Active Action Needed: 3", "Urgent SLAs: 1" (styled as micro status badges with good/warn/bad color tones).
    - Modular feed cards showing incoming messages. Tapping a card opens the interactive review panel.
  - **Screen 2: 1-Tap AI Draft Review Panel**:
    - Displays the customer context at the top: Name, channel icon (Instagram, WhatsApp, etc.), last purchase date ("bought vanilla cake 2 days ago").
    - Translucent glass card containing the AI-Proposed reply:
      "Proposed Reply (Instagram DM): Hi Maya, yes! We can customize the vegan chocolate cake for Friday. Here is a Stripe link for the $20 deposit..."
    - Large, primary button (minimum 44x44px touch target) for "Approve & Send" (styled with active background blur and clean borders).
    - Secondary "Edit" button that expands a native fluid-height text editor.

  ---

  ## 8. Technical Parity & Security Requirements

  - **Zero-Trust Workload Identity**: Uses SPIFFE/SPIRE for communication between background workers and database interfaces.
  - **WebSocket Security**: Ephemeral WebSocket subscriptions are authenticated using short-lived JWT tickets (`aud=ohc-realtime`) requested via an HttpOnly cookie-authenticated REST session.
  - **Secure Attachments**: Media uploads undergo content-type sniffing and malware scanning in a quarantined storage bucket before being accessible.
  - **Offline Sync**: Leverages PowerSync with client-side SQLite storage to guarantee sub-100ms user interface responsiveness and offline persistence.

  ---

  ## 9. Implementation Prompt for the Engineering Swarm

  **Proposed Outcome**: Build a high-performance, multi-tenant Omnichannel Customer Support & Chat Engine written in Rust within `onehumancorp/mono` to replace Chatwoot completely.

  **Critical User Journey (CUJ)**:
  1. Maya (baker) receives a webhook notification from Meta signifying a customer inquiry on Instagram DMs ("Do you do vegan chocolate?").
  2. The ingestion layer verifies the webhook signature, maps the event to a canonical `ReceiveMessage` payload, resolves the contact, and commits the message in a database transaction under Maya's `tenant_id`.
  3. An automated outbox worker triggers the Customer Success Agent (The Ambassador), which scans Maya's cake catalog and drafts a personalized reply ("Yes, our custom vegan chocolate starts at $50...").
  4. Maya opens the OHC mobile app on her 375px phone screen, sees a clear notification, reviews the draft, and presses "Approve & Send".
  5. The transactional outbox safely dispatches the reply back to the Meta endpoint, preserving state changes.

  **Acceptance Criteria**:
  - Zero Chatwoot residue remains across manifests, deployments, or configs.
  - Row-Level Security (RLS) is fully tested with unit tests demonstrating cross-tenant access is rejected.
  - Ephemeral WebSocket gateways successfully authorize connections using single-use tickets.
  - Provide at least 5 automated Playwright E2E tests simulating the entire intake-draft-approval loop.
  - Visual components pass responsive layout audits on 375px/768px viewports.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
