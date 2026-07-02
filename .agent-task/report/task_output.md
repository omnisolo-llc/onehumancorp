issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  ## Title
  Integrate WhatsApp Cloud API for Unified Customer Messaging

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a large volume of their orders, inquiries, and customer requests directly through WhatsApp. Currently, owners have to constantly switch between their personal/business WhatsApp app and their operational tools to manage orders, coordinate pickups, or answer questions. This creates fragmented communication, lost leads, and missed context. Owners need their WhatsApp messages to flow directly into their OHC Work Triage feed so they can reply, generate quotes, and create tasks without leaving their work assistant.

  ## Research Report
  **Market Context & Tool Discovery:**
  WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and parts of APAC, and is growing rapidly in the US. Competitors like HubSpot, DingTalk, and Shopify have heavily invested in unified inbox solutions that include WhatsApp.

  **Selected Tool: Meta's WhatsApp Cloud API**
  - **Ease of Use for Owners:** The owner does not interact with the API directly. They simply link their WhatsApp Business Account via an embedded OAuth flow (Meta Business Login) inside OHC. Once linked, messages appear in the OHC Work Triage feed alongside Instagram DMs and emails.
  - **Pricing:** Meta charges per conversation (a 24-hour window), with the first 1,000 user-initiated service conversations per month typically being free or very low cost depending on the region. This is highly viable for the free/entry tiers of OHC.
  - **Cloud vs. Standalone:** The Cloud API supports webhooks which fit perfectly into OHC's multi-tenant cloud architecture. For local standalone users, a tunneling solution or long-polling mechanism could be adapted, but the primary target is Cloud multi-tenant.
  - **Capabilities:** Supports rich text, images, document sharing, and quick reply buttons, all of which map cleanly to OHC's AI agent drafting and response capabilities.

  ## Design Doc
  - **Authentication & Setup:** Owners connect their WhatsApp Business account in OHC Settings via Meta's embedded signup flow. OHC stores the access token securely.
  - **Message Ingestion:** A centralized webhook endpoint in OHC receives incoming WhatsApp messages. The OHC Work Triage agent parses these messages, attaches them to the correct customer record (creating one if it doesn't exist based on phone number), and surfaces them in the owner's feed.
  - **Assistant Drafting & Response:** When an owner views a WhatsApp message, the Customer & Relationship Assistant drafts a proposed reply (e.g., an order confirmation or quote). The owner clicks "Send," and OHC dispatches the message back via the WhatsApp Cloud API.
  - **Notifications:** Urgent customer messages can trigger OHC push notifications based on priority rules set by the owner.

  ## Implementation Prompt
  **User-Facing Outcome:**
  The owner should be able to go to their OHC settings, click "Connect WhatsApp," and complete the Meta authorization flow. Once connected, any message sent by a customer to the owner's WhatsApp Business number should immediately appear in the OHC Work Triage feed. The owner should be able to read the message, review the AI-drafted reply, edit it if necessary, and send the response back to the customer's WhatsApp directly from OHC.

  **Acceptance Criteria:**
  - Owner can successfully authorize and link a WhatsApp Business account.
  - Incoming text and image messages from WhatsApp appear in the Work Triage feed within seconds.
  - The AI Assistant automatically drafts contextual replies to new WhatsApp inquiries.
  - Owner can send messages from the Work Triage feed back to the customer's WhatsApp.
  - The UI accurately reflects message status (e.g., sent, delivered).
  - The implementation must cleanly handle Meta's webhook verification and ongoing token lifecycle without owner intervention.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
