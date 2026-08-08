issue_title: "Native Rust Omnichannel Chat System: WhatsApp Cloud API Integration"
issue_description: |
  ## Mission Overview
  **Role:** Principal Integrations Engineer (L7)
  **Domain:** Native Rust Omnichannel Chat System (Replacing external Chatwoot)
  **Outcome:** Implement native WhatsApp Cloud API support in our internal Rust `chat` microservice.

  ### Problem Statement
  Small-business owners (like Maya the home baker or Carlos the field service owner) rely heavily on WhatsApp to communicate with their customers. Currently, our system relies on retiring third-party solutions (Chatwoot). We need to build a native Rust multi-tenant omnichannel chat system that acts as the core communication engine for OHC. A critical first step is integrating the WhatsApp Cloud API directly into our platform, allowing owners to receive customer inquiries, draft AI-assisted replies, and handle operations (like capturing order deposits) directly within the OHC Assistant shell.

  ### Research Report
  - **Competitor Landscape**: Tools like Tencent Workbuddy, WeCom, and localized CRM systems all offer deeply integrated WhatsApp/WeChat functionalities. Chatwoot, which we are retiring, handled this via a `Channel::Whatsapp` model that stored business management tokens, message templates, phone numbers, and health statuses.
  - **Tool Deep-Dive (WhatsApp Cloud API)**:
      - **User-First Value Mapping**: Maya can receive custom cake inquiries from her Instagram/WhatsApp directly into the OHC feed. The AI Work Triage can classify it, draft a reply, and notify her. All context is kept in one place.
      - **Capabilities**: Meta provides the WhatsApp Cloud API for sending and receiving messages, managing templates, and handling rich media. It relies on webhooks for incoming messages and HTTP APIs for outgoing ones.
      - **SaaS Viability**: WhatsApp Cloud API is viable for multi-tenant (Cloud) setups where each tenant (owner) configures their own WhatsApp Business Account (WABA) credentials.
  - **Chatwoot Source Benchmarking**:
      - Chatwoot's `channel_whatsapp` schema: `phone_number`, `provider_config`, `business_management_token`, `message_templates`, `phone_number_health`.
      - Chatwoot's `inboxes` schema: `channel_id`, `channel_type`, `account_id`, `name`, `enable_auto_assignment`.
      - We need to replicate this natively in Rust, using `tenant_id` for row-level security.

  ### Design Doc
  - **Integration Architecture**:
      - Build a new Rust crate/module (e.g., `ohc-chat` or `ohc-integrations-whatsapp`).
      - Define gRPC/REST endpoints for tenants to link their WhatsApp Business Accounts.
      - Implement a robust Webhook receiver to handle incoming WhatsApp messages from Meta. This receiver must validate Meta's payload signature (SHA256).
      - Store WhatsApp channel configurations in PostgreSQL (e.g., `whatsapp_channels` table linked to a unified `inboxes` table), utilizing RLS with `tenant_id`.
      - When a message is received, enqueue it via the AI Job Queue (PostgreSQL `SKIP LOCKED`) for processing by the `Work Triage` and `Customer & Relationship` assistants.
      - Implement a sender service that uses Meta's HTTP API to send outbound messages and template messages.
  - **User Experience**:
      - **Setup**: The owner navigates to "Channels" and follows a simple guided flow to connect their WhatsApp Business Number.
      - **Daily Use**: WhatsApp messages appear in the unified Work Triage feed. The owner sees AI-drafted replies and can click "Send" or edit them. They do not need to know it's a WhatsApp message unless relevant.

  ### Implementation Prompt
  - Create the PostgreSQL migrations for `whatsapp_channels` (tenant_id, phone_number, provider_config, auth_token, health_status) and integrate it with a unified `inboxes` table if it doesn't exist yet.
  - Implement a Rust Axum/Tonic service that exposes a webhook endpoint for Meta. It must verify the signature and parse incoming text and media messages.
  - Implement an outbound message client in Rust that sends messages back to the customer via the WhatsApp Cloud API.
  - Ensure all database queries enforce Row-Level Security (`tenant_id`).
  - Add comprehensive unit tests for the webhook verification and message parsing. Add E2E Playwright tests simulating an owner connecting their WhatsApp and receiving a message in the UI (using a local mock/adapter for the Meta API).

  ### Priority & Scope
  - **Priority**: P0 (Critical path for replacing Chatwoot)
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
