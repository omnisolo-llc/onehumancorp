issue_title: "Integrate Twilio for WhatsApp Business API for Agentic Work Triage"
issue_description: |
  ## Mission Queue Protocol: Twilio for WhatsApp Business API

  ### Problem Statement
  For owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart), customer demand doesn't arrive neatly in web forms—it arrives via direct messages on platforms like WhatsApp. Currently, these owners are forced to constantly context-switch between their personal/business WhatsApp apps and their operational tools, leading to missed leads, delayed replies, and chaotic manual order entry. They need their OHC assistant to natively read incoming WhatsApp inquiries, draft replies, parse orders/service requests, and synchronize everything into their unified work triage feed without requiring them to leave OHC.

  ### Research Report
  - **Tool Evaluated:** Twilio API for WhatsApp (which abstracts the Meta WhatsApp Cloud API complexities).
  - **Market Context:** Competitors like Tencent Workbuddy, WeCom, and DingTalk thrive because they embed deeply into the dominant regional chat ecosystems (WeChat). For the global and LATAM/EMEA SMB market, WhatsApp is the equivalent ecosystem.
  - **Ease of Use:** Twilio provides a robust, single API for SMS and WhatsApp, meaning we can unify SMS fallback and WhatsApp messaging under one integration pattern. The non-technical owner just needs to click "Connect WhatsApp" and follow the embedded signup flow.
  - **Pricing & Viability:** Twilio charges per conversation (business-initiated vs. user-initiated), which aligns well with OHC's potential value-based billing or usage pass-through. It supports webhook integration for real-time inbound messages, perfectly fitting OHC's async, event-driven agent architecture.
  - **Technical Capability:** Supports rich media (images for Maya's cake references, location pins for Carlos's service routes), message templates (for notifications), and interactive buttons (for booking confirmations).

  ### Design Doc
  - **Integration Trigger:** A new "Messaging Channels" settings page allows the owner to connect their Twilio WhatsApp Sender via OAuth/API key.
  - **Inbound Flow:** Twilio webhooks are routed to OHC's multi-tenant API. The payload is enqueued and picked up by the **Work Triage** agent.
  - **Agent Action:** The Work Triage agent reads the incoming WhatsApp message, identifies the customer, and matches it to existing context. The **Customer & Relationship Assistant** drafts a reply.
  - **Owner View:** The incoming message and drafted reply appear in the OHC assistant feed. The owner can tap "Approve & Send," which calls the Twilio API to dispatch the message back to the customer's WhatsApp.
  - **Operations:** If the message implies an order or booking, the Operations Assistant extracts structured data and creates a pending task/booking linked to the conversation.

  ### Implementation Prompt
  - Create a webhooks handler for Twilio incoming messages, storing the raw message linked to the appropriate `tenant_id` and customer profile.
  - Create a capability for the Customer Assistant to receive these inbound messages, load the customer's history, and draft a relevant reply in the OHC feed.
  - Provide an outbound API wrapper for Twilio to send the owner-approved draft back to the customer's WhatsApp.
  - Acceptance Criteria: An owner can receive a WhatsApp message from a customer, see it pop up in their OHC Work Triage feed with a drafted reply, and click "Send" to successfully deliver the response via Twilio.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []