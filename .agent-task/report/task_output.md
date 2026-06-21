issue_title: "🔍 Scout: Tool Integration Research - WhatsApp Cloud API (Meta)"
issue_description: |
  # Mission Queue Protocol: Tool Integration Research - WhatsApp Cloud API (Meta)

  ## Problem Statement
  For personas like **Maya (Home Baker)**, **Carlos (Field Service)**, and **Fatima (Food Cart)**, WhatsApp is the primary channel for customer communication, inquiries, and orders. Currently, these interactions are isolated on their physical phones or separate apps. Because OHC lacks visibility into WhatsApp, the "Work Triage" and "Customer Assistant" capabilities are severely crippled. The AI Assistant cannot see inbound demand, cannot link requests to past customer context, and cannot draft replies, forcing the owner to manually copy-paste context between OHC and WhatsApp.

  ## Research Report
  - **Market Context:** WhatsApp is the dominant communication platform for SMBs globally (especially in LATAM, EMEA, and Asia-Pacific). Competitors like WeCom (Tencent) and DingTalk seamlessly integrate native chat (WeChat/DingTalk), which is the cornerstone of their utility. Shopify, Wix, and HubSpot all feature top-ranking WhatsApp integrations in their app stores.
  - **Tool Evaluated:** WhatsApp Cloud API (hosted by Meta).
  - **Ease of Use for Owners:** Non-technical owners can connect their WhatsApp Business accounts using Meta's "Embedded Signup" flow, which is a standard pop-up OAuth-like experience. No technical configuration of webhooks is required from the owner's side.
  - **Pricing & Viability:** Meta provides 1,000 free user-initiated service conversations per month, which easily covers the volume for small operators like Maya or Leo. Beyond that, it uses a pay-as-you-go per-conversation model. Operating in the cloud (Meta-hosted) removes the old requirement of hosting local WhatsApp containers, making it perfect for OHC's multi-tenant architecture.

  ## Design Doc
  - **Integration Point:** Unified Work Triage Feed & Customer Relationships.
  - **Triggers:** A customer sends a WhatsApp message (text, image, or voice note) to the owner's business number.
  - **Assistant Action:** OHC receives the webhook, identifies the customer by phone number, pulls up their history (e.g., past cakes ordered, previous service routes), and places the message at the top of the owner's daily feed. The Customer Assistant pre-drafts a context-aware reply or suggests an action (like "Create Quote" or "Schedule Visit").
  - **User Experience:** The owner opens the OHC app on their 375px mobile screen. They see a single unified feed. A new WhatsApp inquiry appears with an AI-drafted reply. The owner simply taps "Approve & Send" or edits the text. The customer receives a native WhatsApp message. The owner never had to open the Meta app.

  ## Implementation Prompt
  - Create a "Connect WhatsApp" setup card in the owner's settings using the Meta Embedded Signup flow.
  - Establish a unified webhook receiver for Meta to ingest inbound WhatsApp text and media.
  - Map inbound phone numbers to OHC Customer profiles to provide relationship context.
  - Display incoming WhatsApp messages in the unified Assistant Work Triage feed alongside Instagram DMs and emails.
  - Allow the AI to draft replies that the owner can review, edit, and send directly from the OHC interface.
  - **Acceptance Criteria:** A non-technical owner can link their number, receive a WhatsApp message from a customer in their OHC feed, see an AI-drafted reply, and successfully reply to the customer's phone without leaving the OHC app. The UI must handle Meta's 24-hour customer service window restrictions gracefully (e.g., warning the owner if they try to reply too late).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
