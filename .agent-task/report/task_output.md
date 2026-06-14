issue_title: "OHC Multi-Channel Messaging Hub (Work Triage Agent)"
issue_description: |
  # Mission Queue Protocol: OHC Multi-Channel Messaging Hub

  ## Problem Statement
  Business owners like Maya (the baker) and Carlos (the field service operator) juggle customer inquiries across multiple platforms (Instagram DMs, WhatsApp, email, SMS, and their website). They miss leads because inquiries are scattered, and it's hard to distinguish urgent custom order requests from casual questions. They need a unified, intelligent "Work Triage" feed that not only consolidates messages but drafts contextual replies, handles routine queries automatically, and surfaces the high-value actions to the owner on their mobile device.

  ## Research Report
  - **Competitor Insights**: HubSpot's Breeze and Intercom Fin focus heavily on enterprise/B2B context with complex setups. Tools like 11x.ai handle outbound or phone, but there is a gap for a dead-simple, unified inbox for micro-SMEs that feels like an Apple/iMessage experience.
  - **Pain Points**: SMBs spend hours copying context between systems. They forget to follow up. They lose deposits because they cannot quickly convert a DM into a quote or payment link.
  - **Opportunity**: OHC can differentiate by deeply integrating the unified inbox with the "Sales & Revenue Assistant" and "Customer & Relationship Assistant," meaning an Instagram DM can instantly generate a drafted reply *and* a Shopify-style checkout link within the same view.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - Ingestion Layer: Webhooks from WhatsApp Business API, Instagram Graph API, Email (SendGrid/Mailgun inbound), and Web Chat.
    - Routing Layer: A PostgreSQL `SKIP LOCKED` job queue where the Work Triage Agent categorizes and prioritizes the message.
    - AI Agent Layer: The Customer Assistant retrieves tenant memory, drafts a response, and attaches necessary actions (like creating a draft quote).
    - Presentation Layer: A unified feed in the Flutter app where the owner reviews the draft and hits "Approve & Send."
  - **Mobile UX Flow**:
    - **Screen 1**: "Today's Attention" Feed. Shows unified messages sorted by urgency/value (e.g., "Urgent: Maya asked about her cake delivery tomorrow" > "Lead: New inquiry from Web Form").
    - **Screen 2**: Thread View. Looks like iMessage. The AI's suggested reply sits in a frosted glass text input area above the keyboard.
    - **Screen 3**: Action Sheet. Owner can tap "Edit," "Send," or "Attach Payment Link."
  - **AI Agent Integration**:
    - `Customer_Assistant_Prompt`: Ingests the new message and past customer context. Returns a structured JSON containing a confidence score, a drafted reply, and suggested next actions (e.g., `CREATE_QUOTE`).
  - **Key Design Decisions**:
    - The AI *never* sends a message to a new contact without explicit owner approval (unless explicitly configured in advanced settings).
    - All messages flow through a single `messages` table with `tenant_id` RLS and a `channel` enum.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend ingestion, agent routing, and frontend UI for the OHC Multi-Channel Messaging Hub (Work Triage).
  - **CUJ**: As Maya, I receive an Instagram DM asking "Do you make vegan cakes for this Saturday?". I open the OHC app, see the message at the top of my feed, see an AI-drafted reply ("Yes, we do! I have a slot open. A 6-inch vegan cake starts at $50. Would you like to book?"), and tap "Send."
  - **Acceptance Criteria**:
    - Create the necessary database schema for unified messages with tenant isolation.
    - Implement a mock ingestion endpoint for testing the flow.
    - Implement the AI agent call to draft the reply using the existing LLM provider setup.
    - Build the Flutter UI (mobile-first, 375px) to display the feed and the thread view with the drafted reply.
    - Ensure all interactions are verified via Playwright/browser tests (no mock data in the final UI).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
