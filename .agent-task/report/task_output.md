issue_title: "Implement the Mobile-First Unified Agent Feed (Zero-Click Operations)"
issue_description: |
  **Title**: Implement the Mobile-First Unified Agent Feed (Zero-Click Operations)

  **Problem Statement**:
  Business owners (e.g., Maya the Home Baker, Carlos the Handyman) currently face an administrative dashboard that forces them to hunt for tasks, approvals, and insights across disparate menus. The traditional SaaS "dashboard" paradigm does not fit the reality of small-business operators who work 100% on their mobile devices (375px screens) and have no time to navigate nested settings. They need an AI work assistant that acts as a true operational partner—presenting critical, actionable items directly to them without complex navigation.

  **Research Report**:
  *Findings & Competitive Analysis*:
  - **Legacy Systems** (Shopify, Wix, Squarespace) treat mobile apps as companion viewers or simple POS endpoints. They lack robust configuration and operational execution on mobile, often redirecting users to a desktop browser for complex tasks.
  - **AI Builders** (Durable, 10Web) excel at zero-to-one setup (generating a site in 30 seconds) but fail to offer continuous, proactive operational agency (e.g., handling customer inquiries, auto-restocking, dynamic scheduling).
  - **Market Gap**: A mobile-first feed that aggregates multi-department AI Agent proposals into a single "Unified Inbox" stream (e.g., Marketing agent proposing an Instagram post; Operations agent prompting for a deposit on a booking).
  - **Conclusion**: We must implement an "Approval UI" paradigm where complex backend actions are synthesized into single feed cards with large, clear action buttons ("Approve", "Edit", "Discard"), strictly constrained to a 375px mobile viewport.

  **Design Doc**:
  *Architecture Design*:
  - **Event Ingestion Pipeline**: A PostgreSQL-backed table for `AgentFeedItems`. Diverse agents (Sales, Ops, Marketing) in the KAIROS orchestration mesh publish normalized `FeedCard` events (via gRPC to the HubService).
  - **Context Resolution & Drafting**: The AI agent queries the tenant's specific business data (inventory, CRM, policies) to build context, drafts a proposed action or response, and creates a pending feed item.
  - **Mobile UX Flow (375px Baseline)**:
    1. The owner opens the OHC mobile app. The traditional metrics dashboard is replaced by a vertical `Agent Feed`.
    2. The feed displays actionable "Agent Proposal" cards. Each card strictly adheres to 375px constraints.
    3. Each card features: Context Summary (e.g., "3 new cake inquiries"), Agent Persona (e.g., Customer Service), the drafted response, and large touch targets (>44x44px) for primary actions like "Approve & Send".
    4. Upon pressing "Approve", the frontend makes an async call to the backend API, executing the state change invisibly, and gracefully animating the card's completion state (macOS Translucent Glass styling, `--transition-fluid`).
  - *Mermaid Architecture Diagram*:
    ```mermaid
    sequenceDiagram
      participant App as Mobile App (375px)
      participant API as API Gateway (gRPC/REST)
      participant FeedDB as Feed Database
      participant Agent as KAIROS Agent (Ops/Sales)

      Agent->>FeedDB: Publish Proposed Action (Drafted Reply)
      App->>API: GET /api/v1/feed
      API->>FeedDB: Fetch Active Feed Cards
      FeedDB-->>API: List<FeedCard>
      API-->>App: Render Action Cards (Glassmorphism UI)
      App->>API: POST /api/v1/feed/{id}/approve
      API->>Agent: Execute Approved Action
      Agent-->>API: Success Response
      API-->>App: UI Updates (Card Completed)
    ```

  **Implementation Prompt**:
  As an Implementer agent, your mission is to build the "Unified Agent Feed (Mobile MVP)". You will design and implement the backend schema for storing agent-generated Action Cards, extend the API layer to expose this feed, and build the frontend UI to display it as the primary home screen. The UI must strictly follow mobile-first constraints (375px width, minimum 44x44px touch targets). Use the OHC Premium Token design system (translucent glass styling, `bg-slate-950`).

  The Critical User Journey (CUJ) starts with a non-technical owner (e.g., Maya) logging in and seeing a feed of AI-proposed actions. The user clicks "Approve" on a card, triggering a backend state change simulating the execution of the task. Do not strictly adhere to any specific schema or route in this prompt; design them yourself based on the existing `Agent Manager` and `HubService` architecture. Verify the UI is fully functional, properly styled, and reactive using Playwright E2E tests simulating a 375px mobile viewport.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
