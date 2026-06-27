issue_title: "Integrate WhatsApp Business API via Twilio for Unified Messaging"
issue_description: |
  **Title**: Integrate WhatsApp Business API via Twilio for Unified Messaging

  **Problem Statement**: What gap, pain point, or opportunity does this address?
  Operators like Maya (Home Baker) and Fatima (Food Cart Operator) often receive a high volume of orders and inquiries via WhatsApp. Currently, managing these on a standalone mobile device or via the disjointed WhatsApp Business app leads to missed leads, slow response times, and limited visibility for anyone else on the team. They need a unified way to handle WhatsApp conversations within OHC's Work Triage, transforming casual chats into actionable tasks, quotes, or bookings without constantly switching apps.

  **Research Report**:
  WhatsApp is the dominant messaging platform in many global markets, making it a critical channel for small businesses. The native WhatsApp Business App is limited to single-device or simple multi-device use, lacking deep integration into business workflows.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Ease of Use for Owners**: Owners do not need to understand Twilio's infrastructure. They simply authenticate and connect their business number through an OHC setup flow.
  - **Pricing & Viability**: Twilio utilizes a pay-as-you-go conversational pricing model. User-initiated conversations are very cost-effective (often a few cents per 24-hour session, depending on region). This makes it highly viable for OHC's multi-tenant SaaS Cloud model, with potential to pass through costs or bundle them into premium tiers.
  - **Reputation & API Quality**: Twilio is an industry standard with robust webhooks, high reliability, and excellent documentation for managing multi-tenant messaging.

  **Design Doc**:
  - **Connection**: Owner navigates to OHC Settings > Integrations and connects their WhatsApp Business account (via embedded Twilio signup or OAuth-like flow).
  - **Ingestion (Trigger)**: A customer messages the business on WhatsApp. Twilio sends a webhook to OHC.
  - **Processing (Action)**: OHC identifies the tenant via the webhook's recipient number, finds or creates the customer profile, and surfaces the message in the "Work Triage" feed.
  - **Owner Experience (UI)**: The owner sees the WhatsApp message alongside DMs and web inquiries. The Customer Assistant AI automatically drafts a reply based on business context.
  - **Fulfillment**: The owner clicks "Send", and OHC pushes the message back through the Twilio API. If the intent is a purchase, the AI can prompt the owner to insert a payment link directly into the WhatsApp thread.

  **Implementation Prompt**:
  - Build a secure webhook endpoint to receive incoming Twilio WhatsApp messages, ensuring proper tenant routing and signature validation.
  - Create the frontend integration setup screen for connecting a WhatsApp account.
  - Update the Work Triage UI to support a new "WhatsApp" message type, complete with WhatsApp icon indicators.
  - Implement the reply capability to send outbound messages through Twilio.
  - Ensure all database writes for messages and customer profiles respect row-level security and tenant isolation.
  - Acceptance Criteria: A non-technical owner can connect WhatsApp, receive a message from a customer in the Work Triage feed, read an AI-drafted reply, and successfully respond without leaving the OHC interface.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []