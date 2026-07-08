issue_title: "Implement Central Work Triage & Owner Feed"
issue_description: |
  # Research Report: Architectural Design for Central Work Triage & Owner Feed

  ## Problem Statement
  Owners like Maya, Carlos, Priya, Leo, Fatima, Nora, and Jun are overwhelmed by fragmented work streams. Messages, bookings, orders, alerts, and agent drafts come from different channels (Instagram DMs, WhatsApp, forms, etc.). They lack a centralized "Work Command Center" that triages these inputs into a prioritized feed of what needs attention *today*, why it matters, and what the recommended next action is. Without this, the owner struggles to keep momentum and track what the agents are doing behind the scenes.

  ## Research Findings
  - **Persona Needs**: Every persona needs to immediately know what needs attention.
    - Maya needs to see new cake inquiries and which orders need deposits today.
    - Carlos needs service requests, bookings, and unrecovered leads.
    - Fatima needs new pre-orders and pickup alerts.
  - **Competitive Landscape**:
    - Traditional CRM tools (Salesforce, HubSpot) are too complex for small business operators, feeling like "admin portals".
    - Communication tools (Slack, Teams) are message-centric, not work-centric.
    - OHC needs an *assistant-first* shell where AI triage prioritizes work into actionable cards.
  - **Gap**: OHC currently lacks a unified feed architecture that aggregates and prioritizes multi-channel work items into a single, mobile-first feed for the owner.

  ## Architectural Design
  ### Overview
  The Central Work Triage system will act as the ingestion and prioritization engine.

  ```mermaid
  graph TD
      subgraph Ingestion
          DMs[Instagram/WhatsApp DMs]
          Forms[Web Forms]
          Alerts[System Alerts]
          Bookings[New Bookings/Orders]
      end

      subgraph Triage Engine
          Queue[(AI Job Queue)]
          TriageAgent[Work Triage Agent]
          DB[(PostgreSQL - Tenant DB)]
      end

      subgraph Frontend "Flutter / Next UI"
          Feed[Owner Work Feed 375px]
          ActionCards[Actionable Cards]
      end

      DMs --> Queue
      Forms --> Queue
      Alerts --> Queue
      Bookings --> Queue

      Queue --> TriageAgent
      TriageAgent --> DB : Groups, prioritizes, creates FeedItems

      DB --> Feed
      Feed --> ActionCards
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Screen**: The main tab is "Today's Focus".
  2. **Feed Items**: Displayed as cards (Translucent Glass styling). Each card shows:
     - **Context**: "Maya, you have 3 new custom cake inquiries."
     - **Agent Draft**: "Customer Assistant drafted replies."
     - **Action**: A prominent, thumb-friendly button (e.g., "Review & Send", "Approve Deposit Request").
  3. **Interaction**: Swiping or tapping a card expands it for more details (e.g., the exact drafted message).
  4. **Offline Capability**: Feed items are cached locally. Actions can be queued offline and synced later.

  ### AI Agent Integration
  - **Work Triage Agent**: The orchestrator. Monitors incoming events, groups related items (e.g., multiple DMs from the same person about the same order), assesses urgency, and generates a plain-language summary for the owner.
  - **Customer Assistant**: Drafts replies for communication-based feed items.
  - **Operations/Sales Assistants**: Prepares quotes, tasks, or booking confirmations attached to the feed items.

  ## Implementation Prompt
  **User Facing Outcome**: When the owner opens OHC, they see a clean, prioritized list of tasks, messages, and alerts (the "Owner Feed"). Each item clearly explains what happened and offers a 1-click action (e.g., "Approve Quote").

  **CUJ**:
  1. Owner logs in and sees 3 pending items in the feed.
  2. Item 1: A drafted reply to an Instagram DM. Owner clicks "Approve".
  3. Item 2: A notification that an order needs a deposit. Owner clicks "Request Payment".
  4. The feed updates seamlessly, moving completed items out of focus.

  **Acceptance Criteria**:
  - Implement a `FeedItem` data model with multi-tenant row-level security.
  - Create a background job or event listener for the Work Triage Agent to generate feed items from various sources (simulated or real).
  - Build the mobile-first (375px) UI for the Feed using Translucent Glass design tokens and 44x44px touch targets.
  - Ensure the feed updates dynamically (or optimistically) upon action.
  - Must include E2E Playwright/UI tests verifying the flow.

  ## Details
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
