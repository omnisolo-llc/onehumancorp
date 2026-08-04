issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture Design

  ## Problem Statement
  Small business owners and operators (such as Maya the custom baker, Carlos the handyman, Fatima the food cart owner, Priya the boutique operator, and Leo the music tutor) handle custom sales and client relationships across multiple unlinked channels (Instagram DMs, WhatsApp, SMS, and Web Widgets). Standard platform unified inboxes (like Shopify Inbox or Wix Inbox) simply aggregate messages as raw text feeds, lacking the customer's transaction/booking context, and requiring high-friction manual typing. Modern enterprise tools (like Zendesk or Intercom) are too complex and expensive.

  OHC is retiring its third-party Chatwoot integration and replacing it with a custom high-performance, native Rust omnichannel support and customer memory platform. This new native chat system unifies incoming channels, resolves customer identity across multiple sources, feeds the Customer Success Agent (The Ambassador) for proactive AI context-aware drafting, and provides premium Mac-style glassmorphism UI layouts for 1-tap operator approval—all while guaranteeing perfect, strict multi-tenant row-level isolation and SQLite fallback.

  ---

  ## Chatwoot Feature Benchmarking & Omnichannel Architecture Audit
  To achieve 100% native Rust feature parity, we audited the open-source Chatwoot data structures and message routing flows. Chatwoot represents its omnichannel platform using these primary tables:
  1. **accounts**: The multi-tenant billing and configuration boundary (equivalent to OHC's `tenants`).
  2. **inboxes**: Logical routing channels (e.g., an Instagram inbox, a Web Widget inbox, or an Email inbox).
  3. **channel_web_widgets / channel_email / channel_facebook_pages / channel_instagram / channel_sms / channel_twilio_sms / channel_whatsapp**: Channel-specific adapters that map connection credentials and provider webhooks.
  4. **contacts**: Represents the customer. Contacts hold email, phone, custom attributes, and a unique identifier.
  5. **contact_inboxes**: Links a `contact` to a specific `inbox` with a provider-scoped `source_id` (e.g., an Instagram sender ID, or a phone number). This is the key identity resolution link.
  6. **conversations**: Represents a thread of interaction. Belongs to a contact, an inbox, and holds assignment states (assignee_id), priorities, and statuses (open, snoozed, resolved).
  7. **messages**: Individual chat units. Belongs to a conversation, has `message_type` (0: incoming, 1: outgoing, 2: activity/system message, 3: private note), content, and sender reference.
  8. **attachments**: Associated media files linked to messages.

  ---

  ## OHC Native Rust Architecture Design

  ### 1. Unified Tenant-Isolated Database Schema
  The native OHC database schema matches Chatwoot's core capabilities, implemented natively in Rust (`sqlx`) using PostgreSQL with row-level security (RLS) and SQLite for offline standalone deployments.

  #### PostgreSQL Tables with Strict RLS Isolation:
  - **chat_inboxes**: Defines the channel category and tenant routing rules.
  - **chat_channels**: Stores provider webhook credentials and API config.
  - **chat_contacts**: The canonical customer identity.
  - **chat_contact_identities** (replaces `contact_inboxes` / identity graph): Maps an external provider's unique channel-specific ID (e.g., `instagram_user_123`, `whatsapp_phone_9876`) to a single canonical `chat_contact` for that tenant.
  - **chat_conversations**: Thread state, SLA timers, priority, assignee, and status.
  - **chat_messages**: Content, translation indicators, sender types (customer, agent, bot, private), and content attributes.
  - **chat_attachments**: Bounded media references (GCS/MinIO paths or local SQLite folders), strictly prohibiting direct base64 blobs in database.

  #### Mermaid.js Entity-Relationship Diagram:
  ```mermaid
  erDiagram
      tenants ||--o{ chat_inboxes : owns
      chat_inboxes ||--o{ chat_channels : routes
      chat_inboxes ||--o{ chat_conversations : groups
      chat_contacts ||--o{ chat_contact_identities : resolves
      chat_contacts ||--o{ chat_conversations : starts
      chat_conversations ||--o{ chat_messages : contains
      chat_messages ||--o{ chat_attachments : references

      tenants {
          string id PK
          string name
      }
      chat_inboxes {
          uuid id PK
          string tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      chat_channels {
          uuid id PK
          string tenant_id FK
          uuid inbox_id FK
          string channel_type
          jsonb config "Encrypted credentials, Meta tokens"
      }
      chat_contacts {
          uuid id PK
          string tenant_id FK
          string name
          string email
          string phone
          jsonb custom_attributes
      }
      chat_contact_identities {
          uuid id PK
          string tenant_id FK
          uuid contact_id FK
          string channel_type "instagram, whatsapp, sms, web_widget"
          string external_identity_key "provider sender_id"
      }
      chat_conversations {
          uuid id PK
          string tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          string priority "low, medium, high"
          uuid assignee_id
          timestamptz last_activity_at
      }
      chat_messages {
          uuid id PK
          string tenant_id FK
          uuid conversation_id FK
          string sender_type "contact, agent, bot, system"
          uuid sender_id
          text content
          boolean is_private
          string status "sent, delivered, read"
          timestamptz created_at
      }
      chat_attachments {
          uuid id PK
          string tenant_id FK
          uuid message_id FK
          string file_type
          string storage_path
          bigint file_size
      }
  ```

  ### 2. Omnichannel Ingress & Secure Identity Resolution
  Webhooks from channels (Meta Instagram, WhatsApp Cloud API, Twilio SMS) are processed by OHC's secure edge gateways:
  1. **Signature Verification**: Verifies inbound payload signatures natively using HMAC constant-time matching without parsing lossy raw strings (e.g., verifying `X-Hub-Signature-256` for Meta).
  2. **Identity Resolution Engine**:
     - Extract `sender_id` (e.g., `+15550199` or `insta_usr_abc`).
     - Query `chat_contact_identities` for the given `tenant_id` and channel.
     - If matched, retrieve the canonical `chat_contact`.
     - If unmatched, check if `sender_id` matches an existing `email` or `phone` in `chat_contacts`.
     - If still unmatched, create a new `chat_contact` and insert a new `chat_contact_identities` link.
  3. **Single Transaction Ingestion**: The gateway inserts the message, links the conversation thread, updates the last activity timestamp, and appends a background AI triage job in a single, atomic database transaction.

  ---

  ## Mobile-First Visual UX Flow (375px First)
  Every interface conforms to OHC's **Premium macOS-style Translucent Glassmorphism** design system with modular, tactile elements perfect for mobile-first hand operations.

  - **Screen 1: Priority Activity Feed (375px)**
    - Restrained translucent glass cards with blurred backing.
    - Prominent status indicator: "Action Required" badge in vibrant sunset amber.
    - Display card: "New Custom Order Inquiry - Instagram" from Maya's customer Sarah.
    - Explains context in 1 line: "Sarah (Instagram) asked about Gluten-Free Vegan Chocolate Cake."
    - AI Suggestion: Contains a pre-rendered glass container showing the drafted reply.
  - **Screen 2: Interactive Inbox & Context Pane (375px)**
    - Upper Card: Customer context dashboard showing Sarah's active profile, lifetime value, and past purchase history (" Sarah ordered 1 Custom Birthday Cake on March 15th").
    - Lower Panel: Interactive chat message thread.
    - Action Bar: Prominent 44x44px primary action button ("1-Tap Approve & Send") and a secondary "Edit Draft" button which pulls the native mobile keyboard instantly.

  ---

  ## AI Agent Orchestration (The Ambassador)
  The AI work buddy operates as a unified coordinate of agents:
  1. **The Ambassador (Customer Success Agent)**: Triggered by incoming event signals. It runs RAG against the tenant's product catalog and local context memories to form the reply draft.
  2. **The Manager (Operations Agent)**: If the customer's DM involves a transaction (e.g., "Do you have slots for Friday?"), The Ambassador invokes The Manager via the inter-agent mesh to query the active booking calendar/inventory before drafting the availability response.
  3. **Real-time Event WebSockets**: Once the AI completes the draft, a secure, single-use, cryptographically signed token is issued for the client's WebSocket subscription. This pushes the new action card to the owner's mobile screen in real-time, matching PowerSync subscription boundaries.

  ---

  ## Customer Support Journeys (Personas Evaluated)

  ### Maya - Custom Baker
  - **Context**: Maya is baking and gets an Instagram DM from Sarah: "Do you make vegan vanilla cake for this Friday?"
  - **Journey**:
    1. Meta webhook posts payload to OHC.
    2. Identity resolver matches Sarah's Instagram username to her existing contact card (Sarah ordered custom birthday cupcakes last month).
    3. The Ambassador agent queries Maya's product catalog and checks current availability for Friday.
    4. The Ambassador drafts the reply: "Hi Sarah! Yes, I can make a 10-inch Gluten-Free Vegan Vanilla Cake for Friday. Click here to confirm your reservation slot."
    5. Maya receives an active notification on her iPhone, taps "1-Tap Send", and continues baking.

  ### Carlos - Field Service Owner
  - **Context**: Carlos is on a repair job. A new customer, Dave, sends an SMS: "Hey Carlos, can you look at my broken HVAC tomorrow afternoon?"
  - **Journey**:
    1. Twilio SMS webhook captures Dave's phone number.
    2. Identity resolver identifies Dave as a new customer, auto-creates his contact record, and triggers a new Service Lead event.
    3. The Operations agent checks Carlos's Google Calendar integration for tomorrow afternoon slots.
    4. The Ambassador drafts an SMS reply with Dave's details: "Hey Dave! I can stop by between 2:00 PM and 4:00 PM tomorrow. Let me know if that works!"
    5. Carlos views his Android feed, taps "Confirm Slot", sending the SMS back through Twilio automatically.

  ### Priya - Boutique Operator
  - **Context**: Priya is managing her shop. An online customer visits her store's embedded Web Widget: "Is the emerald dress in size Medium available in-store?"
  - **Journey**:
    1. The native OHC widget client opens a secure sandboxed WebSocket connection.
    2. The message is ingested. The Ambassador checks the physical retail inventory database.
    3. AI identifies size Medium is in stock (2 left), and drafts a message: "Hi! Yes, we have 2 Medium emerald dresses left in-store. Would you like me to hold one for you to try on today?"
    4. Priya approves the draft on her phone, and the customer is instantly notified via the widget.

  ---

  ## Implementation Prompt (for Implementer Agent)

  **User-Facing Outcome:** Build a fully unified native Rust omnichannel customer support and identity resolution chat engine. When a customer sends a message over an external mock channel (Instagram, SMS, Web Widget), the OHC system must securely resolve their identity across channels, persist the conversation thread, invoke the AI Ambassador agent to generate a draft reply, and push an interactive action card to the operator's mobile-sized dashboard in real-time for 1-tap approval.

  **CUJ & Acceptance Criteria:**
  1. **Omnichannel Ingress & Gateway**: Implement signature verification for webhook ingress routes, natively checking HMAC constant-time signatures.
  2. **Multi-Tenant Identity Resolution Engine**: Build Rust/sqlx handlers for `chat_contacts` and `chat_contact_identities`. Resolve identities by mapping channel identifiers, falling back to email/phone lookup before auto-creating a new record. Write unit tests proving that cross-tenant queries are blocked by RLS policies.
  3. **AI Ambassador Draft Generation**: When an incoming customer message is inserted, enqueue a triage background job. Trigger the AI agent to query the catalog/memory database, draft a response, and store it in `chat_messages` as an unapproved draft.
  4. **Tauri / Web Real-Time WebSocket Delivery**: Create the `/api/v1/auth/realtime-ticket` ticket route. Issue single-use, audience-bound JWT ticket tokens to establish secure, tenant-isolated WebSocket connections. Push conversation and draft updates in real-time.
  5. **Operator Layouts & Playwright Verification**: Build a beautiful, responsive glassmorphic UI layout supporting all three visual modes (AI priority console, classic three-pane, focus-first) adapting to a 375px mobile viewport. Write at least 5 Playwright E2E tests verifying:
     - Ingesting a mock external customer message webhook.
     - Login, navigation to the unified feed, and verify the activity feed displays the new action required card.
     - Tapping the action card displays the correct customer purchase history context and the AI draft reply.
     - Tapping "Approve" successfully sends the draft, transitions the message state to `sent` on the thread, and dispatches the outbound payload.
     - Modifying/Editing the draft updates the content and persists the edit correctly.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
