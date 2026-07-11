issue_title: "Integrate Meta WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  ### Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on messaging apps to take orders and answer customer inquiries. Currently, these messages exist outside of OHC, meaning the Work Triage system cannot see them, and the Customer & Relationship Assistant cannot draft replies or update customer preferences. Managing DMs separately slows down response times, leads to missed orders, and forces the owner to manually copy information between apps. Non-technical owners need WhatsApp to function as a seamless part of their OHC assistant feed, without having to manage tokens or webhooks themselves.

  ### Research Report
  **Candidate Tool:** Meta WhatsApp Cloud API

  **Market Context:**
  WhatsApp is the primary business communication channel in Latin America, India, Europe, and increasingly North America. Competitors like WeCom and WhatsApp Business app itself lack unified multi-channel triage (e.g. combining WhatsApp with Instagram DMs and web forms).

  **Evaluation:**
  - **Ease of Use (for non-technical users):** The Meta Business setup is traditionally complex, but OHC can use the Embedded Signup flow (OAuth) to let owners connect their WhatsApp number with just a few clicks. Once connected, owners interact entirely within OHC's clean Work Triage interface.
  - **Pricing:** Meta charges per conversation (User-initiated vs. Business-initiated). The first 1,000 user-initiated conversations per month are free, which perfectly covers the volume of most of our target personas (like Maya or Fatima) without adding extra costs.
  - **Technical Capabilities & Limits:**
    - The API uses Webhooks to deliver incoming messages (text, media, location).
    - It supports replying with rich media, interactive messages (buttons/lists), and automated AI drafts.
    - Cloud-hosted by Meta (no need to run a local WhatsApp client).
    - Very reliable SLA. Rate limits are tiered and scale easily beyond the needs of small businesses.
  - **SaaS Viability:** Excellent for multi-tenant cloud setup. We can register OHC as a Meta Business Solution Provider (BSP) or use standard OAuth for simple integrations.

  ### Design Doc
  **Integration Flow:**
  - **Trigger/Connection:** Owner goes to "Channels" in OHC settings, clicks "Connect WhatsApp", and completes the Meta OAuth popup. OHC stores the access token securely.
  - **Inbound Action (Webhooks):** When a customer messages the connected number, Meta sends a webhook to OHC. The OHC API parses this, associates it with a Customer record (or creates a new lead), and pushes it to the AI Job Queue.
  - **Assistant Triage:** The Customer & Relationship Assistant reads the message, drafts a context-aware reply (e.g., pulling up Maya's delivery calendar), and places it in the owner's Work Triage feed.
  - **Owner Action:** The owner sees the drafted reply in OHC, approves or edits it, and OHC sends it back via the WhatsApp Cloud API POST endpoint.

  ### Implementation Prompt
  Implement the Meta WhatsApp Cloud API integration to allow owners to receive and reply to WhatsApp messages directly within their OHC Work Triage feed.

  **Acceptance Criteria:**
  - Create a setup flow for the owner to connect their WhatsApp Business account via Meta OAuth.
  - Implement a secure webhook receiver to ingest incoming WhatsApp messages and map them to the correct OHC tenant.
  - Route incoming messages to the AI Triage system so the assistant can draft context-aware replies.
  - Provide a UI in the Work Triage feed for the owner to view the conversation, edit the AI's draft, and send the reply back to WhatsApp.
  - Support basic text and image message types.
  - Ensure the solution is mobile-responsive and functions perfectly on a 375px screen.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
