issue_title: "Unified Agent Feed: Mobile-First Proactive Dashboard"
issue_description: |
  ## Title
  Unified Agent Feed: Mobile-First Proactive Dashboard

  ## Problem Statement
  Legacy platforms (Shopify, Wix) treat mobile apps as supplementary dashboards for viewing stats, requiring a desktop for actual store building and complex management. Business owners like Maya (baker, Instagram DM sales) or Fatima (food cart, mobile-only) need to run 100% of their operations from a 375px mobile screen. Complex forms with dozens of toggles (e.g., setting up a discount code) do not work on mobile. They need an assistant-first shell where they review and approve agent-proposed actions instead of navigating complex admin menus.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **The Legacy Paradigm (Shopify, Wix):** Inherently designed for desktop. The mobile apps are "companion apps" for checking revenue and fulfilling orders. Changing designs or configuring apps requires returning to a desktop browser.
  - **Mobile-First Creators (Linktree, Stan Store):** Succeeded because the modern solopreneur operates entirely from their phone. They rely on absolute simplicity and big touch targets, but lack robust business platform features.
  - **OHC Opportunity:** The solution to complex mobile UI is not better responsive design; it is "Chat & Approval UI" powered by Agents. Instead of a complex form, the AI drafts the logic (e.g., "Run a 20% off sale"), schedules it, and presents a single "Card" on the mobile dashboard detailing the proposed actions with an "Approve" button.
  - **The Agent Feed Deep Dive:** The feed is the central nervous system. It ingests events (webhooks, state changes), classifies intent, uses RAG for context, and generates draft actions/responses pushed directly to the user's mobile device for review.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Sources: DMs, Orders, Inventory] -->|Ingest| B(Event Bus/Queue)
      B --> C[Intent Classification LLM]
      C --> D{Context RAG & Identity Engine}
      D -->|Fetch Context| E[Tenant DB / Customer Graph]
      D --> F[Specific Department Agent: Sales, Ops, CS]
      F -->|Draft Proposal| G[Agent Feed Service]
      G --> H[Action Card Database]
      H --> I[Flutter Mobile App 375px]
      I -->|User Approves Card| J[Execution Engine]
      J --> K[External Services & DB Updates]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen (The Feed):** The first screen is a vertical feed of "Agent Proposals" and "Urgent Items", replacing traditional dashboards.
  - **Action Cards:** Each card (e.g., "Draft Reply for Maya", "Inventory Alert for Priya") uses clean Apple/Ubiquiti-style hierarchy, translucent materials, and strong spacing. Touch targets are at least 44x44px.
  - **Interaction:** Cards have a brief summary, a detailed view on tap, and clear primary actions like "Approve", "Edit", or "Discard".
  - **Visual Design:** OHC Premium Token library, restrained translucent materials, readable typography.

  ### AI Agent Integration Points
  - **Event Ingestion:** Triggers the pipeline.
  - **LLM Intent Classification & Draft Generation:** Gemini Pro primary (fallback to GPT-4o). Queries tenant-scoped memory and drafts the proposal.
  - **Agent Feed Service:** Manages the lifecycle of action cards in the feed, ensuring real-time or near-real-time delivery to the mobile app.

  ### Key Design Decisions
  - **Proactive Push:** Move from user-seeking-info to system-pushing-actionable-proposals.
  - **Mobile-First Strictness:** No horizontal scroll on 375px. Every workflow must be completable via this feed mechanism.
  - **Approval UI:** Simplify complex operations into structured agent proposals needing only a single tap to execute.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, when I open the OHC app, I don't see a static dashboard. Instead, I see a prioritized feed of action items drafted by my AI assistants (e.g., a drafted reply to a customer, a proposed order to restock supplies). I can approve these items with a single tap.
  **CUJ:**
  1. Open the Flutter app on a 375px simulated mobile device.
  2. Authenticate and land on the main "Agent Feed" screen.
  3. See a list of at least 3 distinct Action Cards (e.g., from CS, Ops, and Sales agents).
  4. Tap "Approve" on one card.
  5. Verify the card moves to a "Completed/Executed" state and the underlying action is triggered (mocked or verified via UI state change).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []