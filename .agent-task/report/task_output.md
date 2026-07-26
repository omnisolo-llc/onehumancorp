issue_title: "Implement WhatsApp Business API integration"
issue_description: |
  **Mission:** Expand OHC's capabilities by integrating WhatsApp Business API natively to avoid using third-party Chatwoot services, achieving full feature parity via Rust implementations.

  **Problem Statement:**
  Small-business owners like Maya and Carlos often communicate with customers through WhatsApp. Manually tracking orders, requests, and follow-ups via personal or basic business WhatsApp is messy. They need OHC to triage WhatsApp messages natively alongside their other operations.

  **Research Report:**
  Based on competitors (like WeCom, Tencent Workbuddy) and community forums, WhatsApp integration is critical, especially in LATAM and India, but also for small businesses globally. The official Meta WhatsApp Business API allows businesses to send and receive messages, use templates, and integrate seamlessly with third-party platforms.
  Following the Chatwoot retirement mandate, we investigated Chatwoot's approach in `app/models/channel/whatsapp.rb`, `WhatsappCloudService`, and `WebhookSetupService`. Chatwoot manages a WhatsApp channel utilizing `phone_number_id`, `business_account_id`, and `api_key` for Meta WhatsApp Cloud API credentials. It handles text, attachment, interactive messages, and templates, while using Webhooks for receiving inbound messages and maintaining a sync of WhatsApp templates. We should build a native Rust multi-tenant WhatsApp connector inspired by these features.

  **Design Doc:**
  - Create a new Rust crate/service within `onehumancorp/mono` for the Omnichannel Chat System, starting with a WhatsApp channel connector.
  - Implement data models representing a WhatsApp channel containing `phone_number_id`, `business_account_id`, `api_key`, `webhook_verify_token`, etc.
  - Implement a Webhook endpoint that receives incoming messages from Meta's API and handles `hub.challenge` verification.
  - Route incoming messages to the appropriate `tenant_id` and create/update conversations in the database.
  - Implement an outgoing message queue that uses Meta's API to reply to customers (handling text, attachments, templates, interactive messages).
  - Include functionality to sync WhatsApp message templates.
  - OHC's Work Triage agent will then process these conversations.

  **Implementation Prompt:**
  - Build a native Rust WhatsApp Business API connector directly within OHC.
  - Users should be able to connect their WhatsApp Business account via OAuth or manual setup (API Key, Phone ID, Business Account ID) in the OHC UI.
  - Incoming WhatsApp messages should appear in the OHC "Work Intake" feed.
  - The owner should be able to reply directly from OHC (handling texts and media).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
