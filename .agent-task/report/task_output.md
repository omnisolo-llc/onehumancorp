issue_title: "Architecture Design: Agentic Work Triage & Unified Owner Feed"
issue_description: |
  ## Mission Queue Protocol: Agentic Work Triage & Unified Owner Feed

  ### Problem Statement
  The core promise of OneHumanCorp (OHC) is "Open OHC and immediately know what needs attention today." Currently, our backend has domain-specific systems (billing, supply chain, delivery, etc.), but lacks a centralized "Work Triage" engine. Without this, OHC behaves like a traditional software suite (Shopify/Wix) where owners like Carlos (Handyman) or Maya (Baker) must navigate through menus to discover what needs doing. We need a unified AI-driven feed that synthesizes cross-departmental events into actionable, human-readable "Action Cards."

  ### Research Report & Gap Analysis
  - **Competitor Landscape**: Shopify Sidekick and Wix AI act as responsive chatbots, answering questions or doing tasks *when asked*. Traditional dashboards use passive notification badges.
  - **The OHC Differentiator**: Proactive, assistant-first orchestration. The system must act as a Chief of Staff, identifying tasks (e.g., "Invoice overdue," "Vegan cake inquiry," "Low flour inventory") and presenting them as ready-to-execute decisions.
  - **Identified Gap**: Missing `Triage` and `ActionFeed` sub-systems. We lack a real-time event aggregation pipeline that feeds raw signals to an AI intent-classifier, which then produces a durable `ActionCard` for the owner's mobile feed.

  ### Design Doc
  #### High-Level Architecture
  ```mermaid
  graph TD
      E_Stripe[Stripe Webhooks] -->|Event| IB(Event Bus / Redis PubSub)
      E_IG[Instagram Graph API] -->|Event| IB
      E_DB[Internal Service Changes] -->|Event| IB

      IB -->|Dequeue| T_Agent[Chief of Staff Agent / Triage Engine]
      T_Agent -->|Query Context| Mem[Tenant Memory & DB]
      T_Agent -->|Classify & Synthesize| Draft[Generate Action Card]

      Draft --> DB_Feed[(Action Feed DB - PostgreSQL)]
      DB_Feed --> Cache[Redis Feed Cache]

      Cache -->|Real-time sync| Mobile[Flutter App 375px]
      Mobile -->|Approve/Reject| Exec[Action Executor]
      Exec -->|Dispatch| Services[Domain Services]
  ```

  #### Mobile UX Flow (375px First)
  1. **The Morning View**: Owner opens the app and lands immediately on the `Feed` tab. No dashboards or charts clutter the view.
  2. **Translucent Glass Cards**: Each prioritized item is a card. E.g., "Sarah asked about custom cupcakes. The Ambassador drafted a reply. [Review & Send]".
  3. **One-Tap Actions**: The card contains primary buttons (e.g., "Send", "Approve Quote", "Ignore"). Tap targets are 44x44px minimum.
  4. **Swipe Gestures**: Swipe right on a card to quickly approve the AI's suggested action. Swipe left to dismiss/archive.
  5. **Zero-Latency Feel**: Optimistic UI updates. Approving an action hides the card instantly while the backend processes the task.

  #### AI Agent Integration Points
  - **Chief of Staff Agent**: Subscribes to the global event mesh. Determines priority, groups related events (e.g., 3 inquiries about the same product), and creates the `ActionCard`.
  - **Domain Agents (The Ambassador, The Accountant)**: Called by the Chief of Staff to populate the payload of the `ActionCard` (e.g., The Accountant drafts the overdue invoice reminder).

  #### Key Design Decisions
  - **Eventual Consistency for Feed Processing**: Raw events are ingested instantly, but action cards may take ~2-5 seconds to appear as AI synthesizes context. We rely on PostgreSQL `SKIP LOCKED` for reliable queue processing.
  - **Tenant Isolation**: Strict row-level security on the `ActionFeed` table. Caches in Redis use `ohc:tenant:{tenant_id}:feed`.
  - **Idempotent Executions**: Every action card has a unique token. Executing it (e.g., charging a card) uses this token to prevent double-execution over flaky mobile networks.

  ### Implementation Prompt
  **User-Facing Outcome**: Carlos opens his OHC app and sees his 3 priorities for the day: a quote to approve, a missed call to follow up on, and a reminder to buy supplies. He approves the quote with one tap, and the AI emails it to the customer.
  **CUJ & Acceptance Criteria**:
  1. Define the `triage.proto` defining `ActionCard`, `ActionIntent`, and `FeedQuery`.
  2. Implement a PostgreSQL-backed queue using `SKIP LOCKED` to ingest simulated events (e.g., a new message, a failed payment).
  3. Create the `Chief of Staff` worker that consumes these events, calls a mock LLM provider to synthesize a title/description, and persists an `ActionCard` to the database.
  4. Build the Flutter UI component for the "Feed Screen" at 375px width, rendering a list of action cards.
  5. Add Playwright E2E tests: A test user logs in, the test injects 2 events via an admin API, the UI updates to show 2 cards, the user clicks "Approve" on one card, and the card is removed from the feed.

  ### Scope & Priority
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
