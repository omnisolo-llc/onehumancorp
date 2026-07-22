issue_title: "Integrate Twilio WhatsApp Business API for Unified Customer Messaging"
issue_description: |
  # Mission Queue Protocol: WhatsApp Business Integration Brief

  ## Title
  Integrate Twilio WhatsApp Business API for Unified Customer Messaging

  ## Problem Statement
  Owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart Operator) run 80% of their customer communications through WhatsApp. Currently, these interactions occur outside of the OHC system. This forces owners to constantly context-switch between their personal devices and the OHC platform. Because OHC lacks visibility into these conversations, our AI assistant cannot capture incoming demand, draft replies, surface relevant past orders, or generate quotes and bookings automatically. The absence of WhatsApp integration breaks our core promise of a "unified work feed" where the owner knows exactly what needs attention.

  ## Research Report
  - **Dynamic Discovery:** Competitor platforms like Sirena, Zoko, HubSpot, and Tencent Workbuddy heavily feature WhatsApp or WeChat integrations as their top-installed plugins. In emerging markets (LATAM, India, SE Asia) and parts of Europe, WhatsApp is the *only* channel customers use to interact with small businesses.
  - **Tool Evaluated:** Twilio API for WhatsApp Business vs. Meta Cloud API. We recommend Twilio due to its robust webhook reliability, simplified template approval process, and straightforward multi-tenant SaaS capabilities, although Meta Cloud API is a valid alternative.
  - **User-First Value Mapping:** For a non-technical owner like Maya, this means she can link her WhatsApp Business number to OHC. From that moment, any WhatsApp message she receives appears in her OHC feed. The AI Assistant pre-drafts replies (e.g., "Yes, we have vegan options for Friday!"), which she can review and send with one tap, all without leaving the OHC interface.
  - **SaaS Viability & Pricing:** Twilio uses a pay-as-you-go model per conversation. OHC can absorb this cost in a premium tier or pass it along seamlessly. It perfectly supports Cloud (multi-tenant) operations by using Twilio subaccounts or dynamic webhook routing based on incoming numbers.
  - **Capabilities & Limits:** The main constraint is the "24-hour session window" imposed by Meta. If an owner replies after 24 hours, they must use a pre-approved template message. The integration must gracefully handle this limitation, guiding the owner to use an approved template when the window expires.

  ## Design Doc
  - **Trigger:** A customer sends a WhatsApp message to the owner's Twilio-provisioned or linked business number.
  - **Action:**
    - Twilio fires a webhook to OHC (`/api/webhooks/twilio/whatsapp`).
    - The OHC backend verifies the Twilio signature, identifies the tenant via the destination number, and persists the message to the tenant's unified `Conversation` table.
    - The event triggers the `Work Triage` queue.
    - The `Customer Assistant` agent reads the message intent, retrieves customer context, and generates a draft reply.
    - If the intent requires operations (e.g., a quote), the `Sales Assistant` prepares a pending quote artifact.
  - **User Visibility:** The owner's UI (Mobile/Web) updates in real-time, surfacing the new message in the "Today's Priorities" feed. They see the AI-drafted reply and any generated artifacts (quotes/bookings). They can tap "Approve & Send," which calls the OHC backend to dispatch the reply via the Twilio API.
  - **Edge Cases:** If the 24-hour window has closed, the UI disables free-form text input and prompts the owner to select an approved "Follow-up" template.

  ## Implementation Prompt
  - Create a "Connect WhatsApp" settings flow in the UI where owners can authenticate or provision their number via Twilio embedded signup.
  - Implement a secured, scalable webhook endpoint to receive and acknowledge Twilio incoming messages.
  - Parse the incoming payload, match it to the correct tenant, and insert it into the unified messaging data store.
  - Set up an async job that triggers the AI assistant to read the new message and produce a `DraftReply` record.
  - Expose an endpoint for the UI to fetch the conversation and submit a reply.
  - When the owner approves the reply, send it via the Twilio WhatsApp API.
  - **Acceptance Criteria:** A non-technical user can connect their WhatsApp, receive an external message in the OHC shell, view an AI-generated draft, and successfully send a reply back to the customer's WhatsApp device. The integration must function correctly on a 375px mobile screen.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
