issue_title: "Feature: Twilio WhatsApp Business Integration for Customer Messaging"
issue_description: |
  **Mission Queue Protocol - Research Report**

  **Title**: Feature: Twilio WhatsApp Business Integration for Customer Messaging

  **Problem Statement**:
  Small business owners and operators (like Maya the home baker or Fatima the food cart operator) rely heavily on WhatsApp to communicate with their customers, take pre-orders, and handle inquiries. Currently, these interactions happen outside of OHC, leading to scattered context, missed opportunities, and manual copying of information. Non-technical owners need a unified assistant that intercepts WhatsApp messages, drafts replies, creates bookings or orders directly from chat, and remembers customer preferences without forcing them to juggle multiple apps.

  **Research Report**:
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Relevance & Ecosystem**: WhatsApp is the dominant communication channel for SMBs in many international markets (LATAM, Europe, India). Competitors like WeCom and Shopify offer deep messaging integrations. Twilio provides a mature, reliable API wrapper around the Meta WhatsApp Business Platform.
  - **Capabilities**:
    - Supports two-way conversational messaging (customer support, order taking).
    - Supports one-way notifications (OTP, appointment reminders, order status).
    - Handles rich media (images, PDFs) and interactive buttons/templates.
    - Webhooks for incoming messages enable real-time OHC agent interception.
  - **Ease of Use (for Owners)**: Non-technical users do not need to understand Twilio. OHC will handle the technical integration. The owner only needs to connect their business number (via OAuth/Twilio ISV onboarding or self-provided credentials) and see messages appear in the OHC Work Triage feed.
  - **SaaS Viability & Architecture Fit**:
    - **Cloud (Multi-tenant)**: OHC can act as a Twilio ISV/Tech Provider or allow tenants to plug in their own Twilio Account SID/Auth Token. Webhooks can be routed by tenant ID.
    - **Standalone (Local)**: The user provides their own Twilio credentials. Webhooks require a tunnel (e.g., ngrok) or polling, but outbound messaging works seamlessly.
  - **Pricing**: Twilio charges per conversation (marketing, utility, service). This is standard and can be passed through or bundled into OHC premium tiers.

  **Design Doc**:
  - **Integration Point**: A new WhatsApp Service connected to the OHC backend.
  - **Setup UI**: A "Connect WhatsApp" settings card in the OHC desktop/mobile app allowing the user to provide their Twilio credentials.
  - **Inbound Flow**:
    1. Incoming messages from customers on WhatsApp are received via the backend webhook mechanism.
    2. The message payload is parsed and the customer is matched based on phone number. The message is stored in the database.
    3. The AI Job Queue picks up the message, allowing the Customer & Relationship Assistant to draft a reply or create a task in the Work Triage feed.
  - **Outbound Flow**: The owner reviews the AI-drafted reply in the OHC UI and clicks "Send." The backend uses the Twilio API to deliver the message back to the customer on WhatsApp.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp messaging integration.
  1. Add a settings UI for users to connect their Twilio account and provide the required credentials.
  2. Implement backend handlers to receive incoming Twilio WhatsApp messages and store them associated with the correct customer profile.
  3. Integrate the inbound messages into the Work Triage feed, allowing the AI to automatically draft replies.
  4. Implement the outbound messaging flow, allowing the owner to send messages from the OHC interface back to the customer's WhatsApp.
  Acceptance Criteria: A user can successfully connect their Twilio account, receive a WhatsApp message from a customer which appears in their OHC Work Triage feed, and send a reply back from the OHC UI to the customer's WhatsApp.

  **Priority**: P1 (High)

  **Estimated Scope**: Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
