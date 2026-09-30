issue_title: "Integration Research: Twilio WhatsApp API"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Integration Research: Twilio WhatsApp API

  **Problem Statement**: Solo business owners (like web/design/marketing services) struggle to capture inquiries and support customers promptly across channels. Missing a message means losing revenue. WhatsApp is highly requested, but managing it manually is error-prone. OHC needs an automated way to manage WhatsApp conversations within the unified inbox.

  **Research Report**:
  - Evaluated Twilio's WhatsApp Business API.
  - Twilio acts as a robust abstraction over Meta's WhatsApp API, offering unified SMS and WhatsApp messaging, which fits OHC's omnichannel needs.
  - **Pricing**: Twilio charges a flat $0.005 message handling fee per message. In addition, Meta charges conversation-based pricing (utility/auth/marketing templates vs free-form). Utility and authentication start at ~$0.0034/message. During a 24-hour customer service window initiated by the customer, Meta waives its fees for free-form and utility templates, meaning OHC owners only pay Twilio's $0.005/msg. This is highly cost-effective for support and intake.
  - **Usability for non-technical users**: Owners will simply connect their Twilio/WhatsApp credentials in OHC. They won't see API keys or webhooks. The AI team manages the conversation within the 24-hour window automatically.
  - **Limits/OAuth**: Twilio's API requires strict adherence to Meta's 24-hour service window for free-form replies; outside of that, only approved templates can be used. Webhooks must be idempotent as Twilio/Meta can send duplicate events.

  **Design Doc**:
  - **Trigger**: Customer sends a WhatsApp message to the owner's configured business number.
  - **Action**: Twilio webhook notifies OHC. OHC routes the message into the unified inbox. The AI Sales/Support agent drafts and sends a contextual reply via Twilio's API, keeping within the 24-hour free-form window.
  - **User Experience**: The owner sees the conversation in the OHC feed, alongside emails and webchat. No separate WhatsApp app needed.

  **Implementation Prompt**:
  Implement a Twilio WhatsApp webhook receiver and sending capability within OHC's unified inbox. Ensure the AI agent can receive messages, understand the conversation context, and reply. The system must enforce the 24-hour customer service window constraint, blocking or warning the AI from sending free-form messages if the window has expired.

  **Priority**: P1 (high)
  **Estimated Scope**: Medium

  *(Superpowers Provenance: Loaded brainstorming and using-superpowers skills from https://github.com/obra/superpowers.git at revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)*
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
