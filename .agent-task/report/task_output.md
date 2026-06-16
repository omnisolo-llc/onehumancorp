issue_title: "Implement Zero-Configuration Agentic Unified Omnichannel Inbox"
issue_description: |
  # Research Report: Zero-Configuration Agentic Unified Omnichannel Inbox

  ## 1. Problem Statement
  Small business owners and operators (e.g., Maya the Home Baker, Carlos the Handyman) receive customer inquiries across a fragmented landscape of channels: Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Monitoring these channels individually causes context switching, missed leads, and delayed responses. Existing unified inbox solutions (like HubSpot or basic Zendesk) require complex technical setup, rigid routing rules, and do not proactively draft context-aware responses or integrate directly with the business's operational data (inventory, bookings, quotes).

  ## 2. Research Report
  - **Market Context**: Traditional CRMs offer unified inboxes but present them as complex administrative portals. AI-native tools (like Lindy.ai or specific ManyChat flows) often require "building" the logic or lack deep integration with the core commerce/booking engine.
  - **The OHC Opportunity**: OHC can provide a zero-configuration unified inbox that acts as the "Work Triage" center. The core differentiator is the *Agentic layer*: the inbox isn't just a unified view; it actively triages messages, identifies intent (e.g., "quote request", "availability check"), drafts responses using business context (RAG over inventory/calendar), and presents actionable cards to the owner.
  - **Competitor Gaps**:
    - *Shopify Inbox*: Good for web chat and basic IG, but limited AI proactivity and operational depth.
    - *Zendesk/HubSpot*: Overwhelming UI for a mobile-first solo operator; setup requires a dedicated administrator.
    - *ManyChat*: Powerful but requires building complex visual logic trees, violating the "no technical manuals" promise.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Conversation`: Represents a unified thread, linked to a `Customer` and a specific channel provider (e.g., Meta, Twilio).
  - `Message`: Individual messages within a conversation, tracking sender (customer, agent, owner), content, and intent metadata.
  - `ActionCard`: An AI-generated proposed action linked to a conversation (e.g., "Send Deposit Link", "Draft Quote").

  ### AI Integration (Work Triage & Customer Assistant)
  - **Triage Agent**: Monitors incoming messages via webhooks. Uses LLMs to classify intent (inquiry, complaint, booking request).
  - **Drafting Agent**: Uses RAG over the tenant's policies, inventory, and calendar to draft a contextual reply.
  - **Handoff Protocol**: If the agent's confidence in the drafted reply or action is high but requires authorization, it presents an `ActionCard` in the owner's feed for a 1-tap "Approve & Send".

  ### Mobile UX Flow (375px)
  1. **The Triage Feed**: The owner opens the app and sees a prioritized feed of action cards, not just a chronological list of messages. High-value leads or urgent issues are bubbled to the top.
  2. **Action Cards**: A card shows a snippet of the customer message (e.g., "Do you have the red dress in Medium?") and the Agent's drafted reply ("Yes, we have 2 left. Would you like me to hold one?").
  3. **1-Tap Resolution**: The owner taps "Approve" (sends the drafted message) or "Edit" (opens the native keyboard to tweak the response before sending).

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Unified Omnichannel Inbox
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya receives an Instagram DM asking about a custom cake. She doesn't need to check Instagram. OHC's Triage feed shows a card with the customer's message and a drafted, context-aware reply. She taps "Approve" while baking, and the response is sent back to the customer's Instagram DM seamlessly.

  **Next Actions**:
  1. Implement the core Data Models (`Conversation`, `Message`, `ActionCard`) with strict multi-tenant isolation.
  2. Develop the integration layer for a primary channel (e.g., Twilio SMS or Meta Graph API for Instagram) to ingest messages and send replies.
  3. Create the Triage Agent capability to classify incoming messages and the Drafting Agent to generate context-aware replies using RAG.
  4. Develop the Mobile-First (375px) Triage Feed UI, focusing on the Action Card design and 1-tap "Approve" interaction.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
