issue_title: "Twilio for WhatsApp Integration Analysis"
issue_description: |
  **Title**: Deep-Dive: Twilio WhatsApp Integration for Omnichannel Inbox Support

  **Problem Statement**: Solo owners operating heavily on WhatsApp need a robust, centralized omnichannel inbox. While our codebase currently features a basic Twilio stub (`src/server/integrations/twilio/client.rs`) for sending SMS/WhatsApp, it lacks comprehensive support for modern business capabilities (e.g., WhatsApp Cloud API, Meta OAuth, or Twilio Conversations API) to provide reliable, bidirectional messaging. Without a centralized hub, owners miss inquiries and cannot manage scale effectively.

  **Research Report**:
  - **Tool Choice**: Twilio WhatsApp (or Twilio Conversations) API vs. WhatsApp Cloud API direct.
  - **Findings**: The current codebase only implements a rudimentary outbound webhook for Twilio (and mocks it). The market standard for non-technical users looking for unified inboxes is Twilio Conversations API or Meta's WhatsApp Cloud API.
  - **Competitive Analysis**: High-value tools in this space (e.g., Manychat, Twilio Flex) are popular among small businesses.
  - **Pricing/Viability**: Twilio charges per message segment. Offering an OHC-funded key for these could drain resources quickly, so BYOK (Bring Your Own Key) or customer-funded models are strictly required per the OHC contract.
  - **OAuth & Webhooks**: Twilio utilizes traditional auth tokens, whereas Meta uses OAuth.

  **Design Doc**:
  - The integration would present an intuitive "Connect Twilio for WhatsApp" modal in the Integrations UI.
  - Using the BYOK model, users will input their Account SID and Auth Token.
  - A secure webhook endpoint (`/api/v1/webhooks/twilio/whatsapp`) will consume incoming messages, route them to the tenant's unified inbox via `omnisolo.memory`, and trigger the agent matrix (e.g., triage department) using standard JSON-RPC.

  **Implementation Prompt**:
  - Implement a complete BYOK credential vault flow for Twilio.
  - Implement an authenticated incoming webhook that standardizes WhatsApp messages into the unified inbox format.
  - Do not create mock functionality. Hook the integration strictly to the user's provided API key and securely route incoming webhooks to the unified inbox.

  **Priority**: P2
  **Estimated Scope**: Medium

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
