issue_title: "WhatsApp Cloud API Integration"
issue_description: |
  **Title**: WhatsApp Cloud API Integration for Work Triage and Customer Assistant

  **Problem Statement**:
  For small business owners like Maya (home baker), Carlos (field service), and Fatima (food cart), WhatsApp is the primary channel for customer inquiries, orders, and support. However, managing these messages on a personal phone creates fragmented workflows, missed leads, and manual data entry into other systems. Owners need a unified inbox where WhatsApp messages are automatically triaged, tied to customer profiles, and can be replied to using AI drafts, without needing to constantly switch apps.

  **Research Report**:
  - **Tool Name**: WhatsApp Cloud API (Meta)
  - **Ecosystem Demand**: WhatsApp Business is dominant globally (especially in LATAM, EMEA, and APAC). Competitors like HubSpot, DingTalk, and Zendesk all heavily feature WhatsApp integrations as top-tier marketplace apps.
  - **Capabilities**: Enables programmatic sending and receiving of text, media, and interactive messages (buttons, lists). Supports webhooks for real-time inbound message events and status updates (sent, delivered, read).
  - **Ease of Use for Owners**: High. Owners continue to promote their standard WhatsApp number. Once connected, OHC acts as the backend routing engine. The owner simply uses the OHC Work Triage feed.
  - **Pricing**: The first 1,000 service conversations per month are free, which easily covers small businesses. User-initiated conversations are very low cost. Operates effectively in Cloud (multi-tenant via embedded signup).

  **Design Doc**:
  - **Integration Point**: Integrate with the `Work Triage` and `Customer & Relationship Assistant` domains.
  - **User Flow**:
    1. Owner navigates to OHC Settings > Integrations > WhatsApp.
    2. Owner clicks "Connect WhatsApp" and completes the Meta Embedded Signup flow to link their Business Number to OHC.
    3. OHC registers a webhook to listen for inbound messages to that number.
    4. When a customer messages the WhatsApp number, the OHC webhook receives the event, maps it to the appropriate Tenant using the connected number, and creates/updates a customer profile.
    5. The message appears in the owner's `Work Triage` feed. The AI `Customer Assistant` drafts a reply based on business context.
    6. The owner approves or edits the draft in OHC, and OHC sends the reply out via the WhatsApp Cloud API.

  **Implementation Prompt**:
  Create a new integration for the WhatsApp Cloud API.
  1. Add a UI flow for the owner to connect their WhatsApp Business account via Meta's Embedded Signup.
  2. Implement webhook handlers to receive inbound text and image messages from WhatsApp, securely attributing them to the correct OHC tenant.
  3. Route inbound messages into the `Work Triage` feed and trigger the `Customer Assistant` to draft suggested replies based on past context and tenant data.
  4. Build an outbound API client that allows the owner to send approved replies back to the customer's WhatsApp.
  5. Ensure the UI clearly shows the source of the message (WhatsApp icon) in the triage feed.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
