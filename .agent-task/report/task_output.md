issue_title: "Integration: WhatsApp Business for Unified Customer Communication"
issue_description: |
  **Title**: Integrate WhatsApp Business API for Unified Customer Messaging

  **Problem Statement**:
  For our owner/operator personas like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart), WhatsApp is the primary channel for customer inquiries, orders, and service requests. Currently, these owners have to constantly switch between their personal/business WhatsApp app and OHC to coordinate work, leading to missed leads, delayed responses, and fragmented customer context. They need OHC to ingest WhatsApp messages directly so the AI assistant can triage them, draft replies, and trigger workflows (like creating a quote or booking) without leaving the OHC command center.

  **Research Report**:
  - **Market Context**: WhatsApp is the dominant messaging platform in LATAM, EMEA, and parts of APAC. Competitors like Shopify, Wix, and HubSpot all offer robust WhatsApp integrations. Tools like Zendesk and Intercom have made WhatsApp a first-class citizen for customer support.
  - **Integration Candidates**:
    - *Meta WhatsApp Cloud API*: Direct from Meta. No middleman fees, but requires Facebook Business Manager verification which can be a hurdle for very small or new businesses.
    - *Twilio API for WhatsApp*: Offers a smoother developer experience, easy Sandbox testing, and unifies SMS + WhatsApp under one API. Slightly higher cost per message due to Twilio's markup, but simplifies onboarding.
  - **Owner/Operator Usability**: Using Twilio or a similar aggregator allows us to handle the complex Meta compliance and template approvals on the backend. The owner just clicks "Connect WhatsApp" and follows an OAuth/onboarding flow. Once connected, WhatsApp DMs appear in the unified OHC "Work Triage" feed just like emails or web forms.
  - **SaaS Viability**: Twilio's pricing is pay-as-you-go, which maps well to OHC's multi-tenant architecture. We can bill usage back to the tenant or include a baseline quota in their subscription.

  **Design Doc**:
  - **Trigger**: Customer sends a WhatsApp message to the owner's registered WhatsApp Business number.
  - **Ingestion**: A webhook endpoint in the OHC backend (e.g., handling Twilio webhook payloads) receives the message, associates it with the correct `tenant_id` (via the receiving phone number), and creates/updates a customer conversation record.
  - **AI Triage**: The OHC Customer Assistant capability is triggered to analyze the incoming message, extract context (e.g., an order inquiry), and draft a suggested reply.
  - **Owner UI**: The message appears in the owner's unified assistant feed. The owner reviews the AI-drafted reply and taps "Send".
  - **Egress**: The backend sends the approved reply back through the Twilio WhatsApp API.

  **Implementation Prompt**:
  Implement the backend ingestion and egress layer for WhatsApp Business messages (using Twilio or Meta Cloud API).
  - Create the necessary webhook receiver to accept incoming WhatsApp messages and route them to the correct tenant.
  - Integrate this flow with the existing OHC Customer Assistant so that incoming messages appear in the unified work feed and trigger an AI reply draft.
  - Provide an outbound function to send messages back to the customer.
  - Acceptance Criteria: A test message sent to the configured WhatsApp number appears in the OHC UI for the correct tenant. The owner can click "Approve and Send" on an AI draft, and the reply is successfully delivered to the customer's WhatsApp.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
