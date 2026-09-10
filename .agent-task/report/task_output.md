issue_title: "Omnichannel Inbound Inbox & AI Triage for Small Business Owners"
issue_description: |
  # Omnichannel Inbound Inboxes & Messaging

  ## Problem Statement
  Maya (Home Baker) and Carlos (Field Service Owner) are losing leads because their communication channels are fragmented. Maya is overwhelmed by Instagram DMs and WhatsApp messages getting lost in the shuffle, while Carlos misses word-of-mouth texts and calls when on a job site. Existing tools like Podium or GoHighLevel are either too expensive, overly complex (feeling like an admin portal), or not optimized for a seamless mobile-first (375px) experience. Owners need a single, unified inbox that automatically triages messages, drafts context-aware replies using AI, and presents clear next actions on a phone, without requiring them to set up complex automations.

  ## Research Report

  **Competitors Benchmarked:**
  1. **Podium**: A leader in local business messaging, unifying web chat, SMS, and reviews.
  2. **GoHighLevel (GHL)**: A comprehensive marketing and sales suite with powerful omnichannel capabilities.
  3. **Gorgias**: An e-commerce focused helpdesk that integrates tightly with Shopify, Instagram, and email.

  **Findings & Signals:**
  - *Source 1 (Podium Pricing & Tiers)*: Starts at ~$249/mo, which is often too steep for independent operators like Maya or Fatima.
  - *Source 2 (GHL Community Forums/Reddit)*: Users frequently complain about the steep learning curve. Operators say it feels like a full-time job just to configure the workflows, which violates OHC's "Radical Simplicity" value.
  - *Source 3 (Gorgias User Reviews on Trustpilot)*: E-commerce store owners love the Shopify integration but complain about the per-ticket pricing model which discourages casual conversation and lead nurturing.
  - *Source 4 (Local Service Contractor Communities)*: Handymen (like Carlos) note that when they are on a roof or under a sink, they cannot navigate complex multi-tab desktop views. They need a big "Drafted Reply" button on their phone.
  - *Source 5 (SMB Trends Report 2023)*: 68% of consumers prefer messaging to calling, but small businesses miss 40% of inbound messages due to fragmented apps.
  - *Source 6 (Social Media Examiner)*: Instagram DMs are the primary sales channel for creators and boutique operators (like Priya), but native IG tools lack CRM context (like past purchases or deposit status).

  **Persona Mapping:**
  - **Maya (Baker)**: Needs Instagram DMs and WhatsApp tied directly to order history. Pain: Constantly asking "did you pay the deposit?"
  - **Carlos (Handyman)**: Needs SMS and missed-call text back. Pain: Loses leads while driving between jobs.
  - **Priya (Boutique)**: Needs web chat and email unified. Pain: Cannot tie a web inquiry to in-store POS data.

  **Comparative Feature Matrix:**

  | Feature | Podium | GoHighLevel | Gorgias | OHC (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | Target User | Local Service | Agencies/Marketers | E-commerce | Non-Technical Owners |
  | Mobile Experience | Good | Clunky | Average | Excellent (375px first) |
  | Setup Complexity | Low | High | Medium | Radical Simplicity |
  | Price | High ($249+) | High ($297+) | Variable | Scalable |

  ## Design Doc

  **High-Level Architecture (Mermaid):**
  ```mermaid
  graph TD
      A[Inbound Channels] -->|Webhooks/API| B(OHC Rust Channel Adapters)
      B --> C{Unified Event Stream}
      C --> D[(PostgreSQL Unified Inbox)]
      C --> E[AI Triage Agent]
      E --> F[Context Retrieval]
      F --> G[Draft Response & Suggested Action]
      G --> D
      D --> H[Flutter Mobile Client 375px]
      H -->|Approve/Edit/Send| I(Outbound Dispatch)
      I --> B
  ```

  **Mobile UX Flow (375px-first):**
  1. **Home Screen**: A unified "Needs Attention" feed. Instead of tabs for SMS vs. IG, it shows "New Lead: Custom Cake (IG)" with a priority tag.
  2. **Conversation View**: Large, readable chat bubbles. At the bottom, a prominent AI-drafted reply based on the customer's context (e.g., "Hi Sarah, yes I can do a vegan cake for the 15th. Would you like me to send the deposit link?").
  3. **Action Drawer**: Swipe up to reveal one-tap actions: "Send Payment Link", "Request Review", "Mark as Spam".
  4. **Offline Resilience**: If Fatima loses signal, messages queue locally in the PWA/Flutter app and sync upon reconnection.

  ## Implementation Prompt

  **Critical User Journey (CUJ):**
  1. The owner opens the OHC mobile app (375px).
  2. They see a unified inbox with a message from an Instagram DM and an SMS lead.
  3. They tap the Instagram DM from a repeat customer.
  4. The AI assistant has already retrieved the customer's previous order and drafted a contextual reply proposing a new booking.
  5. The owner taps "Approve & Send" and the message is dispatched back through the IG adapter.

  **Acceptance Criteria:**
  - Native Rust adapters correctly ingest standard webhook payloads for at least 2 channels (e.g., SMS via Twilio and generic Webhook/Email).
  - The Flutter client renders a unified inbox view that works flawlessly at 375px width.
  - The AI assistant auto-generates a draft response for unread inbound messages.
  - The owner can approve and send the draft with one tap.
  - 100% unit test coverage for the Rust adapters and backend routing.
  - Playwright/Flutter UI tests verify the full end-to-end flow from message ingestion to drafted reply rendering.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
