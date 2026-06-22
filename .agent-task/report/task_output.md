issue_title: "Integrate WhatsApp Business API via Twilio for Work Triage and Operations"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service) interact heavily with their customers via WhatsApp. Currently, these messages exist in a silo, requiring owners to switch back and forth between their personal/business WhatsApp app and OHC to coordinate bookings, answer inquiries, and share payment links. This context-switching leads to missed messages, lost leads, and manual data entry errors. OHC needs to natively capture WhatsApp interactions so the Work Triage and Customer Assistant capabilities can group inquiries, draft replies, and track order context seamlessly.

  ## Research Report: Twilio WhatsApp Business API
  **Market Need Discovery:**
  - **Competitive Landscape:** Top competitors like HubSpot, Wix, and Shopify have robust WhatsApp integrations as it's the primary messaging channel for small businesses in LATAM, Europe, and Asia.
  - **Relevance to Personas:**
    - *Maya (Baker)* receives custom cake orders and reference photos via WhatsApp.
    - *Carlos (Field Service)* texts ETAs to customers and gets photos of broken appliances.
  - **Tool Evaluated:** Twilio MessagingX (WhatsApp Business API)
  - **Capabilities:** Supports inbound/outbound text and media, conversation history, and rich interactive templates (buttons, list messages).
  - **Pricing Viability:** Twilio offers a pay-as-you-go model per conversation, making it viable for both our Multi-Tenant Cloud mode (where we aggregate usage and bill the tenant) and Standalone mode (where the owner provides their own Twilio credentials).
  - **Ease of Use for Owners:** Non-technical owners shouldn't need to learn Twilio. In OHC, they should just follow an OAuth/Connect flow (in cloud) or paste an API Key (in standalone) and instantly see their WhatsApp messages flow into the OHC feed.

  ## Design Doc
  - **Trigger/Input:**
    - Twilio webhooks deliver incoming WhatsApp messages to OHC.
    - The Work Triage capability ingests these payloads, parsing sender info, text, and media.
  - **Actions Taken:**
    - Inbound messages create or update a unified Customer thread in the tenant's workspace.
    - The OHC Customer Assistant agent reads the thread and drafts a suggested reply.
    - If the message is a new inquiry, OHC generates a pending task/lead for the owner to review.
    - Outbound messages approved by the owner or sent automatically (like booking reminders) are pushed back out via the Twilio API.
  - **User Experience (What the user sees):**
    - A centralized "Inbox" or "Work Feed" that displays WhatsApp messages alongside emails and IG DMs.
    - A simple setting screen: "Connect WhatsApp".
    - In-thread actions to "Draft Reply with AI", "Send Payment Link", or "Create Booking".

  ## Implementation Prompt
  **User-Facing Outcome:**
  The owner should be able to connect their WhatsApp Business account to OHC. Once connected, any incoming WhatsApp message should appear in the owner's Work Triage feed. The AI Assistant should draft a context-aware reply based on the message. The owner can click "Approve & Send", which will deliver the reply back to the customer's WhatsApp.

  **Acceptance Criteria:**
  1. Provide a UI in the settings for the owner to connect their Twilio WhatsApp account.
  2. Inbound webhook endpoint that securely receives and validates Twilio payloads.
  3. Messages must be displayed in the Work Triage UI on both desktop and 375px mobile layouts without horizontal scroll.
  4. The Customer Assistant must automatically propose a draft response to new WhatsApp inquiries.
  5. The owner must be able to send manual or AI-drafted replies back through the WhatsApp API.
  6. E2E Playwright tests must verify the UI connection flow and simulate a message appearing in the Work Triage feed.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
