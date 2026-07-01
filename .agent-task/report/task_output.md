issue_title: "WhatsApp Business API Integration (Meta Cloud API)"
issue_description: |
  **Title**: WhatsApp Business API Integration (Meta Cloud API)

  **Problem Statement**:
  Based on our market research (`ohc_smb_market_report.md`), "Omnichannel Chaos" is the #3 pain point for SMBs (14%), with users complaining "I missed an order because it was in my DMs." Additionally, geographic expansion into LATAM is a priority, where WhatsApp is the dominant communication channel for commerce. OHC's target users, like Maya (home baker) and Carlos (field service owner), heavily rely on instant messaging for intake and customer relationships. Currently, if an owner operates primarily on WhatsApp, they lack a unified assistant-led flow in OHC to capture those leads and orders seamlessly, forcing them to jump between apps.

  **Research Report**:
  - **Tool Evaluated**: WhatsApp Business API (specifically Meta's Cloud API hosted by Meta).
  - **Relevance**: WhatsApp has over 2 billion active users globally, and in regions like LATAM and India, it acts as the primary interface between businesses and consumers.
  - **Usability for Owners**: Direct integration means non-technical owners can connect their WhatsApp Business number to OHC. They won’t need to learn the Meta Developer portal; OHC handles the API handshakes via OAuth or embedded signup.
  - **Pricing/Viability**: Meta offers the Cloud API with no server hosting costs. Pricing is conversation-based (user-initiated vs. business-initiated). For OHC (SaaS), this is highly viable, as we can pass on costs or include a tier that covers a base amount of conversations.
  - **Capabilities**: Webhooks for incoming messages, rich media support (images for product catalogs/invoices), interactive messages (buttons, lists for booking or selecting offers).
  - **Market Position**: Competitors like Shopify offer WhatsApp integration via third-party apps, which adds friction and cost. OHC offering this natively feeds directly into our "Invisible AI Agents" manifesto (e.g., Auto-Reply DM Agent).

  **Design Doc**:
  - **Triggers**:
    - *Inbound*: Customer sends a message to the owner's WhatsApp number. OHC receives a webhook from Meta.
    - *Outbound*: Owner (or an OHC Agent) initiates a conversation (e.g., booking reminder, payment link, cart recovery) via OHC's unified inbox.
  - **Actions**:
    - The `Work Triage` agent processes inbound WhatsApp messages, matching them to existing customers or creating new leads.
    - If it's a new order inquiry, the `Customer Assistant` agent drafts a reply or an offer summary using interactive WhatsApp buttons (e.g., "Accept Quote", "Decline").
  - **User Experience**:
    - The owner sees WhatsApp messages flowing directly into their OHC command center alongside emails and IG DMs.
    - Setup is a simple "Connect WhatsApp" button in OHC Settings, guiding them through the Meta embedded signup flow.

  **Implementation Prompt**:
  Implement the backend webhook handler to receive incoming messages from the WhatsApp Cloud API and route them into the OHC unified inbox as tasks/messages.
  Create the user-facing "Connect WhatsApp" setup flow in the frontend, ensuring the connection state is visible and clearly indicates success or failure.
  Verify that when a test WhatsApp message is sent, it appears in the owner's unified feed on a 375px mobile screen, and that the owner can type a reply in the OHC UI which successfully sends back to the WhatsApp user. Acceptance criteria include full E2E testing of the message receive-and-reply loop using test credentials, with no mock data in the UI.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
