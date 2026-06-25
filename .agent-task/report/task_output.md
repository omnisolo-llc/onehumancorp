issue_title: "Integrate Twilio WhatsApp Business API for Unified Work Triage"
issue_description: |
  ## Problem Statement
  Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart) receive a massive volume of customer inquiries, orders, and service requests via WhatsApp. Currently, these messages sit in their personal or business WhatsApp apps, disconnected from their scheduling, quoting, and operational tools in OHC. This causes missed leads, delayed responses, and forces the owner to manually copy context from WhatsApp into their daily work systems.

  ## Research Report
  - **Market Need**: In many regions (LATAM, Europe, Asia), WhatsApp is the primary communication channel for small businesses. Competitors like Shopify Inbox, WeCom, and HubSpot already offer unified WhatsApp inbox capabilities.
  - **Tool Evaluation (Twilio WhatsApp Business API)**: Twilio provides a robust, scalable WhatsApp Business API. It abstracts away Meta's underlying infrastructure and provides reliable webhooks for inbound messages, as well as simple REST APIs for outbound messages.
  - **Pricing**: Twilio charges a fractional markup on Meta's conversation-based pricing. Meta offers the first 1,000 service conversations per month for free, making it extremely cost-effective for small owners.
  - **Ease of Use for Owners**: The non-technical owner will never interact with Twilio directly. OHC will handle the infrastructure. The owner simply connects their WhatsApp number via an embedded signup flow.

  ## Design Doc
  - **Integration Point**: OHC Tenant Settings -> Messaging Integrations -> "Connect WhatsApp".
  - **Trigger**: Customer sends a message to the owner's WhatsApp number.
  - **Action**: Twilio sends a webhook to OHC's backend. OHC parses the payload and either creates a new `Conversation` thread in the Work Triage feed or appends to an existing one. The AI Customer Assistant drafts a suggested reply based on the business context.
  - **User View**: The owner opens OHC and sees WhatsApp messages unified in their Work Triage feed alongside other demand channels (like Instagram DMs). They can review the AI-drafted reply, edit it, and click "Send", which routes the message back through Twilio to the customer's WhatsApp.

  ## Implementation Prompt
  - Create a "WhatsApp Business" integration option in the workspace settings.
  - Implement a webhook handler to securely receive and validate incoming message payloads from Twilio.
  - Create an outbound messaging service that uses Twilio's REST API to send text, images, and interactive elements (like buttons) to WhatsApp.
  - Store conversation history in the OHC unified message model so the AI Assistant has full context for drafting replies.
  - Ensure the setup experience requires zero technical configuration (no manual copying of API keys) for the owner.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
