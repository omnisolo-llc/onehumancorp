issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Business via Twilio"
issue_description: |
  ## Title
  Integrate Twilio WhatsApp Business API for Unified Customer Messaging

  ## Problem Statement
  Small business owners like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on direct messaging platforms to capture leads, answer questions, and coordinate services. WhatsApp is a dominant communication channel for these personas. Currently, owners must manually juggle their personal or business WhatsApp app alongside OHC, leading to fragmented communication, missed context, delayed responses, and lost revenue. They need a way for OHC's Customer & Relationship Assistant to automatically ingest WhatsApp messages, maintain customer context, and draft or send replies directly from the OHC unified work feed.

  ## Research Report
  - **Ecosystem Scraping & Community Mining**: Competitors like WeCom, HubSpot, and Wix already offer deep WhatsApp integrations. Across r/smallbusiness and Trustpilot, operators frequently request unified inboxes that bring WhatsApp, Instagram, and SMS into one place.
  - **Tool Evaluated**: Twilio API for WhatsApp.
  - **Ease of Use for Non-Technical Users**: The owner simply authenticates their WhatsApp Business account via an embedded OAuth/signup flow. OHC handles the webhooks and API interactions behind the scenes. The owner does not need to configure API keys or webhooks directly.
  - **Pricing & Reputation**: Twilio is highly reputable, offering pay-as-you-go pricing with a solid free tier for testing. WhatsApp conversation-based pricing is affordable for small businesses. Twilio's webhooks are reliable, and its API is well-documented.
  - **SaaS Viability**: Twilio supports multi-tenant Cloud environments effectively.

  ## Design Doc
  - **Integration Point**: The OHC Customer & Relationship Assistant and Work Triage capabilities.
  - **User Experience**:
    - The owner sees a "Connect WhatsApp" option in their integrations settings.
    - Upon connection, incoming WhatsApp messages flow directly into the OHC unified feed.
    - The AI Assistant identifies returning customers, pulls up their order/booking history, and drafts suggested replies for the owner to approve with one tap.
    - The owner can reply directly from the OHC shell (mobile or desktop), and the message is delivered via WhatsApp.
  - **Triggers & Actions**: Twilio webhooks trigger message ingestion into OHC. OHC actions push outbound messages through the Twilio API.

  ## Implementation Prompt
  Implement the Twilio WhatsApp Business API integration so that non-technical owners can easily connect their WhatsApp accounts. The integration must allow incoming messages to appear in the OHC unified Work Triage feed. The Customer & Relationship Assistant should be able to draft replies to these messages, and the owner must be able to send responses back to the customer's WhatsApp directly from the OHC UI. Ensure the connection flow is simple and handles proper routing of messages based on the tenant.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
