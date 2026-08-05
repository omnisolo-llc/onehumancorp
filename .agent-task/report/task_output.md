issue_title: "Native Rust WhatsApp Cloud API Integration for Work Triage"
issue_description: |
  **Title**: Native Rust WhatsApp Cloud API Integration for Work Triage

  **Problem Statement**:
  Our owner/operator personas (specifically Carlos the Field Service Owner and Fatima the Food Cart Operator) heavily rely on direct messaging to capture demand. Traditional CRM or helpdesk apps are too complicated. They need WhatsApp messages—where their customers actually are—to seamlessly flow into the OHC assistant as prioritized tasks, pre-orders, or service requests. Previously, one might rely on third-party tools like Chatwoot for this, but as per our new standard, we need to build this omnichannel chat capability natively in Rust to ensure speed, offline tolerance, and multi-tenant row-level security.

  **Research Report**:
  - **Competitor Benchmark (Chatwoot Source Code Audit)**: We audited the Chatwoot Ruby on Rails source code (specifically `app/models/channel/whatsapp.rb` and `app/services/whatsapp/providers/whatsapp_cloud_service.rb`). Chatwoot uses Meta's WhatsApp Cloud API for multi-tenant WhatsApp integration. It stores `business_management_token` and `provider_config` (API keys, phone number ID, business account ID) per channel. It syncs templates, handles Webhooks for incoming messages, and manages text, interactive (buttons/lists), and media (attachments) messages.
  - **Market Need**: Tools like WhatsApp Business API have high friction for small business owners due to Meta's complex setup (embedded signup, webhooks verification). OHC can abstract this behind a simple "Connect WhatsApp" button, providing a natively integrated, single unified Work Triage feed.
  - **SaaS Viability**: Meta provides the WhatsApp Cloud API with a free tier for the first 1,000 service-category conversations per month, which perfectly fits our standalone and small-tier cloud operators. Native Rust integration ensures we avoid the latency and cost of a middleman SaaS.

  **Design Doc**:
  - **Trigger/Source**: Owner navigates to "Channels" in OHC and clicks "Connect WhatsApp". They complete Meta's Embedded Signup flow, which returns a `business_management_token`, `phone_number_id`, and `waba_id`.
  - **Data Model (Native Rust)**:
    - Introduce a `whatsapp_channels` table in PostgreSQL mapped to the `tenant_id` for row-level security.
    - Fields: `tenant_id`, `phone_number`, `phone_number_id`, `business_account_id`, `api_token` (encrypted), `calling_enabled`, `webhook_verify_token`.
  - **Webhook Ingestion**: A dedicated Rust Axum/Actix webhook endpoint receives Meta's inbound messages. It verifies the payload signature using the webhook verify token.
  - **AI Work Triage**: Inbound messages are dispatched via the PostgreSQL AI Job Queue (SKIP LOCKED). The Work Triage capability (Gemini/GPT-4o) evaluates the message to auto-draft a reply, create a task, or log a pre-order in the owner's feed.
  - **User Experience**: The owner sees incoming WhatsApp requests in their unified assistant feed on their 375px mobile screen. They can tap to approve the AI's drafted response, which sends the message back out via the WhatsApp Cloud API.

  **Implementation Prompt**:
  Implement the backend Rust services and database schema to connect to the WhatsApp Cloud API, heavily inspired by Chatwoot's capabilities but customized for OHC's architecture.
  Acceptance Criteria:
  1. A new PostgreSQL table for WhatsApp channel configuration with `tenant_id` RLS.
  2. A webhook endpoint in Rust that can complete Meta's challenge-response verification and accept inbound text/media messages.
  3. A sending service that formats outgoing text and interactive (button) messages and POSTs them to Meta's `/{version}/{phone_number_id}/messages` endpoint.
  4. Integration with the AI Job Queue so incoming WhatsApp messages appear in the owner's Work Triage feed.
  5. Ensure zero external dependencies on Chatwoot; all implementation must be pure native Rust inside OHC.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
