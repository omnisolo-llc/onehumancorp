issue_title: "Implement AI-Native Unified Agent Action Feed & Inbox (Mobile-First)"
issue_description: |
  # OHC Mobile-First Operations: AI-Native Unified Agent Action Feed

  ## Problem Statement
  Business owners like Maya (Home Baker) and Carlos (Handyman) miss critical sales because they are overwhelmed by scattered communications and disjointed operations. Existing platforms (Shopify, Wix) treat mobile apps as supplementary "dashboards" for viewing stats and require desktop computers for real operational management (e.g., editing store design, complex inventory, running campaigns). OHC must enable 100% of business operations—from initial setup to daily execution—on a 375px mobile screen. The solution to complex mobile UI is not better responsive design; it is a Chat & Approval UI powered by Agents. Users need a centralized, proactive "Agent Feed" that pushes critical updates, drafted communications, and suggested operational actions directly to their mobile device for simple review and approval.

  ## Research Report
  Based on competitive audits (Shopify, Wix, Linktree, Stan Store) and market research (`docs/business/market_research/ohc_smb_mobile_first_design_research.md`, `docs/business/market_research/agent_feed_deep_dive.md`):
  - **Legacy Platforms**: Inherent desktop bias. High friction for on-the-go modifications. The "companion app" model fails for complex tasks.
  - **Mobile-First Creators (Link-in-bio)**: Succeed due to absolute simplicity and big touch-friendly UI. But they lack robust operational backing (inventory, complex routing).
  - **The OHC Gap**: OHC needs a "Zero-Click Generation" flow and an "Approval" Interface paradigm. Instead of a complex form with 20 toggles to set up a discount code, the Marketing Agent proposes it, and the user taps one massive "Approve" button.
  - **Agent Feed Deep Dive**: The Agent Feed acts as the central nervous system. Event ingestion (Stripe, IG Graph API) -> LLM Intent/Context Resolution -> Draft Generation -> Mobile Notification & Approval UX.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Event Ingestion
          IG[Instagram Graph API / DMs] -->|Webhook| MBus(Event Message Bus / Redis PubSub)
          STR[Stripe Events] -->|Webhook| MBus
          INT[Internal OHC Events] --> MBus
      end

      subgraph AI Processing Layer
          MBus --> Worker[Async Job Worker]
          Worker --> LLM[Gemini Pro Intent Classification]
          LLM --> RAG[RAG: Tenant Policies & Inventory]
          RAG --> Draft[LLM Action/Response Drafter]
      end

      subgraph Mobile UX
          Draft --> Feed[Tenant Action Feed DB]
          Feed --> App[OHC Flutter App - 375px]
          App -->|User taps 'Approve'| Exec[Action Executor]
          Exec --> IG_Send[Send Reply via IG API]
          Exec --> State[Update System State]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **The Core Screen**: The user opens the app and sees a vertical feed of "Agent Proposals" and "Urgent Items", utilizing OHC Premium Tokens (translucent glass, Apple/Ubiquiti-style hierarchy).
  2. **Card Structure**: Each item is a high-contrast card.
     - *Card 1 (Customer Service)*: "Maya, 3 new Instagram inquiries about vegan cakes. [Review Drafts]"
     - *Card 2 (Operations)*: "Inventory low for Vanilla Extract. Reorder? [Approve]"
  3. **Interaction**: User taps "Review Drafts". The card expands to show the generated context-aware reply. The user has clear >44x44px touch targets: "Approve & Send", "Edit", "Discard".
  4. **Execution**: Tapping "Approve" executes the drafted action invisibly in the background.

  ### AI Agent Integration Points
  - **Work Triage**: Unifies messages and system alerts into the feed.
  - **Customer & Relationship Assistant**: Drafts replies for Instagram, etc., utilizing RAG on tenant memory.
  - **Operations Assistant**: Drafts inventory and booking tasks.

  ## Implementation Prompt
  **Objective**: Build the backend API, AI processing job queue, and Flutter mobile-first (375px) "Unified Agent Feed" that replaces traditional complex admin dashboards.
  **User-Facing Outcome**: When the owner opens the OHC app, they see a prioritized feed of Agent action cards instead of static charts.
  **Critical User Journey (CUJ)**:
  1. Simulate an incoming customer inquiry (e.g., via a mock webhook or internal event).
  2. The backend Worker processes this, queries tenant context, and drafts a reply.
  3. A new Action Card appears in the user's mobile feed (375px layout).
  4. User taps "Approve" -> The backend logs the approved action and executes it.
  **Acceptance Criteria**:
  - The feed correctly aggregates and displays generated Action Cards.
  - 100% Mobile-first UI (no horizontal scroll at 375px, 44x44px touch targets, premium OHC design tokens).
  - Background processing handles LLM integration asynchronously without blocking the UI.
  - E2E Playwright tests must verify the entire flow from event creation to UI approval.
  - No explicit DB schema prescribed; implementer to design multi-tenant isolated tables (`tenant_id`).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
