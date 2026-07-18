issue_title: "Integrate Twilio WhatsApp Business API for Omni-Channel Inbox"
issue_description: |
  **Title**: Integrate Twilio WhatsApp Business API for Omni-Channel Inbox

  **Problem Statement**:
  Many of our owner/operator personas, such as Maya (Home Baker) and Carlos (Field Service Owner), receive a significant portion of their customer inquiries, booking requests, and service updates via WhatsApp. Currently, these owners have to switch constantly between their personal/business WhatsApp app on their phone and the OHC assistant. This context switching causes missed leads, fragmented customer history, and delayed responses. They need OHC to read their WhatsApp messages and draft replies directly in their main daily feed, so they don't have to juggle multiple apps to handle customer demand.

  **Research Report**:
  - **Ecosystem Scraping**: Competitors like Tencent Workbuddy, WeCom, and HubSpot all offer deep integration with popular messaging platforms. WhatsApp is the dominant messaging app in many global markets (LATAM, EU, India) and is heavily used by SMBs in the US for direct customer interaction.
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Ease of Use**: For the non-technical owner, Twilio provides a seamless integration. Once the owner links their WhatsApp Business account via a guided OAuth-like flow, OHC handles everything else behind the scenes.
  - **Pricing**: Twilio charges per conversation (user-initiated vs. business-initiated). It offers a generous free tier for the first 1,000 service conversations per month, which perfectly fits our standalone/local and small-cloud tenants.
  - **Capabilities**: Supports rich media, quick replies, read receipts, and session-based messaging, which aligns perfectly with OHC's Customer Assistant drafting replies and proposing next actions. It is highly reliable with robust webhooks.

  **Design Doc**:
  - **Trigger**: The integration is triggered when a customer sends a WhatsApp message to the owner's connected business number. Twilio fires a webhook to OHC.
  - **Action**: OHC's Work Triage capability ingests the message, identifies the customer from existing CRM data, and adds the message to the owner's unified feed. The Customer Assistant automatically drafts a suggested reply based on the context (e.g., answering a cake pricing question or confirming a service time).
  - **User Experience**: The owner sees the new WhatsApp message in their daily OHC feed alongside emails and web forms. They see the AI-drafted reply and can simply tap "Approve and Send" or edit it. The owner never has to open the WhatsApp app.
  - **Setup**: In the "Channels" settings, the owner clicks "Connect WhatsApp" and follows a simple guided flow to link their Meta/WhatsApp Business account. Advanced webhook configurations are entirely hidden from the user.

  **Implementation Prompt**:
  "Implement the Twilio WhatsApp Business API integration so that incoming WhatsApp messages appear in the owner's unified feed.
  1. Create a simple setup flow for the owner to connect their WhatsApp Business account via Twilio.
  2. Implement webhook ingestion to receive incoming WhatsApp messages and map them to the correct customer profile in OHC.
  3. Ensure the Customer Assistant automatically drafts a contextual reply for incoming WhatsApp messages and surfaces it to the owner.
  4. Allow the owner to review, edit, and send the reply back to the customer via WhatsApp directly from the OHC UI.
  5. The entire experience must be mobile-friendly and work seamlessly on a 375px screen."

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
