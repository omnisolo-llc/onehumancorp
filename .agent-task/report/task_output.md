issue_title: "Integrate Twilio WhatsApp Business API for Unified Customer Inbox"
issue_description: |
  # Mission Queue Protocol: Integration Brief

  ## Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service) communicate with their customers heavily through WhatsApp. Currently, their work is split between their personal/business WhatsApp app and their operational tools. This scattered communication means missed leads, forgotten follow-ups, and the inability for OHC's AI agents to draft replies, build customer memory, or trigger operational workflows directly from WhatsApp conversations. A unified inbox is critical for keeping momentum without technical overhead.

  ## Research Report
  **Market Need:**
  Competitors like WeCom, WhatsApp Business App, and Shopify App Store plugins demonstrate that conversational commerce via WhatsApp is a primary channel, especially in LATAM, EMEA, and APAC. WhatsApp has an over 90% open rate for small business messaging.

  **Selected Tool:** Twilio WhatsApp Business API
  - **Ease of Use:** For the end-user (owner), the integration will be seamless. They simply authorize their WhatsApp Business number through a guided OAuth/setup flow within OHC, and messages appear in the OHC Work Triage feed.
  - **Pricing:** Twilio offers pay-as-you-go pricing (e.g., $0.005 per message, plus Meta's conversation charges). This is highly scalable for a multi-tenant SaaS.
  - **Reputation & Viability:** Twilio provides robust developer docs, reliable webhooks, and handles Meta's complex compliance rules. It supports both Cloud (multi-tenant) and can be configured per tenant.
  - **Alternatives Considered:** Direct Meta Cloud API (cheaper, but harder tenant management and webhook verification), MessageBird (similar to Twilio but less widespread developer adoption).

  ## Design Doc
  - **Trigger:** When a customer sends a WhatsApp message to the owner's WhatsApp Business number, Twilio fires a webhook to OHC.
  - **Action:** OHC routes the incoming message to the specific tenant's Work Triage feed. The Customer & Relationship Assistant analyzes the message context, associates it with the customer profile (creating one if necessary), and drafts a suggested reply.
  - **Owner View:** The owner sees the message in their unified Assistant feed on their mobile app (375px layout), along with the AI-drafted reply and any suggested operational tasks (e.g., "Create a quote for this cake").
  - **Outgoing:** When the owner approves the draft or types a reply, OHC sends it via the Twilio API back to the customer's WhatsApp.

  ## Implementation Prompt
  Implement the Twilio WhatsApp integration so that owners can receive and reply to WhatsApp messages from within OHC.
  - Build a secure webhook handler to receive incoming messages from Twilio.
  - Ensure messages are correctly routed to the appropriate tenant's Work Triage feed using the recipient phone number.
  - Enable the Customer Assistant to read the message history and generate draft replies.
  - Provide a simple UI flow for the owner to authorize their Twilio credentials or WhatsApp number.
  - Ensure the chat UI works flawlessly on mobile (375px), showing real-time updates and clear pending/sent statuses.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
