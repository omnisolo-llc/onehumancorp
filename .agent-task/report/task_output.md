issue_title: "Implement Native Rust Chatwoot Omnichannel Features"
issue_description: |
  # Native Rust Chatwoot Replacements: Omnichannel Work Triage

  ## Problem Statement
  OHC is currently migrating away from using Chatwoot as an external, third-party integration for omnichannel messaging (WhatsApp, Web Chat, Email, SMS, Instagram). We are establishing our own native Rust multi-tenant omnichannel chat system inside `onehumancorp/mono`. We need a foundational system that supports managing multiple communication channels, handling webhooks from external providers (like Meta/WhatsApp), standardizing incoming messages into a unified "Work Triage" feed, and drafting replies seamlessly via AI or human operators.

  ## Research Report
  Based on an audit of the Chatwoot source code (`app/models/channel/whatsapp.rb`, `app/services/whatsapp/webhook_setup_service.rb`, etc.), Chatwoot handles WhatsApp by persisting channel config (phone number, WABA ID, access tokens) and managing webhooks (registering callbacks to `FRONTEND_URL/webhooks/whatsapp/:phone_number`). It also heavily uses Meta's Graph API to subscribe to `messages` and `calls` fields and polls health endpoints.

  Other competitors (Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark) provide a unified inbox where the underlying provider (WeChat, SMS, etc.) is abstracted. OHC's persona (Maya, Carlos) needs to see a unified feed of messages without worrying about the underlying API configurations.

  ## Design Doc
  We will build a native Rust multi-tenant omnichannel engine.

  - **Data Model:** We need a flexible `Channel` representation in Rust (PostgreSQL backed via Diesel or SQLx, using the `tenant_id` pattern) that stores credentials and provider-specific config (like WABA ID, phone number, access token).
  - **Webhook Ingestion:** A high-throughput Rust HTTP endpoint to receive webhooks from Meta (WhatsApp Cloud API) and other providers. It will authenticate the payload (verify token, signature) and normalize it into an internal `IncomingMessage` event.
  - **Message Processing:** The internal event is pushed to a job queue (PostgreSQL `SKIP LOCKED` or Redis based) where background workers process it, associate it with the correct tenant and customer, and push it to the unified Work Triage feed.
  - **Work Triage Interface:** The Flutter frontend will present a unified inbox.

  ## Implementation Prompt
  1. Define the Rust structs and PostgreSQL schemas (with RLS for `tenant_id`) for `Channel` (specifically WhatsApp configuration initially), `Contact`, and `Message`.
  2. Implement the Meta WhatsApp webhook verification and ingestion endpoint in the Rust backend.
  3. Implement a background worker to process raw webhook payloads and normalize them into unified `Message` records.
  4. Ensure the system is observable (OpenTelemetry) and handles rate limits/retries.

  ## Priority
  P0 (critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
