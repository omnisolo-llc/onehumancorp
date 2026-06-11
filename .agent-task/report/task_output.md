issue_title: "Implement Mobile-First Unified Agent Feed & Optimistic Action Dispatcher"
issue_description: |
  # Problem Statement
  Small business owners (e.g., Maya, Carlos, Fatima) are overwhelmed by complex, multi-tab admin dashboards inherited from legacy systems (Shopify, Wix). They run their businesses on mobile (375px screens) and need actionable insights, not raw data. Currently, they miss critical sales because they have to hunt for information across booking, messaging, and inventory screens. There is no central, AI-curated nervous system that proactively pushes high-context, single-tap actions (like approving an AI-drafted reply or authorizing a discount) to the user's mobile device with offline-first resilience.

  # Research Report
  **Market Competitive Analysis:**
  - **Shopify/Wix:** Rely on a "Companion App" model where mobile is mainly for viewing stats; deep configuration requires a desktop. AI is relegated to a chat side-panel (e.g., Shopify Sidekick) which advises rather than acts.
  - **Square:** Leads in offline POS but lacks a proactive, agent-driven work feed that coordinates tasks across customer service, finance, and marketing.
  - **Link-in-Bio (Linktree, Stan Store):** Excel at mobile-first simplicity and big touch targets, but lack robust back-office logic, multi-tenant coordination, and agentic workflows.

  **OHC Discovery & Gap:**
  Our codebase and docs (`ohc_smb_mobile_first_design_research.md`, `agent_feed_deep_dive.md`) identify the need for an "Approval" Interface Paradigm, but the underlying multi-tenant data structures, background sync coordination (CRDT/Optimistic UI), and AI department handoffs are not yet implemented end-to-end. OHC must pivot from an advisory AI model to an **Executing AI Model**, where agents draft the work and push it to a unified, offline-capable feed for a single-tap owner approval.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Webhook as Event Sources (IG, Stripe)
      participant Ingestion as Event Pipeline (Redis/Kafka)
      participant AI_Agents as AI Departments (Ops, CS, Sales)
      participant Action_Queue as Unified Action Queue (Postgres/CRDT)
      participant Mobile_App as OHC Mobile App (Flutter)

      Webhook->>Ingestion: Incoming Event (e.g. New IG Message)
      Ingestion->>AI_Agents: Route to specific Agent (e.g. CS Ambassador)
      AI_Agents->>AI_Agents: Intent Classification & RAG Context Retrieval
      AI_Agents->>Action_Queue: Create Proposed Action Card (Drafted Reply)
      Action_Queue-->>Mobile_App: Sync to Local SQLite (Offline-Capable)
      Mobile_App->>Mobile_App: Display 375px Action Card in Unified Feed
      Mobile_App->>Action_Queue: User Taps "Approve & Send" (Optimistic Sync)
      Action_Queue->>AI_Agents: Dispatch Approved Action
      AI_Agents->>Webhook: Execute Action via External API
  ```

  ## Mobile UX Flow (375px Viewport)
  1. **The Unified Feed:** Upon opening the app, the user sees a vertical, chronological feed of premium, translucent glassmorphism cards (macOS style + UniFi layout). No complex navigation menus.
  2. **Action Cards:** Each card represents an agent's proposal (e.g., "Drafted response for Maya: Yes, we have vegan cakes!").
  3. **Touch Targets:** Large, accessible actions (44x44px minimum): "Approve", "Edit", "Discard".
  4. **Optimistic UI:** When the user taps "Approve", the card instantly animates out of the feed with a soft haptic feedback. If offline, the action is queued locally (SQLite/CRDT) and a small "Pending Sync" pill appears.
  5. **Zero Technical Jargon:** Errors or state conflicts are caught by the AI Operations Agent and represented as a new advisory card ("We couldn't send the message to Carlos. Want to try SMS instead?").

  ## AI Agent Integration Points
  - **Event Ingestion:** The Event Ingestion Pipeline routes normalized business events to the appropriate AI department (Operations, CS, Finance).
  - **RAG Context:** Agents query the tenant's localized embeddings (policies, inventory) to generate highly accurate action drafts.
  - **Conflict Resolution:** If a user approves an action offline but the backend state has drifted, the Ops Agent intercepts the failure and generates a fallback proposal card.

  ## Key Design Decisions
  - **Mobile-First / 375px Baseline:** Desktop is additive. If a feature cannot be managed via a simple action card on mobile, it must be redesigned.
  - **Offline-First (Local SQLite + CRDT):** Business operators (e.g., food carts, on-site handymen) experience frequent network drops. Action approvals must be optimistic and sync asynchronously.
  - **Agent as Executors, Not Chatbots:** We avoid conversational chat boxes. Agents do the work in the background and present the final draft for approval, minimizing cognitive load.

  # Implementation Prompt
  **User-Facing Outcome:** A mobile business owner opens their app and immediately sees a prioritized feed of drafted actions (replies, quotes, scheduling conflicts) generated by AI agents. They can clear their workload with single taps.

  **Critical User Journey (CUJ):**
  1. An external event occurs (e.g., a customer asks about a sold-out item).
  2. The AI CS Agent drafts a response ("Sorry, we're sold out, but here's a 10% discount for tomorrow") and creates an Action Card.
  3. The owner opens the app (potentially offline), sees the card, and taps "Approve".
  4. The card optimistically dismisses. Once online, the action syncs and the message/discount is sent automatically.

  **Acceptance Criteria:**
  - Implement the unified feed UI in Flutter (strictly adhering to 375px width, 44x44px touch targets, and OHC design tokens).
  - Build the local SQLite queue for optimistic action approval and background synchronization.
  - Implement the backend pipeline to ingest events, route them to an AI agent, and generate a standardized Action Card payload in Postgres with row-level tenant isolation.
  - Add Playwright E2E tests simulating the full flow from event ingestion to mobile UI approval. ZERO mock data in the UI.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
