issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Cloud API"
issue_description: |
  **Title**: 🔍 Scout: Tool Integration Research - WhatsApp Cloud API

  **Problem Statement**:
  Maya (Home Baker), Carlos (Field Service Owner), and Fatima (Food Cart Operator) receive a substantial portion of their work intake and customer inquiries through WhatsApp. Currently, these messages exist outside of any centralized system, causing missed leads, scattered context, and manual data entry. Non-technical owners need WhatsApp to behave like any other intake channel inside OHC, with the Customer Assistant capable of drafting replies and Work Triage turning DMs into actionable tasks.

  **Research Report**:
  - **Tool**: WhatsApp Cloud API (hosted by Meta).
  - **Market Need**: WhatsApp is the dominant business communication tool in LATAM, EMEA, and APAC, and is rapidly growing in the US for small business communication. A large segment of our target personas relies on WhatsApp DMs to run their businesses. Competitors like WeCom and the WhatsApp Business app handle direct messaging but lack full operational and automated integration into the business's core workflows.
  - **Capabilities & Limits**: The Cloud API supports receiving incoming messages (via webhooks), sending free-form replies (within a 24-hour customer service window), sending pre-approved template messages, and handling rich media (images, PDFs). Webhooks are reliable and well-documented. The OAuth flow (Embedded Signup) allows non-technical users to connect their business numbers directly without needing developer portal access.
  - **SaaS Viability**: WhatsApp charges per conversation, but offers a free tier of 1,000 service conversations per month. This structure fits perfectly into a multi-tenant SaaS model, where we can either pass through costs for high-volume users or include the baseline tier in OHC's subscription. It operates robustly in Cloud environments.

  **Design Doc**:
  - **User Experience**: The owner navigates to an "Integrations" section and clicks "Connect WhatsApp". They are guided through the Meta Embedded Signup flow to link their phone number. Once linked, incoming WhatsApp messages flow directly into the **Work Triage** feed. The owner sees the message alongside any existing customer history. The **Customer Assistant** automatically drafts a contextual reply or proposes a next action (e.g., generating a quote for a repair request). The owner reviews, adjusts if necessary, and hits "Send", routing the message back to the customer's WhatsApp.
  - **Integration Points**:
    - OHC Webhook listener to ingest incoming messages and message status updates in real-time.
    - Embedded Signup OAuth flow to handle seamless, secure account connection.
    - OHC API client logic to dispatch text and media replies back through the Meta Graph API.
  - **Principles**: Absolutely no technical jargon (no mentions of API keys, webhooks, or tokens). The interface should just focus on "Connect WhatsApp" and handling the ensuing conversations.

  **Implementation Prompt**:
  - Implement a simple "Connect WhatsApp" UI flow utilizing the Meta Embedded Signup for seamless user onboarding.
  - Expose a secure webhook endpoint to receive incoming WhatsApp messages and map them into the OHC Work Triage system.
  - Enable the Customer Assistant to draft replies for WhatsApp messages, and allow the owner to send replies back from the OHC UI.
  - Build an E2E Playwright test simulating an incoming WhatsApp webhook payload and verifying that an outbound reply can be triggered by the owner via the UI.

  **Priority**: P1 (High)

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
