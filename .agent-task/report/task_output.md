issue_title: "Implement AI Unified Work Triage Architecture"
issue_description: |
  # Research Report: AI Unified Inbox & Work Triage Architecture

  ## Title
  Implement AI Unified Inbox & Work Triage Architecture

  ## Problem Statement
  Small business owners like Maya (Baker) and Carlos (Field Service) are overwhelmed by context switching. They receive demand from Instagram DMs, SMS, WhatsApp, Web Forms, and direct emails. Managing these channels requires bouncing between multiple apps, which leads to missed leads, forgotten follow-ups, and fragmented customer histories. Current platforms (Shopify, Wix) treat messaging as a separate "Inbox" silo rather than the central driver of business operations. OHC needs an AI Unified Work Triage system that turns scattered conversations across platforms into actionable work items (bookings, quotes, orders) in a single feed.

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify Inbox:** A basic consolidated chat app, but it is siloed. It requires manual action to turn a chat into an order and struggles with external channels like WhatsApp or Instagram.
    - **Zendesk/Intercom:** Too enterprise-focused and expensive for micro-SMEs; overly complex setup for a simple baker or handyman.
    - **Tencent Workbuddy/WeCom:** Highly successful at integrating chat with CRM and task management, proving the model works.
  - **OHC Opportunity:** Instead of just consolidating messages, the OHC "Work Triage" should act as an active assistant. When a message comes in, the AI agents (`Customer Assistant` and `Operations Assistant`) should analyze it, extract intent (e.g., "quote request for Saturday"), link it to the customer profile, and proactively draft a reply or propose an action (e.g., generate a quote).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram / SMS / Web Form / Email] --> B[Ingestion Webhooks / APIs]
      B --> C[Event Stream / Queue]
      C --> D[AI Work Triage Service]
      D --> E{Agent Evaluation}
      E -->|Sales Intent| F[Sales Agent: Draft Quote]
      E -->|Support Intent| G[Customer Agent: Draft Reply]
      E -->|Booking Intent| H[Operations Agent: Check Calendar]
      F --> I[Unified Owner Feed]
      G --> I
      H --> I
      I --> J[Mobile App / Web UI]
  ```

  ### Data Model
  - `tenant_id` isolated tables for `communications`, `messages`, `threads`, and `triage_actions`.
  - Unified `Thread` entity linked to a `Customer` entity, allowing cross-channel continuity.
  - `TriageAction` table to track AI-proposed next steps (e.g., `DRAFT_REPLY`, `GENERATE_QUOTE`, `SCHEDULE_MEETING`) mapped to the `Thread`.

  ### Mobile UX Flow (375px First)
  1. **Home Screen Feed:** The primary view is NOT a dashboard of charts; it is a prioritized feed of action items.
  2. **Inbox Card:** A card shows a new message: "Maya, 3 new cake requests from Instagram. 2 are asking for this Saturday."
  3. **Thread View:** Tapping the card opens the unified thread. The AI has already drafted a reply acknowledging the request and includes an actionable button: "Send Draft & Create Quote".
  4. **One-Tap Action:** The owner reviews the draft and taps the button. The system sends the message and transitions to the Quote generation screen with pre-filled details.

  ### AI Agent Integration
  - **Work Triage Coordinator:** Parses incoming messages, identifies the customer, updates their CRM profile with new context, and determines intent.
  - **Department Handoff:** Routes the intent to the appropriate specialist agent (Sales, Operations, Customer Success) to prepare the next best action.
  - **Memory:** Utilizes Redis for fast context retrieval and PostgreSQL for long-term customer history.

  ## Implementation Prompt
  Implement the core backend components for the AI Unified Inbox. Build the data models (Thread, Message, TriageAction) with strict multi-tenant isolation. Create the ingestion API endpoint that simulates receiving a webhook from a messaging channel. Implement the Work Triage Coordinator logic that utilizes the LLM to analyze the message, link it to a Customer, and generate a TriageAction (e.g., drafting a reply). Build the primary endpoint to fetch the unified feed for the mobile client. Do not prescribe specific SQL; ensure proper Redis locking during thread updates to prevent race conditions.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []