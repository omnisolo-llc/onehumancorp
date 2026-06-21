issue_title: "Integrate WhatsApp Business API for Centralized Customer Comms"
issue_description: |
  **Title**: Integrate WhatsApp Business API for Centralized Customer Comms

  **Problem Statement**:
  Small business owners like Maya (Home Baker), Carlos (Field Service Owner), and Fatima (Food Cart Operator) receive a large portion of their orders, inquiries, and customer communication directly through WhatsApp. Currently, these interactions exist outside of the OHC ecosystem, requiring owners to constantly context-switch between their phone's WhatsApp app and OHC to coordinate bookings, issue quotes, and manually capture tasks. This fragmentation causes delayed responses, missed revenue opportunities, and a scattered "customer memory" that the AI cannot access to help draft intelligent follow-ups.

  **Research Report**:
  - **Tool Evaluated**: Meta WhatsApp Cloud API / Twilio WhatsApp Business API.
  - **Market Context**: WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and increasingly NA. Competitors like Shopify App Store, HubSpot, and Wix all offer robust WhatsApp integrations, often as top-tier highly reviewed plugins.
  - **Usability for Non-Technical Owners**: Owners must not be burdened with creating Meta Developer apps or configuring webhooks. The integration must utilize the Embedded Signup flow (OAuth-style) enabling a non-technical owner to connect their existing WhatsApp Business number with a simple pop-up authentication.
  - **Pricing**: Meta charges per 24-hour conversation window (marketing, utility, service). The pricing is affordable and well-suited for a SaaS tier offering or pass-through billing.
  - **Capabilities**: Supports rich media, templates, and interactive messages (e.g., buttons, lists). This enables OHC to send actionable quotes directly to the chat (e.g., a message with "Approve Quote" and "Request Changes" buttons).
  - **Cloud vs Standalone**: In a multi-tenant Cloud setup, OHC handles the central webhooks and routes messages to tenants securely. In Standalone, the user can provide their own Meta/Twilio API keys to run the service locally.

  **Design Doc**:
  - **Integration Architecture**:
    - Incoming webhooks hit a dedicated OHC ingestion endpoint, verified via signature.
    - Messages are placed on the AI Job Queue for processing and linked to the tenant's Customer Profile (using the sender's phone number as the key).
  - **Trigger**: Customer sends a WhatsApp message or interacts with a WhatsApp template.
  - **Action**: The message is surfaced in the unified **Work Triage** feed. The **Customer & Relationship Assistant** contextually analyzes the message against active orders, schedules, and past interactions to instantly draft a suggested reply.
  - **User View (Owner)**: The owner sees a unified message thread on their 375px mobile screen. An AI-drafted reply is pre-filled in the text box. They simply tap "Approve" or edit the draft. If a customer asks for a cake (Maya) or a repair estimate (Carlos), the **Operations Assistant** will propose creating a new Order or Task card right next to the chat context.

  **Implementation Prompt**:
  Build the WhatsApp Business API integration to seamlessly merge WhatsApp conversations into OHC's core work feed. The user-facing outcome must allow an owner to effortlessly connect their WhatsApp account, read messages within OHC, and leverage AI to draft responses and manage related work.

  *Acceptance Criteria*:
  1. Provide a zero-jargon "Connect WhatsApp" button in the Integrations UI utilizing the Embedded Signup flow.
  2. Incoming WhatsApp messages must securely appear in the owner's Work Triage feed in real-time.
  3. The Customer Assistant must automatically draft contextual replies based on the tenant's business history and current operations.
  4. Outgoing messages from the owner via OHC are reliably delivered to the customer's WhatsApp.
  5. Network Resilience: On flaky mobile connections, outgoing replies must show a truthful "pending" state and automatically retry in the background without losing the owner's input.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
