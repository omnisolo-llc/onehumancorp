issue_title: "Integration: Twilio for WhatsApp Business (Work Triage & Customer Assistant)"
issue_description: |
  # Title: Integration: Twilio for WhatsApp Business (Work Triage & Customer Assistant)

  ## Problem Statement
  For owners like Maya (custom cakes), Carlos (field service), and Fatima (food cart), a huge portion of customer inquiries, orders, and service requests come through WhatsApp. Without a centralized assistant, they spend hours manually switching between their personal WhatsApp and other tools, losing track of orders, forgetting to follow up, and missing revenue. They need OHC's Work Triage and Customer Assistant to seamlessly intercept, categorize, and draft replies to WhatsApp messages so they can manage all demand from one place.

  ## Research Report
  - **Ecosystem Scraping:** Competitors like WeCom, DingTalk, and Zendesk all offer deep messaging integrations. WhatsApp Business API is the global standard for B2C messaging in LATAM, EMEA, and parts of APAC/NA.
  - **Tool Evaluation (Twilio for WhatsApp):** Twilio provides a robust, scalable API for WhatsApp Business. It abstracts away Meta's complex Cloud API requirements, offers excellent webhook reliability, and supports rich media (images for cakes, location pins for field service).
  - **SaaS Viability:** Twilio's pay-as-you-go pricing model is highly viable for multi-tenant SaaS. OHC can manage a central Twilio account and map Twilio sender numbers to specific OHC tenants, or allow Bring-Your-Own-Twilio-Credentials for standalone/private deployments.
  - **User-First Value:** The owner never has to understand "webhooks" or "APIs". They simply connect their WhatsApp number, and suddenly Maya's cake requests appear in her OHC Work Feed with drafted replies and automated order tags.

  ## Design Doc
  - **Trigger:** A webhook from Twilio when a new WhatsApp message arrives to a tenant's registered number.
  - **Action:** The OHC integration service receives the payload, resolves it to the correct tenant (via recipient number mapping), and drops the message into the AI Job Queue for the Work Triage agent. The Customer Assistant drafts a reply based on past context and tenant memory.
  - **User Visibility:** The owner sees the message in their central Work Feed, labeled "WhatsApp". They see the AI's drafted reply and can tap "Approve & Send" or edit it. The response is routed back out via the Twilio API.

  ## Implementation Prompt
  - Create the necessary database tables to link a `tenant_id` to a Twilio WhatsApp sender number.
  - Implement a secure webhook endpoint to receive inbound messages from Twilio, verifying the Twilio signature.
  - Normalize inbound WhatsApp messages (text and image media) into OHC's standard unified message format and insert them into the Work Triage queue.
  - Implement an outbound sender client that uses the Twilio API to reply to WhatsApp conversations.
  - Add a settings UI in the Flutter frontend for an owner to configure their Twilio credentials or provision a number, ensuring the flow is simple and non-technical.
  - Acceptance Criteria: An owner can receive a WhatsApp message, see it in their OHC Work Feed, and reply successfully, all without leaving the OHC interface.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
