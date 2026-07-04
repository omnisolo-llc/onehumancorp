issue_title: "Integrate WhatsApp Cloud API for Direct Customer Work Triage & Automated Replies"
issue_description: |
  ### Title
  Integrate WhatsApp Cloud API for Direct Customer Work Triage & Automated Replies

  ### Problem Statement
  Many small-business owners in OHC's target demographic—like Maya (Home Baker) taking custom orders, Fatima (Food Cart Operator) managing pickups, and Carlos (Field Service) chatting with clients—use WhatsApp as their primary business communication channel. Currently, this creates a split-brain problem. Owners have to manually monitor their WhatsApp app, copy details into OHC to create tasks or bookings, and flip back to WhatsApp to reply. This slows them down, drops leads, and defeats the "Ask one assistant" promise. They need OHC's Work Triage and Customer Assistant to see incoming WhatsApp messages, suggest replies, and turn chat requests directly into structured work (quotes, tasks, appointments).

  ### Research Report
  **Market & Ecosystem Demand:**
  * **Ubiquity:** In Latin America, EMEA, and India (and increasingly North America), WhatsApp Business is the standard for B2C communication. Competitors like HubSpot, Shopify Inbox, and DingTalk heavily feature messaging API integrations.
  * **Owner Pain:** Owners are overwhelmed by "Can you deliver on Friday?" or "What's the price for X?" DMs. They want the assistant to instantly draft answers based on their calendar and pricing docs.

  **Selected Tool Evaluation:** WhatsApp Cloud API (Meta)
  * **Pricing & SaaS Viability:** Free tier covers 1,000 service conversations per month. OHC can build a multi-tenant cloud offering where owners link their WhatsApp Business Accounts (WABA) via Embedded Signup. Pricing is usage-based (by conversation), which is highly scalable for SaaS.
  * **Capabilities:** Supports rich messaging (buttons, lists, catalog items). OHC can send proactive notifications (e.g., "Your cake is ready for pickup" or "Carlos is 10 mins away") and handle inbound queries.
  * **Local/Standalone Constraints:** Cloud API requires Meta hosting. For local/private deployments, users would need their own Meta Developer App or we could fall back to simpler WhatsApp web scraping tools (like Baileys/whatsapp-web.js), though the official Cloud API is preferred for reliability.
  * **Ease of Use:** Through Meta's Embedded Signup, non-technical owners just click "Connect WhatsApp," log into their Facebook account, and link their number. No API keys to copy-paste.

  ### Design Doc
  **Trigger:**
  1. **Inbound:** A customer messages the owner's WABA number. WhatsApp triggers a webhook to OHC's backend.
  2. **Outbound:** A booking state changes in OHC (e.g., "Ready for Pickup"). OHC triggers a templated WhatsApp message.

  **Actions:**
  1. **Work Triage Update:** The webhook payload is parsed. If it's an existing customer, the message is appended to their thread. The AI Work Triage capability is invoked to evaluate if this requires urgent owner attention or if it's a routine inquiry.
  2. **Drafting Replies:** The Customer Assistant drafts a suggested reply based on the owner's knowledge base.
  3. **Owner UI:** The owner sees a new item in their feed: "Maya, 3 new WhatsApp inquiries about Saturday cakes. 2 drafts are ready to send." The owner taps 'Approve' to send the WhatsApp message directly from OHC.

  ### Implementation Prompt
  1. Implement the OAuth / Meta Embedded Signup flow so an owner can link their WhatsApp Business Account to their OHC tenant.
  2. Build a reliable webhook receiver (`/api/webhooks/whatsapp`) that accepts incoming messages, verifies the Meta signature, and correctly associates the message with the tenant using the WABA ID.
  3. Route incoming messages into the existing Work Triage pipeline, displaying them in the owner's Feed UI.
  4. Allow the owner to type a reply (or approve an AI-drafted reply) in the OHC UI, which sends the message back via the WhatsApp Cloud API.
  5. Ensure the UI gracefully handles the 24-hour customer service window limitation of WhatsApp (e.g., disabling the free-text reply box if 24 hours have passed).

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []