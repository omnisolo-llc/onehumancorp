issue_title: "Implement Unified Agentic Work Feed & Triage System"
issue_description: |
  ## Problem Statement
  Business owners like Maya and Nora are overwhelmed by fragmented channels: Instagram DMs, emails, web inquiries, and internal task alerts. They lack a single, prioritized view of what needs attention *right now*. Traditional dashboards require them to hunt for information. They need a proactive, mobile-first feed where an AI Triage Agent has already organized, classified, and drafted responses or next actions for every incoming signal.

  ## Research Report
  - **Market Context**: Platforms like Shopify and Wix separate commerce dashboards from customer communication (Inbox apps). WeCom and Feishu integrate communication but are too complex for micro-businesses.
  - **The OHC Opportunity**: A central "Work Feed" that unifies messages, bookings, task reminders, and revenue alerts into a single timeline.
  - **Competitor Gaps**: Shopify requires third-party helpdesk apps. Independent apps like Zendesk or Intercom are too expensive and disconnected from the core commerce data (products, inventory, bookings).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Webhooks: IG, Email, Stripe] -->|Ingest| B(Message Bus / Event Queue)
      B --> C[Triage Agent / LLM Router]
      C -->|Classify Intent & Priority| D{Tenant Database}
      D --> E[Work Feed View]
      C -->|Draft Reply/Action| E
      E -->|User Approves| F[Execution Engine]
  ```

  ### Mobile UX Flow (375px)
  1. **Home Screen Feed**: The primary app screen is a chronological but priority-sorted feed of "Action Cards".
  2. **Action Card**: Each card contains the context (e.g., "New IG DM from Sarah: 'Do you have vegan cakes?'"), a tag ("High Priority - Sales"), and an AI-drafted response or action button ("Approve Reply", "Edit", "Dismiss").
  3. **Interaction**: Tapping "Approve Reply" instantly executes the action via the external API and archives the card. Tapping "Edit" opens a native mobile keyboard to tweak the draft.

  ### AI Agent Integration
  - **Work Triage Agent**: Evaluates every incoming event. Assigns priority, links to existing customer profiles, and flags urgency.
  - **Customer Assistant Agent**: Drafts the response text using the tenant's knowledge base and real-time inventory.

  ### Key Design Decisions
  - **Feed over Dashboard**: Shift the paradigm from "hunting for data" to "processing an inbox".
  - **Action-Oriented Cards**: Every item in the feed must have a clear next step or AI suggestion.

  ## Implementation Prompt
  **Feature**: Unified Agentic Work Feed
  **Target Persona**: Maya the Baker
  **Outcome**: Maya opens the app to see 3 actionable cards for overnight DMs with drafted replies. She taps "Approve" on two and edits one, clearing her inbox in seconds.

  **Acceptance Criteria**:
  1. Implement a unified `WorkFeedItem` data model supporting multiple source types (DM, system alert, booking request).
  2. Build a mobile-first Flutter UI (Action Cards) for the feed.
  3. Integrate the Triage Agent to automatically classify and draft responses for incoming simulated DMs.
  4. Ensure all database writes are tenant-isolated.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
