issue_title: "Architecture Design: Native Rust Omnichannel Customer Support & Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Architecture Design: Native Rust Omnichannel Support & Chat Engine (Chatwoot Replacement)

  ## 1. Problem Statement

  For non-technical small-business owners and operators—like Maya (baker), Carlos (handyman), Priya (boutique operator), Leo (music tutor), and Fatima (food cart operator)—running a business means constantly switching between different communication channels (Instagram DMs, WhatsApp, SMS, web chat) and operational apps (calendars, payment links, order sheets, inventory logs).

  When a customer reaches out with an inquiry, owners suffer from:
  1. **Fragmented Workflows**: Messages are scattered. A customer asks Maya for a gluten-free vegan chocolate cake on Instagram DM; another texts Carlos a picture of a leaky faucet. Neither can easily turn these messages into confirmed orders, scheduled site visits, or secure deposit payments without copy-pasting details across multiple systems or sending insecure, un-tracked payment links.
  2. **High Interaction Latency & Lost Leads**: An owner who is busy baking or performing a plumbing repair cannot reply to messages instantly. By the time they check their phone, the hot lead is lost to a competitor.
  3. **Complex Administrative Portals**: Existing open-source customer support software (like Chatwoot, Zendesk, or Intercom) is bloated, requires advanced technical administration (such as managing Rails servers, Redis queues, and webhook configurations), and feels like an enterprise admin portal rather than a simple work assistant.
  4. **Unreliable External Integrations**: Third-party chat platforms are disconnected from the core business context (like current inventory levels, delivery slots, and client history). These integrations often break, drop webhooks, or expose customer data to third-party vendors.

  **The OHC Goal**: Establish a native, high-performance, and secure Omnichannel Support & Chat Engine inside `onehumancorp/mono` (completely replacing the retired third-party Chatwoot dependency). This engine unifies all incoming demands directly into a single, AI-led Unified Work Inbox that is 100% functional on a 375px mobile phone, helps owners draft quotes, book reservations, collect payments, and manage operations in seconds with full offline resilience.

  ---

  ## 2. Research & Benchmarking Report

  ### 2.1 Competitive Analysis
  To design a superior work assistant, we benchmarked our architectural design against industry-leading platforms:
  - **Chatwoot**: Omnichannel communication platform built on Ruby on Rails + Sidekiq + PostgreSQL. While feature-rich (inbox, channels, contacts, reports), its self-hosted footprint is heavy, non-secure by default, lacks deep transactional business integration, and does not support offline SQLite synchronization. We have fully retired Chatwoot as an external service and are replicating its core omnichannel capabilities natively in Rust.
  - **Shopify Inbox**: Extremely successful mobile-first inbox for merchant-to-customer chat. It integrates product recommendations, discount codes, and live checkout links directly into the chat bar. Our design replicates this native commerce integration, allowing Maya to attach products or Carlos to send deposit links with one tap.
  - **Wix Chat & Squarespace Client Portal**: Built-in website widgets that connect to bookings and contacts. They feel simple to the user but lack AI-led automatic drafting, multi-platform webhook verification, and robust Zero Trust tenant isolation.
  - **GoDaddy Conversations / HubSpot Mobile**: Unifies SMS, email, and social chat. However, they rely on heavy cloud-side API connections and do not support local-first, low-network offline capability, rendering them slow and unusable on flaky mobile data networks.

  ### 2.2 Benchmarking Native Rust vs. Rails/Third-Party
  By migrating from a Rails-based Chatwoot service to a native Rust micro-architecture inside OHC, we achieve:
  - **Resource Efficiency**: Active RAM footprint drops from >1.5GB (Rails, Sidekiq, Node) to <50MB (Rust axum/tokio service).
  - **Deterministic Verification**: All webhooks (Meta, Twilio, Resend, SendGrid) are validated using strict constant-time signature verification, rejecting tampered payloads and preventing cross-tenant injection.
  - **Zero Trust Security**: Leverage SPIFFE/SPIRE for cryptographically verifiable tenant-scoped identities. All credentials are encrypted with envelope keys and are completely inaccessible to browser environments or model prompts.
  - **Local-First Offline Parity**: Leveraging PowerSync and SQLite, owners have access to their entire historical inbox, contacts, and preferences on their phone offline. Outbox writes are queued locally and automatically converged once network connectivity is restored.

  ---

  ## 3. Architecture Design & System Docs

  ### 3.1 Architecture Diagram (Mermaid.js)

  ```mermaid
  graph TD
      %% Ingress Webhook Ingestion Flow
      subgraph WebhookIngestion [1. Webhook Ingestion]
          A[External Channel: WhatsApp/SMS/IG] -->|Signed HTTPS Request| B[Ingress Connector Webhook]
          B -->|Verify Signature & Replay Protection| C{Valid Request?}
          C -->|No| D[Reject with 400/401]
          C -->|Yes| E[Verify Tenant Context via SPIFFE/SPIRE]
      end

      %% Database Transaction Boundary
      subgraph DatabaseTransaction [2. Database Transaction Boundary]
          E --> F[Check Replay & Event Uniqueness]
          F --> G[Resolve/Create Contact Identity]
          G --> H[Create/Update Conversation & Message Sequence]
          H --> I[Write Audit Log & Outbox Event]
      end

      %% Storage & Attachment Handling
      subgraph StorageAttachment [3. Attachment Quarantine & Scanning]
          J[Media Upload/Photo Ingress] -->|Upload to Quarantine Bucket| K[Quarantine S3/MinIO Namespace]
          K -->|Malware/Sniff Scan| L{Scan Succeeded?}
          L -->|No| M[Retain & Flag Operator Error]
          L -->|Yes| N[Move to Public Signed Bucket / local storage]
      end

      %% Outbox and Retries
      subgraph OutboxDelivery [4. Transactional Outbox Delivery]
          I --> O[(Durable DB Store: PostgreSQL/SQLite)]
          O -->|Transactional Outbox| P[AI Job/Delivery Queue]
          P -->|Lease with Expiring Lock| Q[Background Delivery Worker]
          Q -->|Stable Idempotency Key| R[Outbound API: Resend/Twilio/Meta]
          R -->|Acceptance/Receipts| S[Update Message Receipt States]
      end

      %% Realtime Synchronization Gateway
      subgraph RealtimeSync [5. Realtime Sync & Offline Convergence]
          O -->|Next.js Session Auth| T[POST /api/v1/auth/realtime-ticket]
          T -->|Single-Use Ticket| U[WebSocket Gateway / Rust Client]
          O -->|PowerSync JWT Auth| V[PowerSync Server Sync Rules]
          V -->|Local-First Sync| W[SQLite Offline Replica / Flutter Client]
      end
  ```

  ### 3.2 UI Wireframes & Screen Flow (375px Mobile-First)

  #### 3.2.1 Desktop vs. Mobile Layout Compositions
  Operators can configure their workspace layout preference (persisted per user). On desktop, three compositions are available:
  1. **Classic Three-Pane**: Channel filters/queues on the left, active conversation timeline in the middle, customer context card and AI action board on the right.
  2. **AI Operations Console (Default)**: Priority queue list on the left, active chat timeline in the middle, AI policy controller and transaction panel on the right.
  3. **Focus-First Two-Pane**: Clean conversation list on the left, dedicated thread timeline in the center. Auxiliary panels are hidden behind translucent sheets.

  #### 3.2.2 Mobile Responsive Flow (375px Adaptation)
  On a 375px phone, the layout collapses to a high-focus single-view stack with a slide-out drawer navigation:
  - **Screen 1: Priority Queue / Inbox Feed**:
    - Header with translucent glass styling, displaying tenant name, global status badges (e.g., "✨ 2 AI Handled", "4 Open", "0 SLA Breaches").
    - Clean list of items with touch targets of at least 44x44px. Every row has a clear avatar, sender source indicator (e.g., `[WhatsApp] Maya's Bakery`), and a truncated message snippet.
    - Swiping left on an item opens a quick "Resolve" action; swiping right reveals "Escalate / Assign".
  - **Screen 2: Focus Chat Timeline**:
    - Clicking an item slides in Screen 2.
    - **Header**: Back button, Contact Name (e.g., `John Doe (Known Customer)`), Channel Icon, and an "AI On/Off" toggle.
    - **Timeline**: Sent messages are aligned right, received on the left. Translucent chat bubble designs (macOS-style blur with 65% opacity in light/dark mode).
    - **Composer Section**: Text input with auto-expanding line height. Quick-insert icons: Photo Attach, Product Insert, Quick Templates.
    - **AI Action Bar**: Floating at the bottom or attached above the composer. Displays active AI Drafted Actions (e.g., `[✨ Send quote for $45.00]` or `[✨ Approve & Send (Deduct Inventory)]`).

  ---

  ### 3.3 Mobile UX Flow (Non-Technical Persona Walks)

  #### Flow A: Maya Approving a Custom Cake Order (375px)
  1. Maya opens OHC on her iPhone. The app uses macOS translucent materials to show today's priority items.
  2. She see a message in her Inbox: `[Instagram DM] John: "I need a gluten-free vegan cake for Saturday. Can I order?"`
  3. The item has a status badge: `[✨ AI Handled]`.
  4. Maya taps the message. The screen transitions to the Conversation Timeline.
  5. Under the customer's message, she sees a Draft Reply drafted by the Customer Service Agent: *"Hi John! Yes, we have 2 vegan chocolate cakes left for this Saturday. I can hold one for you! Click here to make a $20 deposit: [Link]"*.
  6. Maya also sees the **Unified Customer Memory** card showing: *"John previously ordered gluten-free items. High preference for vegan desserts."*
  7. At the bottom, a prominent green action button reads: **`✨ Approve & Send (Deduct Inventory)`**.
  8. Maya taps the button. The status changes to *"Draft approved and sent"*.
  9. Behind the scenes:
     - The database transaction records the status as "resolved".
     - The AI CS Agent posts the DM.
     - The Inventory Agent decrements the cake inventory count.
     - The Delivery Calendar Agent reserves the Saturday delivery slot.

  #### Flow B: Carlos Triaging a Repair Lead (375px)
  1. Carlos is at a plumbing site. A push notification appears on his phone: `[SMS] New photo received from Sarah regarding a leaky faucet.`
  2. Carlos taps the notification. OHC opens directly into the thread with Sarah.
  3. He sees a message: *"Here is where it is leaking under the kitchen sink."* below a photo attachment card.
  4. The photo card displays: `[🔍 Scanning attachment for security...]` (quarantined mode).
  5. Carlos waits 2 seconds. The scanner completes successfully, and the photo preview fades in beautifully using standard Translucent Glass border styling.
  6. The AI Operations Assistant analyzes the image context (under-sink copper pipe corrosion) and drafts a proposal: *"Estimated 1.5 hours of labor + copper fitting replacement. Total: $225.00. [Link to Book & Pay Deposit]"*.
  7. Carlos taps **`✨ Send quote for $225.00`**. The message is immediately converted into a Twilio SMS and sent.

  ---

  ### 3.4 AI Agent Integration & Departments Coordination

  OHC agents behave as a synchronized department cluster instead of fragmented chat bots:
  - **Work Triage Agent**: Evaluates incoming customer messages across channels, classifies intent (Inquiry, Complaint, Booking, Payment, Support), extracts sentiment and priority, and schedules them in the owner's active queue.
  - **Customer Relationship Agent**: Reads past contact context, attributes, segments, and custom preferences to generate hyper-personalized draft replies. Translates cross-language inputs automatically (e.g., Fatima's Arabic pre-orders are translated to English, and English drafts are translated to Arabic).
  - **Operations & Inventory Agent**: Coordinates bookings, reservations, and inventory. Inspects real-time inventory levels before drafting replies (e.g., confirming "2 vegan cakes left") and triggers real-time ledger deductions upon owner approval.
  - **Finance & Revenue Agent**: Formulates quotes, drafts invoice proposals, schedules deposit payment reminders, and generates Stripe payment links.
  - **AI Safety Fences**:
    1. **Fencing & Takeover**: Once an owner starts typing manually in the composer, an active automation fence increments. This immediately invalidates and halts any active AI background draft jobs for that conversation.
    2. **Strict Budgets**: Tracks per-conversation token usage, tool-call rates, and model budgets. Out-of-budget occurrences block automated sending and force escalation.
    3. **Action Restrictions**: Material side effects (such as refunding payments, issuing discounts, altering calendar availability, or committing to contracts) are strictly read-only for AI. They can only be drafted and require explicit operator tap-to-approve authorization.

  ---

  ### 3.5 Key Design Decisions & Technical Integrity

  - **Transactional Outbox for Guaranteed Delivery**:
    We separate Message States (`draft`, `committed`, `redacted`, `deleted`), Delivery Job States (`queued`, `leased`, `retry_wait`, `completed`, `dead_letter`), Attempt States, and Receipt States (`provider_accepted`, `sent`, `delivered`, `read`, `bounced`, `failed`).
    A message is never reported as "sent" unless we have explicit provider evidence. If network connectivity fails, the delivery worker retries with exponential backoff and jitter, eventually placing exhausted items in a dead-letter queue.
  - **Malware Scanning Sandbox**:
    Attachments are size-restricted and sniffed. They begin in a private quarantine S3/MinIO bucket. Background scanner tasks scan the media using safe ClamAV/Yara pipelines. If successful, they are promoted to the public CDN-fronted bucket using short-lived pre-signed URLs. SVG, HTML, and active elements are converted into inert PNGs or forced to download with `Content-Disposition: attachment`.
  - **Secure Authenticated Real-Time Protocol**:
    The Next.js session never exposes bearer JWT tokens to browser JS. Instead, the browser requests a single-use, 60-second WebSocket Ticket (`POST /api/v1/auth/realtime-ticket`). The ticket is cryptographically signed and verified in Axum, bound to the specific user and tenant. Client connections are restricted strictly to subscribing to their authorized tenant and inbox scopes.
  - **SPIFFE/SPIRE for Tenant-Scoped Authentication**:
    We eliminate all pre-authenticated database routes, test-bypass flags, and default database fallbacks. Every API, webhook, and background worker derives tenant context from verified JWT claims or provider signature metadata. Cross-tenant records return identical 404/denial responses to prevent enumeration attacks.

  ---

  ## 4. Implementation Prompt (For Implementer Agent)

  ```text
  Design and implement the Core Omnichannel Chat Engine inside OHC's Rust backend (`src/server/`) and Next.js frontend (`src/ui/next/`). This system must completely replace the retired Chatwoot dependency, consolidating our database models, real-time gateways, and operator compositions.

  ### 1. Critical User Journeys (CUJs)
  - **CUJ 1: Omnichannel Webhook Ingestion & Deduplication**:
    Implement a secure, fail-closed webhook endpoint for TWILIO (SMS) and META (WhatsApp/Instagram). Perform strict raw-signature verification, prevent replay attacks, and resolve tenant identity without hardcoded fallbacks. Append the message to `omni_inbox_messages` using a transaction-locked monotonic sequence.
  - **CUJ 2: Transactional Outbox Delivery with Dead-Letter Recovery**:
    When an operator approves a draft reply or drafts a manual response, insert a delivery job to a transactional outbox. A background runner must lease the job, send it to the outbound provider using stable idempotency keys, and handle subsequent delivery/read receipts. If delivery permanently fails, move the job to a Dead-Letter State with an actionable operator retry control.
  - **CUJ 3: Authenticated Real-Time WebSocket & PowerSync Syncing**:
    Implement a Ticket-based WebSocket protocol. Add `POST /api/v1/auth/realtime-ticket` returning short-lived single-use tickets. Validate tickets on WebSocket connection and enforce real-time channel subscription filters. Set up PowerSync JWT generation with strict tenant sync filters.
  - **CUJ 4: Mobile-First Responsive Layout Settings**:
    Implement three user-selectable layout preferences: Classic Three-Pane, AI Operations Console (Default), and Focus-First Two-Pane. Ensure all three composition states adapt to a 375px mobile viewport with touch-target rich elements (>= 44x44px) and smooth sliding sheets.
  - **CUJ 5: Quarantined Media Upload and Malware Protection**:
    Implement file uploading to a quarantined storage bucket. Hold media in a pending state while scanning. After successful validation, promote the media using pre-signed temporary URLs. Force active HTML/SVG files to download with strict headers.

  ### 2. Design & Visual Hardening
  - Adopt high-end macOS Translucent Glass materials: use backdrop-blur, 65% opacity backgrounds, subtle high-contrast borders, and clear state markers.
  - No database credentials, access tokens, or raw customer message bodies may leak in logs, debuggers, or model prompts.
  - Ensure the "grandmother test" is met: hide all developer jargon (gRPC, Bazel, Postgres, WebSockets, DB schemas) behind an explicit advanced toggle.

  ### 3. Acceptance Criteria (Verification)
  - All database queries must enforce multi-tenant isolation, tested with active cross-tenant denial scenarios.
  - Prove that typing manually increments the automation fence and cancels pending AI draft background jobs before delivery.
  - E2E Playwright tests must run the full loop from browser login, receiving an inbound message, displaying it in the mobile viewport, approving an AI draft, and asserting delivery receipt states without any mocked network routes.
  - Unit test coverage for all newly added or modified Rust/TypeScript code must be 100%. Ensure `bazel test //...` (or `cargo test` if Bazel is disabled) and Vitest fully pass.
  ```

  ---

  ## 5. Scope & Priority
  - **Priority**: `P1` (High)
  - **Estimated Scope**: Large (Multi-department backend synchronization, frontend component restructuring, real-time ticket handshakes, and strict multi-tenant validation)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
