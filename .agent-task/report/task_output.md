issue_title: "Implement Intelligent Work Intake & Triage System"
issue_description: |
  ## Issue: Intelligent Work Intake & Triage System

  ### Problem Statement
  Currently, small business owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by work arriving from multiple channels—DMs, emails, web forms, and calls. These requests exist in silos, requiring manual consolidation into tasks, bookings, and quotes. They need a centralized, AI-driven intake system that automatically aggregates, triages, categorizes, and proposes the next best action for every incoming request, saving time and preventing lost revenue.

  ### Research Report
  - **Context**: The OHC product vision demands an "Owner Clarity" approach where work intake is unified and actionable.
  - **Competitive Analysis**: Platforms like HubSpot and Shopify Inbox consolidate messages but often require manual triage. AI-first platforms (like Tencent Workbuddy) automate the categorization and next-step proposal.
  - **Observation from Dogfooding**: Simulating the life of an owner, I found that incoming data lacks a unified entry point that immediately suggests an action (e.g., converting a DM into a quote). The absence of this feature directly violates the "Open OHC and immediately know what needs attention today" promise.
  - **Proposed Solution**: A scalable Intake Event Bus that ingests from various channels, processes via an AI Triage Agent, and surfaces standardized `WorkItem` entities in a prioritized feed.

  ### Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      Channels[Email/DM/Form] --> WebhookTunnel[Hybrid MCP Webhook Tunnel];
      WebhookTunnel --> EventBus[Intake Event Bus];
      EventBus --> TriageAgent[AI Triage Agent];
      TriageAgent --> ContextEnrichment[Memory & Context Routing];
      ContextEnrichment --> WorkItemDB[(PostgreSQL WorkItem Table)];
      WorkItemDB --> TriageFeed[Unified Triage Feed UI];
    ```
  - **Mobile UX Flow (375px first)**:
    - **Home Screen**: A clean, unified inbox. New items appear as distinct cards with an AI-generated summary and a primary action button (e.g., "Draft Quote", "Decline").
    - **Detail View**: Tapping a card shows the original message, the AI's reasoning for the triage category, and a unified action bar.
    - **Actions**: Using native mobile keyboards and bottom sheets for quick actions (e.g., approving a draft reply).
  - **AI Agent Integration**:
    - The Triage Agent listens to the Intake Event Bus.
    - It uses the configured LLM (Gemini/MiniMax) to classify the intent (Inquiry, Support, Booking, Spam).
    - It maps the intent to a suggested workflow state.
  - **Key Decisions**:
    - Rely on the existing `webhook-tunnel` for external ingestion.
    - Introduce a generic `WorkItem` model that supports polymorphic payloads depending on the channel.
    - Ensure row-level multi-tenant isolation via `tenant_id` on the `WorkItem` table.

  ### Implementation Prompt
  Implement the Intelligent Work Intake & Triage System. The primary CUJ starts with a simulated incoming DM (via API or test fixture). The system must capture the event, invoke the Triage Agent to categorize it, and display it in the new "Triage Feed" on the dashboard. Acceptance criteria:
  1. A new backend module for `WorkIntake` is created with a `WorkItem` data model (ensure `tenant_id` isolation).
  2. The Triage Agent successfully processes incoming events and assigns a category and suggested action.
  3. The UI (starting with mobile 375px view) displays a prioritized feed of these items.
  4. The owner can click an action (e.g., "Acknowledge") which updates the item's state.
  5. Include full E2E Playwright tests covering this flow from fake-webhook to UI state update.
  6. Unit test coverage must be 100%.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
