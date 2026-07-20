issue_title: "Integrate Twilio for Unified SMS & WhatsApp Messaging"
issue_description: |
  **Problem Statement:**
  Owners like Maya (Home Baker) and Carlos (Field Service) communicate with their customers primarily through WhatsApp and SMS. Juggling personal phones, missing messages, and lacking context on the customer leads to lost sales and poor service. OHC's Work Triage and Customer Assistant need to ingest these messages directly and reply through the same channels so the owner never misses a beat.

  **Research Report:**
  - **Tool Evaluated:** Twilio Messaging API (SMS & WhatsApp Business API).
  - **Market Need:** SMS and WhatsApp are the dominant communication channels for local services and boutique commerce. Competitors like WeCom and HubSpot prioritize unified inboxes.
  - **Ease of Use for Owners:** Non-technical operators will never see Twilio. They will simply click "Connect Business Number" in OHC, which handles the provisioning or linking under the hood.
  - **SaaS Viability:** Twilio offers flexible, scalable pay-as-you-go pricing (per conversation for WhatsApp, per segment for SMS). It supports multi-tenant SaaS environments well through subaccounts or unified webhook routing with tenant identification.
  - **Capabilities:** Reliable webhooks for incoming messages, rich media support (ideal for Maya receiving cake reference photos), and high deliverability.

  **Design Doc:**
  - **Integration Point:** OHC API webhook endpoints will receive incoming Twilio payload events.
  - **Trigger:** Customer sends an SMS or WhatsApp message to the OHC-managed number.
  - **Action:** OHC routes the webhook to the correct tenant, creates or updates a Customer profile, and drops the message into the Work Triage feed.
  - **User Experience:** The owner opens the OHC mobile app and sees the message at the top of their Triage feed. The Customer Assistant pre-drafts a reply based on previous orders and the owner's calendar. The owner clicks "Send," and OHC dispatches the reply via Twilio API.

  **Implementation Prompt:**
  - **User-Facing Outcome:** An owner can activate a business phone number in OHC for SMS and WhatsApp. When a customer texts or WhatsApps that number, the message instantly appears in the owner's OHC Triage Feed. The AI Assistant can draft responses, and the owner can reply seamlessly from the OHC interface.
  - **Acceptance Criteria:**
    - Owner can navigate to Settings > Communications and enable a business number.
    - Incoming SMS/WhatsApp messages create a visible Triage item in the UI.
    - AI-drafted replies are available for incoming messages.
    - Replies approved and sent from OHC are successfully delivered to the customer's device.
    - Full end-to-end flow is verified with Playwright UI tests in the live Docker Compose stack using test credentials.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []