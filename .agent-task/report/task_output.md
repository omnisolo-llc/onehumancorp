issue_title: "WhatsApp Business API via Twilio Integration"
issue_description: |
  ## Title
  Omni-Channel Customer Triage: WhatsApp Business API Integration via Twilio

  ## Problem Statement
  For non-technical owners like Maya (Home Baker) and Fatima (Food Cart Operator), WhatsApp is the primary operating system for their business. Customers send orders, ask about availability, and request support via WhatsApp. However, this creates a fragmented workflow: owners must constantly switch between their OHC dashboard (where their schedule and inventory live) and their WhatsApp app on their phone. This context-switching leads to lost leads, delayed responses, and mental fatigue. They need OHC to ingest WhatsApp messages directly into their "Work Triage" feed, allow the AI to draft context-aware replies based on OHC data, and let the owner approve and send responses without leaving the OHC interface.

  ## Research Report
  **Track 1: Dynamic Integration & Market Need Discovery**
  - **Ecosystem Scraping:** Competitor platforms like WeCom (Tencent) and DingTalk have built-in chat ecosystems. Western counterparts like Shopify and Wix see WhatsApp integration plugins (e.g., Zoko, Wati) constantly trending in their app stores. This indicates that native WhatsApp support is a critical feature for global small businesses.
  - **Community Mining:** Across r/smallbusiness and local commerce forums, a recurrent complaint is the inability to sync WhatsApp Business chats with centralized CRMs and task managers. Tools that bridge this gap are highly valued.
  - **Integration Target:** Twilio's Programmable Messaging API for WhatsApp. It abstracts Meta's complex Cloud API requirements and allows for future expansion into SMS and Email through a single unified API pattern.

  **Track 2: Selected Tool Deep-Dive Evaluation**
  - **User-First Value Mapping:** Maya will no longer forget a custom cake order buried in her WhatsApp chat. When a customer messages her on WhatsApp, OHC will intercept it, create a task in the Work Triage feed, and use the Customer Assistant to draft a reply (e.g., confirming the cake flavor and sending a payment link). Maya just taps "Approve" in OHC.
  - **Capabilities & Limits:** Twilio supports inbound webhooks (essential for real-time triage) and outbound session messages (free-form replies within 24 hours of a user's message). After 24 hours, WhatsApp requires pre-approved template messages. OHC will need to handle this 24-hour session window gracefully by either prompting the owner to reply in time or guiding them to use an approved template.
  - **SaaS Viability:** Twilio offers flexible pay-as-you-go pricing (approx. $0.005 to $0.015 per message depending on region), which is highly scalable for a multi-tenant Cloud environment. For Standalone local deployments, owners can easily supply their own Twilio Account SID and Auth Token.

  ## Design Doc
  - **Trigger:** A customer sends a WhatsApp message to the business. Twilio receives it and fires a webhook to OHC's backend.
  - **Action:**
    1. OHC receives the payload and matches the sender's phone number to an existing customer record (or creates a new lead).
    2. The message is pushed into the owner's "Work Triage" feed as a high-priority item.
    3. The Customer Assistant (AI) reads the message context and drafts a suggested reply.
  - **User Interface:** The OHC dashboard displays a unified chat interface within the Work Triage feed. The owner sees the customer's message, the AI's drafted response, and a clear "Approve & Send" button. The UI clearly displays the 24-hour reply window status.

  ## Implementation Prompt
  **User-Facing Outcome:**
  The owner should be able to connect their Twilio/WhatsApp account in the OHC integrations settings. Once connected, all incoming WhatsApp messages will appear in the unified Work Triage feed. The AI Customer Assistant will automatically read these messages and draft responses. The owner can review, edit, and send these responses directly from OHC, which will be delivered seamlessly to the customer's WhatsApp.

  **Acceptance Criteria:**
  1. The Integrations UI allows an owner to input Twilio credentials (or use OHC managed credentials in Cloud mode) to connect WhatsApp.
  2. Inbound WhatsApp messages via webhook create actionable items in the Work Triage feed.
  3. The Customer Assistant successfully drafts context-aware replies for inbound WhatsApp messages.
  4. The owner can edit and send a reply from the OHC interface, routing it back through Twilio to the customer.
  5. The UI visually indicates if the 24-hour WhatsApp session window is closing, preventing failed sends.
  6. The chat interface works perfectly on a 375px mobile screen with native-feeling touch targets (44x44px minimum).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
