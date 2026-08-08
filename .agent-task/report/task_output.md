issue_title: "Native Omnichannel Chat: Implement WhatsApp Cloud API Integration"
issue_description: |
  # Native Omnichannel Chat: Implement WhatsApp Cloud API Integration

  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on messaging apps to take orders and answer customer inquiries. Currently, these messages exist outside of OHC, meaning the Work Triage system cannot see them, and the Customer & Relationship Assistant cannot draft replies or update customer preferences. Managing DMs separately slows down response times, leads to missed orders, and forces the owner to manually copy information between apps. Non-technical owners need WhatsApp to function as a seamless part of their OHC assistant feed, without having to manage tokens or webhooks themselves.

  ## Research Report
  **Candidate Tool:** Meta WhatsApp Cloud API

  **Market Context:**
  WhatsApp is the primary business communication channel in Latin America, India, Europe, and increasingly North America. Competitors like WeCom and WhatsApp Business app itself lack unified multi-channel triage (e.g. combining WhatsApp with Instagram DMs and web forms). As part of OHC's mandate, we are retiring any reliance on third-party Chatwoot services and instead building a 100% native Rust omnichannel chat system within OHC's multi-tenant architecture. We have inspected Chatwoot's source code (e.g., `app/models/channel/whatsapp.rb` and `app/models/message.rb`) and will replicate and improve upon its feature set natively in Rust.

  **Evaluation:**
  - **Ease of Use (for non-technical users):** The Meta Business setup is traditionally complex, but OHC can use the Embedded Signup flow (OAuth) to let owners connect their WhatsApp number with just a few clicks. Once connected, owners interact entirely within OHC's clean Work Triage interface.
  - **Pricing:** Meta charges per conversation (User-initiated vs. Business-initiated). The first 1,000 user-initiated conversations per month are free, which perfectly covers the volume of most of our target personas (like Maya or Fatima) without adding extra costs.
  - **Technical Capabilities & Limits:**
    - The API uses Webhooks to deliver incoming messages (text, media, location).
    - It supports replying with rich media, interactive messages (buttons/lists), and automated AI drafts.
    - Cloud-hosted by Meta (no need to run a local WhatsApp client).
    - Very reliable SLA. Rate limits are tiered and scale easily beyond the needs of small businesses.
  - **SaaS Viability:** Excellent for multi-tenant cloud setup. We can register OHC as a Meta Business Solution Provider (BSP) or use standard OAuth for simple integrations.

  ## Design Doc
  - **Data Model:** Create Rust representations for WhatsApp Channels (tenant-scoped) containing phone numbers, business management tokens, template caches, and health checks, mimicking the intent of Chatwoot's `channel_whatsapp` schema but adapted for OHC's database.
  - **Webhook Ingestion Engine:** Build a highly available, idempotent webhook endpoint to receive incoming WhatsApp messages. This engine must parse Meta's JSON payload, identify the OHC tenant, map the sender to a Customer profile, and persist the message as a conversation thread inside the new native Chat engine.
  - **Work Triage Integration:** Once a message is ingested, it should trigger an event in the AI Job Queue so that the Work Triage capability can prioritize the message in the owner's feed and the Customer Assistant can draft a reply.
  - **Outbound Sending:** Implement a client in the `src/server/integrations` package for the Meta Cloud API to send text, images, and template messages. The UI will call this service when an owner approves an AI draft or types a manual response.
  - **UI/UX:** Add a simple "Connect WhatsApp" button in the Workspace settings that launches the Meta Embedded Signup flow.

  ## Implementation Prompt
  Implement a native Rust module for integrating the Meta WhatsApp Cloud API into OHC's new omnichannel chat system. The solution should allow an owner to securely link their WhatsApp Business number. Implement the webhook receiver to ingest incoming messages into OHC's Work Triage feed. Build the API client to send replies back to the customer's WhatsApp. Ensure multi-tenant isolation and adhere strictly to OHC's robust error handling and observability standards. Do not rely on any external Chatwoot services. Make sure the solution fits smoothly within the 375px mobile-first frontend.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
