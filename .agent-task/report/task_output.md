issue_title: "Scout: Tool Integration Research - WhatsApp/Twilio"
issue_description: |
  **Mission Queue Protocol Brief:**

  **Title:** Add Twilio WhatsApp Integration for Work Intake & Notifications
  **Problem Statement:** Maya (Home Baker) and Fatima (Food Cart Operator) receive a huge portion of their work and pre-orders through WhatsApp. Currently, they have to manually read these messages, type out replies on their personal phones, and copy the details into OHC. This slows down response times, splits their attention, and risks losing orders. They need their WhatsApp messages to show up in OHC's Work Triage feed automatically so the Assistant can help draft replies and coordinate the work.

  **Research Report:**
  * **Ecosystem Scraping:** Competitors like Shopify (via apps), HubSpot, and WeChat-based CRMs offer deep integration with messaging apps. WhatsApp is the de facto business communication tool in LATAM, EMEA, and parts of APAC.
  * **Community Mining:** Small business subreddits frequently ask for "WhatsApp CRM" or "WhatsApp to Task" bridges.
  * **Selected Tool:** Twilio API for WhatsApp.
  * **Capabilities & Limits:** Twilio provides a robust API for sending and receiving WhatsApp messages. It supports webhooks for incoming messages, template messages (for notifications outside the 24h window), and session-based freeform messaging.
  * **SaaS Viability:** Pricing is per-message, making it very viable to bundle or pass through to owners on paid tiers. It supports multi-tenant operation natively (we can segregate numbers or use Twilio's sub-accounts).

  **Design Doc:**
  * **Integration Trigger:** A new card in the "Tool Integrations" page in OHC will allow owners to connect their Twilio WhatsApp Sender.
  * **Data Flow:** Incoming messages trigger a webhook to OHC, which parses the sender and message, matching it to a customer profile.
  * **Owner View:** The message appears in the Work Triage feed. The AI Assistant drafts a reply or suggests creating a task/booking based on the message content.
  * **Response:** When the owner approves the draft, OHC sends it back via the Twilio API to the customer's WhatsApp.

  **Implementation Prompt:**
  1. Add a Twilio WhatsApp connection flow in the Integrations UI.
  2. Implement webhook handling to receive incoming WhatsApp messages and inject them into the Work Triage feed.
  3. Ensure the AI Assistant can read these messages and draft appropriate replies.
  4. Implement outgoing message sending via the Twilio API.
  5. The feature must be fully usable by a non-technical owner like Maya, who just wants to see messages in her feed, without dealing with API keys unless absolutely necessary (consider a streamlined OAuth or managed number flow in the future, but start with simple credential input for MVP).

  **Priority:** P0 (critical)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
