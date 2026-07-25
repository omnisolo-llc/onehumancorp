issue_title: "Build Native Rust Omnichannel Chat System Parity with Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp relies heavily on external communication channels. As per the new product mandate, we are 100% retiring Chatwoot as an external third-party integration. OHC must build its own multi-tenant omnichannel customer support and chat engine natively in Rust inside `onehumancorp/mono`. We need feature parity with Chatwoot's API and core features. We will start with a native WhatsApp Cloud API integration using Rust, translating the logic and capabilities that Chatwoot's `WhatsappCloudService` and `IncomingMessageBaseService` handle.

  This issue is to implement full omnichannel support directly in the OHC platform, starting with robust native WhatsApp integration capabilities (Cloud API).

  ## Research Report
  Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) reveals how they handle WhatsApp integration:
  - **Provider Architecture**: They use a base service `Whatsapp::Providers::BaseService` and a cloud service `Whatsapp::Providers::WhatsappCloudService` which communicates directly with `graph.facebook.com`.
  - **Webhook Subscriptions**: `Whatsapp::WebhookSetupService` sets up webhooks and subscribes to `messages`, `smb_message_echoes`, and `calls` (if voice enabled).
  - **Message Types**: Supports Text, Attachments (image, audio, video, document), Interactive (buttons, lists).
  - **Message Ingestion**: `Whatsapp::IncomingMessageBaseService` handles parsing webhook payloads from Meta. It checks for statuses (sent/delivered/read) and incoming messages. It has specific atomic locks (Redis SET NX) to prevent double-processing.
  - **Error Handling**: Captures Meta API errors (e.g. error 131060) and marks messages as `failed` with an `external_error`.

  OHC has partial stubs for `whatsapp` and `whatsapp_cloud` in `src/server/integrations/whatsapp` and `src/server/integrations/whatsapp_cloud`, but they lack the robustness of Chatwoot's feature set. The current Rust implementation is missing:
  1. Full Webhook setup logic (Registering Phone Numbers, PINs, Verification).
  2. Complete Incoming Message Handler (parsing attachments, statuses, location, contacts).
  3. Support for Interactive messages (buttons, lists) in the `WhatsAppClient`.
  4. Template synchronization and sending.

  ## Design Doc
  We will enhance the native Rust implementation of the WhatsApp Cloud integration to achieve feature parity with Chatwoot's WhatsApp support:
  1. **Webhook Handler**: Enhance `src/server/integrations/whatsapp/webhook.rs` to process all message types (text, interactive, media) and statuses. Map Meta's webhook format to internal OHC message structures.
  2. **WhatsApp Client**: Enhance `src/server/integrations/whatsapp/client.rs` (or `whatsapp_cloud`) to send Template messages, Interactive messages (Lists, Buttons), and Media messages.
  3. **Webhook Setup Service**: Implement a setup flow that registers the phone number via Meta API and subscribes to webhooks.
  4. **Idempotency**: Use Redis or a database lock (similar to Chatwoot's `lock_message_source_id!`) to prevent processing the same webhook event twice.

  ## Implementation Prompt
  - Ensure the OHC native WhatsApp Cloud implementation handles all WhatsApp message types (Text, Image, Video, Document, Location, Interactive).
  - Implement full parsing of incoming Meta webhooks, including handling message echoes and delivery/read statuses.
  - Make sure the system registers webhooks with the Meta API securely.
  - You must not use any external Chatwoot service; all omnichannel logic must be native Rust code in `src/server/integrations/whatsapp_cloud` and `whatsapp`.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
