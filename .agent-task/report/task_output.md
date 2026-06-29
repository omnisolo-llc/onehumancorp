issue_title: "Integration: Native WhatsApp Business Messaging via Meta API"
issue_description: |
  **Problem Statement:**
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a huge portion of their customer inquiries, orders, and follow-ups directly through WhatsApp. Currently, owners have to switch constantly between OHC and their personal or business WhatsApp apps on their phone, leading to missed messages, lost context, and disjointed customer records. They need WhatsApp to be a first-class citizen in OHC's Work Triage, so the AI Assistant can draft replies and capture orders without leaving the platform.

  **Research Report:**
  - **Market Need:** WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and APAC. Competitors like HubSpot and Shopify heavily promote their WhatsApp integrations.
  - **Technical Tool:** Meta's WhatsApp Cloud API (or Twilio API for WhatsApp). Meta now offers direct access to the Cloud API without requiring a BSP (Business Solution Provider), significantly reducing costs for the owner.
  - **Ease of Use for Owners:** High. The owner simply authenticates their Meta Business account once. After that, OHC handles all message routing.
  - **Pricing:** The first 1,000 service conversations per month are free on Meta's Cloud API, which easily covers most early-stage owner/operators.
  - **SaaS Viability:** Cloud-hosted (multi-tenant) via Meta Webhooks.

  **Design Doc:**
  - **Trigger:** A new message arrives at the owner's WhatsApp Business number.
  - **Action:** A Meta Webhook fires to an OHC endpoint. The backend processes the incoming payload and creates a new message thread in OHC's `Customer Relationships` module.
  - **Assistant Integration:** OHC's AI Work Triage reads the incoming message context, identifies the customer, and drafts a reply. For standard inquiries (e.g., "Are you open today?"), the assistant suggests a 1-tap response.
  - **Owner View:** The owner sees the WhatsApp message right inside the OHC Assistant feed, indistinguishable from a web form or email but labeled with a WhatsApp icon. When they hit "Send," OHC pushes the reply back through the Meta API.

  **Implementation Prompt:**
  Implement the WhatsApp Business Cloud API integration. Create an onboarding flow where the owner can link their Meta Business account. Build the webhook handler to receive incoming WhatsApp messages and insert them into the existing OHC message triage feed. Integrate the AI assistant to read these messages and generate draft replies. Ensure the outbound message API is wired up so the owner's reply in OHC sends successfully to the customer's WhatsApp.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
