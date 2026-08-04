issue_title: "Native WhatsApp Cloud API Omnichannel Integration"
issue_description: |
  # Research Report: WhatsApp Cloud API Integration

  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on messaging apps to take orders and answer customer inquiries. Currently, these messages exist outside of OHC, meaning the Work Triage system cannot see them, and the Customer & Relationship Assistant cannot draft replies or update customer preferences. Managing DMs separately slows down response times, leads to missed orders, and forces the owner to manually copy information between apps. Non-technical owners need WhatsApp to function as a seamless part of their OHC assistant feed, without having to manage tokens or webhooks themselves.

  ## Research Findings
  Our research into the legacy omnichannel system revealed that a robust integration for WhatsApp requires specific data models and channel integrations. WhatsApp is critical in many regions, and Meta provides the WhatsApp Cloud API which allows businesses to interact via automated channels.

  - Meta Business setup uses an Embedded Signup flow (OAuth) to let owners connect their WhatsApp number easily.
  - Integration uses Webhooks to deliver incoming messages (text, media, location).
  - It supports replying with rich media, interactive messages, and automated AI drafts.
  - The API is reliable and scales to small business needs.

  ## Architectural Design
  The integration will align with OHC's native omnichannel chat schema in Rust (recently introduced in `1009_native_omnichannel_chat.sql`).

  **Data Model Map:**
  - `ChatChannel`: A new channel type `whatsapp_cloud` will be added. The `config` JSONB field will store Meta credentials like `phone_number`, `business_management_token` (encrypted if possible/needed), `provider_config` (API keys, account ID), and `webhook_verify_token`.

  **System Components:**
  - **Meta OAuth/Embedded Signup Controller:** A new endpoint under `src/server/api/chat` to handle the OAuth flow and acquire tokens for the `whatsapp_cloud` channel.
  - **Webhook Handler:** An endpoint (e.g. `/api/webhooks/whatsapp`) to receive incoming messages from Meta. It will verify the payload using the `webhook_verify_token` stored in the `ChatChannel` config.
  - **Message Processing Pipeline:** Incoming Webhooks will be parsed and mapped to `ChatContact`, `ChatConversation`, and `ChatMessage` entities. The existing `ChatService` will be expanded to handle these incoming flows and trigger the Work Triage AI agent.
  - **Outbound Message Service:** Extend `ChatService` or create a new `WhatsAppService` to send messages using the Meta Cloud API. This service will be used by the AI agent to draft and send replies.

  **Mobile UX Flow:**
  - In the Assistant Feed, WhatsApp messages will appear just like any other work item.
  - The owner can review AI-drafted replies and tap "Approve & Send".
  - A settings page will allow the owner to "Connect WhatsApp" using a simple Meta login button, hiding the technical complexities (tokens, webhooks).

  ## Implementation Prompt
  Implement the WhatsApp Cloud API integration for the native Rust omnichannel chat system.
  1. Add an OAuth endpoint to handle Meta's Embedded Signup for WhatsApp.
  2. Implement a webhook receiver endpoint that validates and parses incoming WhatsApp messages.
  3. Ensure incoming messages create/update `ChatContact`, `ChatConversation`, and `ChatMessage` records correctly.
  4. Implement an outbound message sender that uses the Meta API to send replies.
  5. Ensure all database operations adhere to the strict row-level security (RLS) tenant isolation policies.
  6. Add E2E tests covering the webhook reception and message sending flows (mocking external Meta calls via the test adapter pattern if needed).

  Acceptance Criteria: A non-technical owner can connect their WhatsApp business account, receive incoming messages in their OHC triage feed, and have the AI agent successfully draft and send outbound replies.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
