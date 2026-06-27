issue_title: "Integrate WhatsApp Business API (via Twilio) for Unified Customer Messaging"
issue_description: |
  **Title**: Integrate WhatsApp Business API (via Twilio) for Unified Customer Messaging

  **Problem Statement**:
  Small business owners and operators (e.g., Maya the Home Baker, Carlos the Field Service Owner, Fatima the Food Cart Operator) manage a massive volume of customer demand, orders, and questions through WhatsApp. Currently, this communication is trapped in a silo—separated from OHC’s scheduling, quoting, and payment systems. Owners are forced to context-switch between their phone's WhatsApp app and OHC, manually copying information, which leads to missed leads, delayed responses, and lost revenue.

  **Research Report**:
  - **Ecosystem & Competitor Analysis**: Leading operator tools like WeCom, DingTalk, and Shopify Inbox recognize that conversational commerce is essential. WhatsApp is the undisputed dominant messaging platform in LATAM, EMEA, and APAC, and is rapidly growing as a business channel in North America.
  - **Tool Evaluated**: Twilio's WhatsApp Business API.
  - **Owner/Operator Benefit**: Connects their business number to OHC, routing all customer inquiries into the unified Work Triage feed. Owners no longer need to monitor a separate device or app. OHC's AI agents can read WhatsApp messages, understand the context of the customer's past orders, and draft accurate replies for the owner to approve with one tap.
  - **SaaS Viability**: Twilio provides a highly reliable, scalable API with a straightforward pay-as-you-go conversational pricing model. It supports both multi-tenant Cloud environments (where OHC can manage numbers on behalf of tenants) and Standalone environments. The developer experience, webhook reliability, and SLA are enterprise-grade.

  **Design Doc**:
  - **Integration Point**: A new WhatsApp integration module in the backend.
  - **Trigger**: Customer sends a WhatsApp message to the owner's provisioned Twilio number.
  - **Action**: Twilio sends a webhook to OHC. The integration layer authenticates the webhook, extracts the `tenant_id` and customer context, and pushes the message into the unified Work Triage feed. The Customer & Relationship Assistant agent is triggered to draft a context-aware reply.
  - **User View**: The owner opens the OHC Assistant on their 375px mobile screen. They see a new unread WhatsApp message in the unified feed, complete with an AI-drafted response (e.g., "Yes, we can deliver the custom cake by 3 PM tomorrow. Here is the deposit link: [Link]"). The owner taps "Approve & Send", which routes the message back through Twilio to the customer's WhatsApp.

  **Implementation Prompt**:
  Build a WhatsApp Business integration using Twilio that seamlessly routes incoming customer messages into the OHC Work Triage feed.

  Outcome: A non-technical owner should be able to navigate to "Settings > Channels", select "WhatsApp", and securely connect their account. Once active, incoming messages must instantly appear in their OHC command center. The owner must be able to read the message, review AI-drafted replies, and respond natively within the OHC app. Outbound system events (like booking confirmations) should also have the option to send standard WhatsApp notification templates.

  Acceptance Criteria:
  1. Secure OAuth/Connection flow for the owner to link WhatsApp via Twilio.
  2. Incoming messages populate the Work Triage feed in real-time.
  3. Owners can reply from the OHC mobile/desktop UI, which sends the message back to the customer's WhatsApp.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
