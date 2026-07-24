issue_title: "🔍 Scout: Tool Integration Research - Native Omnichannel Chat (Chatwoot Parity)"
issue_description: |
  ### Title
  Build Native Rust Omnichannel Chat Engine (Chatwoot Parity)

  ### Problem Statement
  Owners and operators (like Maya the home baker or Carlos the field service owner) receive customer inquiries across multiple fragmented channels: WhatsApp, Instagram DMs, web chat, SMS, and email. Managing these manually leads to lost leads, delayed responses, and scattered customer context. Previously, third-party integrations like Chatwoot were considered, but relying on an external SaaS for core customer communication introduces latency, data silos, and reliability issues. OHC needs a built-in, native omnichannel chat engine that unifies all customer messages into a single triage feed, enabling the AI assistant to draft replies and coordinate work instantly.

  ### Research Report
  **Chatwoot Source Benchmarking:**
  As per the OHC Engineering Standard, integrating Chatwoot as an external service is 100% RETIRED. Instead, a deep dive into the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) was conducted to evaluate its architecture.
  - **Data Model:** Chatwoot uses polymorphic channels (`app/models/channel/`). Specifically, `Channel::Whatsapp` handles WhatsApp via various providers (Twilio, Cloud API), storing `phone_number` and `provider_config`. Similarly, it supports Web Widgets, Instagram, and Email.
  - **Core Entities:** `Account` (Tenant), `Inbox`, `Conversation`, `Message`, and `Contact`.
  - **Strengths:** Excellent normalization of disparate message formats into a unified `Message` model. Robust webhook ingestion for real-time channels.
  - **Weaknesses/Limitations for OHC:** Built in Ruby on Rails, which doesn't align with OHC's high-performance native Rust microservice architecture. It also relies heavily on Sidekiq for jobs, whereas OHC uses PostgreSQL `SKIP LOCKED` queues.

  **Market Need:**
  Competitors like Tencent Workbuddy, Feishu, and Shopify Inbox provide unified messaging natively. A non-technical user (e.g., Priya the boutique operator) needs to connect her WhatsApp Business and Instagram without configuring API keys, webhooks, or external platforms. The native chat engine must handle incoming webhooks, normalize them, and push them to the Flutter frontend in real-time.

  ### Design Doc
  - **Architecture:** A native Rust microservice (`chat-engine`) within the OHC mono-repo that replicates Chatwoot's channel connector model.
  - **Tenant Isolation:** All tables (`inboxes`, `conversations`, `messages`, `contacts`) will use `tenant_id` for row-level security in PostgreSQL.
  - **Channel Connectors:** Implement trait-based channel adapters in Rust. Start with `WebWidget` (WebSocket-based) and `WhatsApp` (Cloud API webhook ingestion).
  - **Webhook Ingestion:** An API endpoint to receive incoming provider webhooks, verify signatures, and enqueue a processing job using OHC's PostgreSQL job queue.
  - **AI Integration (Work Triage):** Once a message is normalized and saved, trigger the AI Work Triage agent to categorize the intent and draft a reply in the unified owner feed.
  - **User Experience:** The owner sees a unified "Inbox" in the Flutter shell. They don't know if a message came from WhatsApp or Web Chat; they just see the customer's name, intent, and the AI's suggested reply.

  ### Implementation Prompt
  **User-Facing Outcome:** The owner can navigate to the "Inbox" tab in the OHC Flutter app and see incoming messages from a test Web Chat widget and WhatsApp. They can type a reply (or approve an AI draft) and it sends successfully back to the customer's original channel.

  **Acceptance Criteria:**
  1. Define the database schema for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with `tenant_id` RLS.
  2. Implement a native Rust backend service to handle unified messaging logic.
  3. Implement the `Web Widget` channel adapter (WebSocket support).
  4. Implement the `WhatsApp` channel adapter (Meta Cloud API webhooks).
  5. The Flutter frontend must display a unified message feed matching the OHC Premium Token design system.
  6. E2E tests must verify a message sent from a mock WhatsApp webhook appears in the UI, and a reply from the UI triggers an outgoing HTTP request to the provider.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
