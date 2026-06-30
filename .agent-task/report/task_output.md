issue_title: "Integrate WhatsApp Business Messaging via Twilio"
issue_description: |
  **Title**: Integrate WhatsApp Business Messaging via Twilio

  **Problem Statement**:
  Owners like Maya (Home Baker) and Carlos (Field Service) communicate with clients primarily through WhatsApp. Right now, these messages are disconnected from OHC, requiring owners to manually switch between WhatsApp on their phones and OHC to draft quotes, check booking times, or update order statuses. This fragmentation leads to dropped leads, missed follow-ups, and an incomplete owner feed.

  **Research Report**:
  - **Competitor Landscape**: Tencent Workbuddy, WeCom, and CRM tools like HubSpot integrate deeply with WhatsApp, allowing business owners to consolidate work in one inbox.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Capabilities & Limits**: Twilio provides a robust, developer-friendly REST API for sending/receiving WhatsApp messages. It supports rich media (images for cake references or service photos), templates (for appointment reminders), and webhooks for real-time inbound message processing. Rate limits apply, and Meta requires template approval for outbound business-initiated messages.
  - **SaaS Viability**: Twilio uses a pay-as-you-go pricing model with no large upfront costs. It can operate in multi-tenant mode, routing messages via a unified webhook (using OHC's `tenant_id` mappings based on incoming numbers or Twilio subaccounts).
  - **User-First Value Mapping**: A non-technical owner will connect their WhatsApp number once. OHC will then route incoming WhatsApp DMs directly into the "Work Triage" feed. The Customer Assistant can instantly draft replies within OHC, keeping context intact.

  **Design Doc**:
  - **Trigger**: Incoming WhatsApp message hits OHC Twilio Webhook. Outgoing messages triggered by owner actions or Agent drafts via OHC interface.
  - **Integration**:
    - Store Twilio credentials and WhatsApp Sender ID per tenant.
    - Implement a unified webhook endpoint that maps incoming messages to the correct `tenant_id` and creates an inbox task.
    - Expose a capability in the AI Assistant (Customer Assistant) to draft and send replies.
  - **User Experience**: The owner sees WhatsApp messages inline with emails and web forms in the OHC shell. They can tap "Reply" within OHC, and the response is sent seamlessly via WhatsApp.

  **Implementation Prompt**:
  Implement a Twilio WhatsApp integration that allows owners to send and receive WhatsApp messages directly from the OHC Assistant Feed. Create the necessary backend webhook to ingest inbound Twilio webhooks, map them to an OHC tenant, and present them in the Work Triage UI. Add functionality for the AI Customer Assistant to draft replies that the owner can approve and send back via WhatsApp. The end goal is to deliver the end-to-end user experience of connecting WhatsApp and messaging customers seamlessly within OHC.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
