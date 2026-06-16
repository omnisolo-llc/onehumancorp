issue_title: "Integrate Twilio for WhatsApp Business to centralize customer conversations"
issue_description: |
  **Problem Statement**
  Small-business owners (like Maya the home baker or Carlos the field service owner) receive inquiries scattered across Instagram, SMS, and especially WhatsApp. They waste hours switching apps, missing leads, and copying context between tools. WhatsApp is the primary communication channel for billions, yet owners struggle to track orders, schedule services, and collect deposits manually on their personal phones.

  **Research Report**
  * **Ecosystem Analysis**: Competitors like Shopify (via Inbox), HubSpot, and WeCom heavily feature WhatsApp integrations because it's a critical sales channel.
  * **Tool Selected**: Twilio API for WhatsApp.
  * **Capabilities**: Twilio provides a reliable, well-documented API for sending and receiving WhatsApp messages. It supports rich media (images, PDFs for quotes), template messages for notifications (e.g., appointment reminders), and session-based conversational messaging.
  * **SaaS Viability & Pricing**: Twilio's pricing is pay-as-you-go per conversation (marketing, utility, authentication, service). The first 1,000 service conversations per month are often free or heavily discounted depending on the Meta WhatsApp Business Account (WABA) rules. It's scalable for multi-tenant (Cloud) using subaccounts, and works well for Standalone (local) setups with simple API keys.
  * **User-First Value Mapping**: A non-technical owner just authenticates their WhatsApp number. Instantly, all incoming WhatsApp messages appear in OHC's "Work Triage" feed alongside Instagram DMs and emails. The AI Assistant can read the message, understand the intent (e.g., "how much for a cake next Tuesday?"), draft a reply, and allow the owner to send it with one tap.

  **Design Doc**
  * **Trigger**: Owner links their WhatsApp Business account via OHC's settings. Incoming webhook from Twilio triggers OHC's Work Triage.
  * **Actions**:
    1. OHC creates/updates a Customer Profile and appends the message to the unified thread.
    2. The Customer & Relationship Assistant drafts a suggested reply based on past context and current availability.
    3. The owner reviews the draft in their feed, taps "Send", and OHC pushes the reply back via Twilio's API.
    4. Automated utility messages (e.g., "Your cake is out for delivery") can be triggered by Operations Assistant task completions.
  * **User Experience**: The owner sees a WhatsApp icon next to the message in their feed. They reply from OHC just like any other channel. No need to open the WhatsApp app.

  **Implementation Prompt**
  * Create a new integration module for Twilio WhatsApp.
  * Implement the OAuth/API key setup flow in the UI so the owner can securely link their Twilio account.
  * Expose an incoming webhook endpoint to receive messages from Twilio, parsing text and media.
  * Route incoming messages into the existing Work Triage pipeline.
  * Provide an outgoing service to send replies and template messages back to the customer.
  * **Acceptance Criteria**:
    1. Owner can configure Twilio credentials in the UI.
    2. Incoming WhatsApp messages appear in the OHC unified feed.
    3. Owner can reply from OHC, and the message is delivered to the customer's WhatsApp.
    4. All operations gracefully handle Twilio API errors and retry transient failures.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
