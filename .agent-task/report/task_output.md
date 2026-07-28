issue_title: "Native Rust Omnichannel Chat: WhatsApp Provider Integration"
issue_description: |
  ## Title
  Native Rust Omnichannel Chat: WhatsApp Provider Integration

  ## Problem Statement
  Our owner/operator personas (like Maya, Carlos, and Priya) rely heavily on WhatsApp to communicate with their customers, receive orders, and answer queries. Currently, managing these conversations requires constantly switching between their personal/business WhatsApp app and the OHC assistant. They lack a unified inbox where the OHC assistant can automatically triage messages, draft replies, and track orders directly from WhatsApp. We need a native Rust WhatsApp channel connector in OHC to replace the retired external the legacy chat provider dependency, allowing owners to manage WhatsApp conversations directly inside their OHC workspace.

  ## Research Report
  - **Tool Evaluated**: WhatsApp Business API (specifically looking at the legacy chat provider's implementation pattern in `app/models/channel/whatsapp.rb`).
  - **Competitor/Ecosystem Context**: Platforms like Tencent Workbuddy, WeCom, and the legacy chat provider provide native WhatsApp integrations. the legacy chat provider models this as a `channel_whatsapp` with a `phone_number`, `provider` (e.g., Twilio, Cloud API), and `provider_config` (API keys, webhook secrets).
  - **Value to Non-Technical Owner**: Owners can link their WhatsApp Business number to OHC. Once linked, any message sent by a customer to their WhatsApp appears in the OHC Work Triage feed. The AI Assistant can instantly draft replies (e.g., pricing, scheduling) for the owner to approve, saving hours of manual typing and preventing lost leads.
  - **SaaS Viability**: WhatsApp Cloud API is widely available and charges per conversation. OHC can build a multi-tenant webhook handler to receive incoming messages, and route them to the appropriate tenant's workspace using the receiving phone number.

  ## Design Doc
  - **Trigger/Input**: A customer sends a message to the owner's connected WhatsApp Business number. A webhook payload is sent from WhatsApp/Twilio to OHC's new multi-tenant webhook receiver.
  - **Action**: The native Rust service authenticates the webhook, looks up the `tenant_id` associated with the destination phone number, and creates a unified `Conversation` and `Message` record in OHC.
  - **User Experience**: The message immediately appears in the owner's "Work Intake" feed. The OHC Assistant runs in the background, analyzing the message, and prepares a suggested reply. The owner sees the message and the drafted reply, and can click "Send" to push the response back to WhatsApp via the API.
  - **Configuration**: A simple settings page where the owner clicks "Connect WhatsApp", authenticates via Facebook/Meta OAuth, and selects their business number.

  ## Implementation Prompt
  Implement a native Rust channel adapter for WhatsApp Business within the new OHC omnichannel chat system. The solution should expose a webhook endpoint that receives incoming WhatsApp messages, identifies the correct tenant workspace, and stores the message in the unified inbox. It must also provide a mechanism to send messages back to the WhatsApp API. Ensure that the configuration is intuitive for a non-technical owner, focusing on a one-click connection flow rather than asking them to manually copy-paste API tokens and webhook URLs. Verify the end-to-end flow with Playwright tests that simulate an incoming webhook and verify the message appears in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
