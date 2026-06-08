issue_title: "Integrate Twilio for WhatsApp Business Messaging"
issue_description: |
  ## Title
  Integrate Twilio for WhatsApp Business Messaging

  ## Problem Statement
  Many small business owners and operators (like Maya the home baker and Carlos the field service owner) rely heavily on WhatsApp to communicate with their customers, receive inquiries, and negotiate orders. However, these messages live on their personal phones, isolated from their primary work tools. This forces them to constantly switch between WhatsApp and their booking or order management systems. When an owner is busy, messages fall through the cracks, leading to lost revenue and poor customer experiences. Owners need a way to manage WhatsApp conversations directly within the OHC Assistant, so that AI can help draft replies, pull in customer history, and turn inquiries into actionable tasks or quotes without manual data entry.

  ## Research Report
  - **Ecosystem & Community Needs:** WhatsApp is the dominant messaging platform in many global markets (e.g., LATAM, India, parts of Europe) and is heavily used for conversational commerce. Competitors like HubSpot, Zendesk, and specialized CRMs offer WhatsApp integrations. SMB operators frequently request unified messaging to handle customer support and sales in one place.
  - **Tool Evaluation (Twilio API for WhatsApp):** Twilio provides a robust, scalable API for WhatsApp Business. It handles the complexities of WhatsApp template approvals, opt-ins, and session windows (24-hour customer care window).
  - **Ease of Use for Non-Technical Users:** Twilio itself is a developer tool, but its integration into OHC will completely abstract this away. The owner simply connects their WhatsApp Business account (via an OAuth-like flow or embedded signup) and begins receiving messages in OHC's unified Work Triage feed.
  - **Pricing & SaaS Viability:** Twilio charges per conversation (user-initiated or business-initiated) rather than per message, which aligns well with SMB transaction models. It supports both Cloud (multi-tenant) operations via standard webhook routing and can be adapted for standalone setups. Twilio is an industry standard with high reliability and comprehensive documentation.

  ## Design Doc
  - **Trigger:** When a customer sends a message to the owner's WhatsApp Business number, Twilio fires a webhook to OHC.
  - **Action:** OHC ingests the message, associates it with an existing customer profile (or creates a new one), and surfaces it in the Work Triage feed. The Customer & Relationship Assistant analyzes the message, drafts a context-aware reply, and presents it to the owner for approval or immediate dispatch.
  - **User Visibility:** The owner sees a unified chat interface within OHC. They can read the incoming WhatsApp message, edit the AI-drafted reply, and hit "Send." OHC routes the reply back through the Twilio API to the customer's WhatsApp. The owner can also initiate conversations using pre-approved WhatsApp templates directly from a customer profile or booking detail page.

  ## Implementation Prompt
  Implement a Twilio WhatsApp Business integration that allows OHC users to connect their WhatsApp business number. The integration should capture incoming WhatsApp messages and display them in the unified Work Triage feed. The AI assistant should be able to read these messages, suggest replies, and allow the user to send messages back to the customer's WhatsApp directly from the OHC interface. The setup process for the owner should be straightforward, ideally guiding them through the Twilio/WhatsApp connection process without requiring them to copy-paste raw API keys if possible, or providing crystal-clear instructions if manual key entry is required. Ensure the UI clearly indicates when a message was received via WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
