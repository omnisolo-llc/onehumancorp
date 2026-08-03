issue_title: "Implement Native Rust Omnichannel Chat: Twilio SMS & WhatsApp Connectors"
issue_description: |
  ### Mission Queue Protocol Brief

  **Title**: Implement Native Rust Omnichannel Chat: Twilio SMS & WhatsApp Connectors

  **Problem Statement**:
  Our owner/operator personas (like Maya, Carlos, and Priya) rely heavily on WhatsApp and SMS to communicate with customers. They need a centralized inbox (Work Triage) that natively captures these messages without requiring external third-party helpdesk software. Chatwoot has been retired as an external integration; OHC must implement its own high-performance, multi-tenant omnichannel chat engine natively in Rust.

  **Research Report (Chatwoot Benchmarking)**:
  Based on an audit of Chatwoot's source code (`https://github.com/chatwoot/chatwoot`), specifically `app/models/channel/twilio_sms.rb` and `app/models/channel/whatsapp.rb`, we found the following key architectural requirements:
  - **Twilio SMS/WhatsApp Connector**: Requires storing `account_sid`, `auth_token`, `messaging_service_sid`, and `phone_number`. It uses Twilio's API to send messages and sets up webhooks to receive incoming SMS/WhatsApp messages.
  - **WhatsApp Cloud API Connector**: Requires storing `business_management_token`, `phone_number`, and `message_templates`. It must handle periodic syncing of WhatsApp message templates and track `phone_number_health`.
  - **Tenant Isolation**: In Chatwoot, channels are tied to an `account_id`. In OHC, this maps to our PostgreSQL row-level security using `tenant_id`.

  For non-technical owners, this means they can simply connect their Twilio account or WhatsApp Business number once, and all messages seamlessly appear in their OHC Work Triage feed. The AI Assistant can then draft replies directly.

  **Design Doc**:
  - **Data Model**: Introduce new PostgreSQL tables for channel configurations, such as `channel_twilio_sms` and `channel_whatsapp`, secured by `tenant_id` RLS.
  - **API Layer (Rust)**: Implement a Rust-based webhook handler service that receives incoming payload from Twilio and WhatsApp Cloud API, validates the HMAC signatures, and enqueues processing jobs via PostgreSQL `SKIP LOCKED`.
  - **Worker Layer**: Background workers will dequeue messages, link them to existing Customer profiles (or create new leads), and insert them into the unified Work Triage inbox.
  - **Outgoing Messages**: Implement a native Rust service that uses Twilio SDK/REST API and WhatsApp Cloud API to send replies drafted by the AI or sent by the owner.
  - **UI/UX**: Provide a simple "Connect Channel" screen for owners. Hide the technical webhook setup; OHC should automatically configure webhooks on the provider side using the provided API keys.

  **Implementation Prompt**:
  - Create the necessary database schemas for Twilio and WhatsApp channel configurations with tenant isolation.
  - Implement a Rust webhook receiver for Twilio and WhatsApp that parses incoming messages and routes them to the unified inbox.
  - Build a secure API for sending messages out through Twilio and WhatsApp.
  - Develop the frontend UI in Flutter for owners to connect their Twilio or WhatsApp Business accounts with clear, non-technical instructions.
  - Ensure all features are fully tested, including E2E Playwright tests simulating incoming messages and agent replies.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []