issue_title: "Integrate WhatsApp Business API for Conversational Work Intake & Customer Relationships"
issue_description: |
  ## Problem Statement
  Small-business owners and operators—like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart)—rely heavily on conversational channels like WhatsApp to communicate with customers. Currently, inquiries, custom orders, service requests, and follow-ups received on WhatsApp remain disconnected from OneHumanCorp's unified work feed. This forces operators to context-switch between their phone and OHC, risking missed messages, dropped leads, and double-entry of booking/payment details. They need a single, assistant-led feed where WhatsApp messages automatically turn into trackable tasks, draft replies, and actionable quotes without requiring technical setup.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **Ecosystem Demand:** Competitors like Tencent Workbuddy, WeCom, DingTalk, and HubSpot treat social messaging (especially WhatsApp/WeChat) as a primary data source. WhatsApp is the de facto business communication tool in LATAM, EMEA, and APAC, making it an indispensable integration target.
  - **Tool Evaluated:** WhatsApp Business Platform (Cloud API by Meta).
  - **Capabilities:** Allows programmatic sending/receiving of text, media, and interactive messages (buttons, lists). Supports webhooks for real-time message intake.
  - **SaaS Viability & Pricing:**
    - Operates entirely in the Cloud. Meta's Cloud API removes the need to host local servers, aligning perfectly with OHC's multi-tenant architecture.
    - **Pricing:** Conversation-based pricing. The first 1,000 service conversations per month are free, which heavily favors our target small-business owners.
  - **Usability for Non-Technical Operators:** When abstracted by OHC, the operator never sees an API token or webhook URL. They simply "Connect WhatsApp" via an embedded OAuth/Business Manager flow (or a simplified partner flow), and their WhatsApp DMs instantly populate the OHC Work Triage feed.

  ## Design Doc
  **Integration Strategy:**
  1. **Work Intake (Webhook Listener):** Implement a secure webhook endpoint to receive inbound WhatsApp messages. OHC will parse these payloads and route them to the specific tenant's Work Triage feed based on the registered phone number.
  2. **AI Coordination:**
     - **Customer Assistant:** Automatically drafts context-aware replies for the owner based on historical data and current availability.
     - **Operations Assistant:** Extracts intent (e.g., "cake for Saturday", "repair my AC") to propose bookings, tasks, or quote generation.
  3. **User Experience (The Owner Feed):** The operator sees incoming WhatsApp messages in their unified OHC feed. They can click "Approve Reply", "Send Quote", or type manually. OHC translates these actions back into outgoing Meta Cloud API calls.
  4. **Architecture Fit:** Requires mapping WhatsApp IDs to OHC `Customer` records and securely storing Meta API credentials per tenant.

  ## Implementation Prompt
  1. **Webhook Handler:** Create a fast, robust Meta Cloud API webhook listener that verifies signatures and parses inbound text/media messages.
  2. **Tenant Routing:** Map the receiving WhatsApp number to the correct OHC tenant and store the message in the `Customer Relationships` context.
  3. **Agent Integration:** Wire the inbound message event to the `Customer Assistant` agent so it can generate a draft reply and suggest operational actions (e.g., "Draft: We can do Saturday. Deposit is $50.").
  4. **Outbound API Client:** Implement an outbound API client to send owner-approved replies or interactive messages (e.g., payment links) back to the customer's WhatsApp.
  5. **Acceptance Criteria:** A non-technical owner can connect their WhatsApp, receive a customer message in the OHC UI, see an AI-drafted reply, and click "Send" to deliver the message back to the customer—all without leaving OHC.

  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
