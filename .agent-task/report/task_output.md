issue_title: "Build Native WhatsApp Cloud API Connector (Chatwoot Replacement)"
issue_description: |
  **Title**: Build Native WhatsApp Cloud API Connector (Chatwoot Replacement)

  **Problem Statement**:
  Small business owners like Maya (Home Baker) and Carlos (Field Service) live on WhatsApp. Currently, OHC relies on external systems or lacks a deeply integrated omnichannel chat for WhatsApp. As part of our Chatwoot retirement mandate, we need to bring WhatsApp Business messaging natively into OHC's backend so owners can triage customer inquiries, draft AI-assisted replies, and handle custom order negotiations directly from their OHC work feed without relying on a separate third-party chat tool like Chatwoot.

  **Research Report**:
  - **Tool Evaluated**: Chatwoot's WhatsApp Cloud API implementation (from `https://github.com/chatwoot/chatwoot`).
  - **Findings**: Chatwoot supports WhatsApp via multiple providers (WhatsApp Cloud, 360dialog, Twilio). The WhatsApp Cloud API is the standard and direct way to connect via Meta. It requires an API token, Business Account ID, and Phone Number ID.
  - **Chatwoot Benchmarking**: Chatwoot's implementation (`Whatsapp::Providers::WhatsappCloudService` and `WebhookSetupService`) handles standard text messages, attachments (images, audio, video, documents), interactive messages (buttons, lists), and templates (for starting conversations after the 24-hour window). It also uses webhooks to receive incoming messages.
  - **Value to Owner**: A native implementation means OHC can intercept WhatsApp messages, run them through the AI Job Queue for auto-drafting replies or parsing orders, and present them in the unified Work Triage feed. The owner just authenticates once and OHC handles the rest natively.

  **Design Doc**:
  - **Triggers**:
    - Inbound: Webhook endpoint receives payloads from Meta (WhatsApp Cloud API) when a customer sends a message.
    - Outbound: Owner or AI Assistant sends a reply via the OHC UI/API.
  - **Integration Flow**:
    - **Configuration**: Owner connects WhatsApp Business account in OHC Settings, which saves the API Key, Phone Number ID, and Business Account ID securely per tenant.
    - **Inbound Message Flow**: The backend webhook handler receives the Meta payload. It extracts the sender phone number, message text/media, and maps it to an OHC Customer/Lead. It publishes an event to the unified Work Triage feed.
    - **Outbound Message Flow**: When responding, the backend formats the message (handling templates if outside the 24-hour window) and calls the Meta Graph API `/{phone_id}/messages` endpoint.
    - **AI Assistance**: The Customer Assistant AI can pre-process inbound webhook payloads to draft replies and suggest actions (e.g., create a quote).

  **Implementation Prompt**:
  - Implement a native WhatsApp Cloud API connector in the OHC backend to achieve parity with Chatwoot's WhatsApp integration.
  - Create a webhook endpoint that can receive and verify Meta's webhook challenges and parse inbound text and media messages.
  - Create an outbound message sender that can send text messages, attachments, and interactive buttons using the Meta Graph API.
  - Integrate these flows into the multi-tenant architecture, ensuring credentials (API keys, IDs) are securely stored and isolated per tenant.
  - Expose the inbound messages to the Work Triage system so they appear in the owner's unified feed.
  - Support the 24-hour customer service window logic, identifying when a template message is required to reach out to a customer.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
