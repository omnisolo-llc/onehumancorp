issue_title: "Integration: Meta WhatsApp Cloud API for Work Triage"
issue_description: |
  **Title**: Integration: Meta WhatsApp Cloud API for Work Triage

  **Problem Statement**:
  For small business owners like Maya (Home Baker) and Fatima (Food Cart Operator), WhatsApp is the primary channel where customer demand happens. Currently, managing multiple DMs directly on a phone leads to missed orders, forgotten follow-ups, and an inability to share customer context with staff or agents. They need these WhatsApp inquiries to flow directly into the OHC Work Triage feed as actionable items, so they can turn casual messages into quotes, bookings, and tasks without manually switching apps.

  **Research Report**:
  *   **Market Context**: Tools like WeCom (Tencent) and WhatsApp Business API integrations are heavily requested by small business operators globally, especially outside the US. Many competitors like HubSpot or Zendesk offer this, but they feel like heavy CRMs. A lightweight, assistant-driven integration is highly differentiated.
  *   **Capabilities & Limits**: The Meta WhatsApp Cloud API allows sending and receiving messages using a phone number registered to a WhatsApp Business Account. It supports text, media, and interactive messages (like buttons or lists). It requires webhooks to receive incoming messages. There is a 24-hour customer service window for free-form replies; outside that, approved template messages are required.
  *   **SaaS Viability**: Meta provides the API directly. Pricing is conversation-based (user-initiated vs. business-initiated). It's very viable for a multi-tenant Cloud setup (OAuth with Meta Business Manager) but also feasible for a Standalone setup if the user brings their own Meta developer app credentials or we proxy it securely.
  *   **User-First Value Mapping**: A non-technical owner just wants to connect their WhatsApp business number. Once connected, a customer messaging "Do you have any cakes available today?" appears in OHC's Work Triage. The Customer Assistant drafts a reply, and the Operations Assistant can help create an order directly from the thread.

  **Design Doc**:
  *   **Connection Flow**: The user links their WhatsApp Business account in OHC Settings. This authorizes OHC to receive messages via webhooks.
  *   **Inbound (Webhook)**: When a WhatsApp message arrives, OHC creates or updates a "Customer Conversation" entity in the database.
  *   **Triage Integration**: The incoming message triggers an event for the Work Triage agent. If the customer is new, a lead is created. If it's an existing order, the message is attached to that order's context.
  *   **Outbound**: When the owner (or an approved AI agent) replies from OHC, the backend sends the message payload to the WhatsApp Cloud API.
  *   **UI/UX**: The Work Triage feed displays WhatsApp messages with a recognizable WhatsApp icon. The chat interface supports drafting, AI assistance, and one-click actions (e.g., "Create Quote").

  **Implementation Prompt**:
  Implement the backend and frontend components to connect OHC Work Triage with the Meta WhatsApp Cloud API.
  1. Build a setup flow in the UI where an owner can securely connect their WhatsApp Business account.
  2. Implement a webhook receiver in the backend to process incoming WhatsApp messages and ingest them into the OHC database as Customer Conversations.
  3. Update the Work Triage feed UI to display these inbound WhatsApp messages as actionable items.
  4. Build the outgoing message flow so that replies drafted in OHC are sent back to the customer via WhatsApp.
  5. Ensure the Customer Assistant agent has access to the conversation context to draft smart replies.
  Acceptance Criteria: A user can connect their account, receive a WhatsApp message in their Work Triage feed, and successfully send a reply from the OHC UI that reaches the customer's WhatsApp app.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
