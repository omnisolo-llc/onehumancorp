issue_title: "Tool Integration Research - Twilio WhatsApp Business"
issue_description: |
  # Mission Queue Protocol: Twilio WhatsApp Business Evaluation

  ## Title
  Twilio WhatsApp Business Integration for OmniSolo Channels

  ## Problem Statement
  Owners need a direct, native channel for automated follow-up and customer outreach outside of traditional email. Email open rates are dropping, and small business owners (especially in retail, service, and B2C segments) increasingly depend on WhatsApp to securely manage customer relationships and confirm appointments/services. OmniSolo needs a scalable WhatsApp Business API integration so that agents can interact with customers on a more immediate, high-engagement channel.

  ## Research Report
  - **Tool Evaluated**: Twilio API for WhatsApp Business.
  - **Usability for Non-Technical Owners**: High. Owners only need to provide their Twilio Auth Token and SID in the OmniSolo dashboard; the rest is handled seamlessly by OmniSolo's channel dispatch logic.
  - **Pricing Model**: Twilio charges per conversation (business-initiated vs user-initiated), with standard rates applying. The integration supports a "bring your own key" (BYOK) model where the owner pays Twilio directly for actual usage, avoiding middle-man markup.
  - **Capabilities**: Enables rich media (images, PDFs for invoices), structured templates, and reliable delivery receipts. Robust webhooks ensure the OmniSolo memory system correctly tracks customer responses to automated agent outreach.
  - **Technical Footprint**: Integrates with OmniSolo's native worker matrix and existing webhook ingestion endpoints.

  ## Design Doc
  - **Connection**: Extend the existing `ToolIntegrationsApiState` and `connection_vault` to securely accept and encrypt Twilio `account_sid` and `auth_token`.
  - **Channel Registration**: Register "twilio_whatsapp" as a supported provider, turning the current `usable: false` and `501 Not Implemented` stubs into active status paths.
  - **Message Dispatching**: Agent actions attempting to send an SMS/WhatsApp message will route through a standard channel interface which formats the payload into Twilio's required JSON and issues the HTTPS POST request.
  - **Webhook Processing**: A dedicated endpoint will receive inbound messages from Twilio, map them to the correct owner/tenant based on the webhook path or query param, and push them into the orchestrator memory queue for agent processing.

  ## Implementation Prompt
  Implement the Twilio WhatsApp Business connector. Add the required provider verification to `src/server/api/tool_integrations.rs` so that connecting a Twilio account validates the credentials via a test API call (e.g., retrieving account status). Update the relevant frontend components in `src/ui/next` to display a functional "Connect Twilio WhatsApp" configuration modal, handling the SID and Auth Token fields securely. Finally, provide integration tests demonstrating that a test payload can successfully trigger the Twilio dispatch path.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
