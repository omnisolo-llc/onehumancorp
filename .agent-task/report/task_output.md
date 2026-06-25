issue_title: "WhatsApp Business API Integration for OHC Unified Inbox"
issue_description: |
  # WhatsApp Business API Integration for OHC Unified Inbox

  ## Title
  Integrate WhatsApp Business Cloud API to enable unified messaging and agentic follow-ups

  ## Problem Statement
  For owners like Maya (Home Baker) and Carlos (Field Service), a massive portion of their leads and customer inquiries come through WhatsApp. Currently, these messages sit siloed on their personal or business phones. They have to manually reply, copy-paste booking details, and switch between OHC (to check inventory/schedule) and WhatsApp (to reply to the customer). If they miss a message, they lose a sale. OHC needs to ingest these messages into the "Work Triage" feed so our Customer Assistant agent can draft replies, book appointments, and capture deposits automatically without the owner ever having to switch apps.

  ## Research Report
  **Market Context:** WhatsApp is the dominant messaging platform in LATAM, EMEA, and parts of APAC. Small businesses run on WhatsApp Business. Competitors like HubSpot, Zoko, and Wati all offer deep WhatsApp integrations.
  **Tool Selected:** Meta Cloud API for WhatsApp Business.
  **Why Meta Cloud API:**
  - Direct integration, no third-party markup (unlike Twilio, which charges a premium per message).
  - Robust webhook support for real-time message ingestion.
  - Supports rich media (images for Maya's cakes, location pins for Carlos).
  - Cloud-hosted by Meta, meaning no local infrastructure needed for the API client.
  **Ease of Use for Owners:** The integration flow can be simplified using Facebook Login for Business, allowing owners to connect their WhatsApp Business number in a few clicks without dealing with access tokens directly.
  **Pricing:** Meta charges per conversation (24-hour window). Service/utility conversations are cheap, and marketing messages are tiered. The first 1,000 service conversations per month are often free, fitting perfectly with OHC's small business focus.

  ## Design Doc
  **Trigger:**
  - Owner connects their WhatsApp account via an "Integrations" page using Facebook Login for Business.
  - When a customer sends a WhatsApp message to the connected number, Meta sends a payload to an OHC webhook endpoint.

  **Actions:**
  - The OHC webhook receives the message, identifies the tenant, and creates/updates a Customer Record.
  - The message is pushed into the owner's unified "Work Triage" feed.
  - The AI Customer Assistant reads the message context, drafts a suggested reply (or takes an autonomous action like sending a booking link, based on owner settings).

  **User Experience:**
  - The owner sees a new WhatsApp icon in their Triage feed.
  - The owner can review the AI-drafted reply, edit it, and click "Send".
  - OHC sends the reply back through the Meta Cloud API, and the customer receives it on their WhatsApp natively.

  ## Implementation Prompt
  Implement the backend and frontend necessary to connect a WhatsApp Business account and route incoming/outgoing messages through OHC's Work Triage feed.

  **Acceptance Criteria:**
  - The owner can authenticate and link their WhatsApp Business number via the UI.
  - Incoming WhatsApp messages appear in real-time in the Work Triage feed.
  - The owner can type a reply (or approve an AI draft) in OHC, which is successfully delivered to the customer's WhatsApp.
  - Must correctly handle rich media (at least images) in both directions.
  - Must gracefully handle the 24-hour customer service window limitation of WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
