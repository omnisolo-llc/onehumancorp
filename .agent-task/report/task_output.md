issue_title: "Integrate WhatsApp Cloud API for Small Business Owner Interactions"
issue_description: |
  # Research Report: WhatsApp Integration for Small Business Owners

  ## Problem Statement
  Small business owners and operators (like Maya, the home baker, and Carlos, the field service owner) communicate heavily with their clients over WhatsApp. Currently, they have to manually manage these interactions, leading to dropped leads, missed follow-ups, and a fragmented view of the customer relationship in OHC. They need a way for OHC to automatically capture WhatsApp conversations, draft replies using their business context, and turn WhatsApp requests into actionable tasks or bookings within the OHC ecosystem.

  ## Research Report
  - **Tool Name**: WhatsApp Cloud API (Meta)
  - **Relevance**: WhatsApp is the dominant communication channel for small businesses globally, particularly in emerging markets and service-oriented sectors. Many competitors (e.g., Zendesk, HubSpot, and regional CRM tools) offer native WhatsApp integration.
  - **Capabilities**: The Cloud API allows for sending and receiving messages, managing templates for notifications (e.g., booking confirmations, payment links), and integrating bots for initial triage.
  - **Pricing/SaaS Viability**: The API is priced per conversation, with the first 1,000 service conversations per month being free. This is highly viable for small operators. It operates smoothly in cloud environments via webhooks and OAuth-based onboarding.
  - **User Experience**: Non-technical owners will connect their WhatsApp Business number to OHC. Once connected, all inbound DMs will appear in OHC's "Work Triage" feed, where the AI assistant can suggest replies or convert messages to quotes/bookings.

  ## Design Doc
  - **Onboarding**: A straightforward UI flow in the "Integrations" section where the owner clicks "Connect WhatsApp" and authenticates via Meta's embedded signup flow.
  - **Data Flow**: Inbound messages from Meta are received via an OHC webhook endpoint. OHC stores the message, associates it with a customer profile (using the phone number), and alerts the Work Triage agent.
  - **Action**: The OHC assistant can draft replies directly within the OHC UI. When the owner approves, OHC sends the message back via the WhatsApp Cloud API. The assistant can also trigger actions like sending a payment link via WhatsApp.
  - **Visibility**: The conversation thread is visible in the customer's OHC profile and the owner's daily feed.

  ## Implementation Prompt
  - Create a new "WhatsApp" integration under the OHC platform settings.
  - Implement a secure webhook endpoint to receive inbound messages from the WhatsApp Cloud API.
  - Build the functionality to send outgoing text and template messages to a customer's WhatsApp number.
  - Ensure that inbound messages appear in the Work Triage feed and the Customer Relationship context.
  - Provide a UI for the owner to review AI-drafted replies and click "Send via WhatsApp".
  - Ensure robust error handling for failed message deliveries or unauthorized webhook calls.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
