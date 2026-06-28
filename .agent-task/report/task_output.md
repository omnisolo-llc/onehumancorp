issue_title: "[Architectural Gap] Standardize and Consolidate Unified Agent Feed for Mobile-First Ops"
issue_description: |
  ## Problem Statement
  OHC aims to differentiate itself from legacy platforms (like Shopify and Wix) by enabling business owners to perform complex operations natively on a 375px mobile screen. This "Invisible AI Automation" vision centers around a "Unified Agent Feed"—a single, cohesive timeline where agents proactively surface actionable tasks, drafts, and notifications.

  However, our codebase currently fragments this experience. There are multiple overlapping endpoints (`/api/ui/dashboard/unified-feed`, `/api/ui/dashboard/unified-agent-feed`, `/api/agent-feed`) returning different data shapes, mixing legacy triage events (`triage-UUID`) with newer agent action requests and standard feed items. The frontend components (like `UnifiedAgentFeed.tsx` and `dashboard.html`) have convoluted logic to merge `triage`, `pending_approvals`, `agent_feed`, `approvals`, and `items` arrays, creating significant technical debt, schema inconsistencies, and edge-case bugs when updating item states.

  A true owner/operator (like Maya the Baker) expects one reliable feed where every card, regardless of whether it's an operations alert or a marketing draft, behaves consistently and can be approved/dismissed with a single tap.

  ## Research Report
  - **Shopify Sidekick / Wix Studio:** High power, but fundamentally tied to complex, nested desktop dashboard configurations.
  - **Durable / Lindy.ai:** Excellent mobile-first chat execution, but lack a clear, asynchronous "approval feed" for complex, backgrounded multi-step tasks.
  - **Current OHC State:** We have the backend queue and KAIROS orchestration, but the final mile—the mobile presentation layer—is severely fragmented. `load_ui_agent_feed_from_db` merges `agent_feed_items` with `agent_action_requests`, but UI caching logic across `UI_UNIFIED_AGENT_FEED_CACHE` and `UI_TRIAGE_CACHE` causes desyncs. The React/HTML clients are forced to implement complex normalization routines on the fly.

  ## Design Doc

  ### Architecture
  The goal is to consolidate the disparate feed sources into a single, standardized API endpoint and data model specifically optimized for the 375px mobile experience.

  ```mermaid
  graph TD;
      subagent[AI Agents / Departments] --> |Create Event| MsgBus[Message Bus];
      MsgBus --> |Process Event| FeedWorker[Feed Worker];
      FeedWorker --> |Write/Update| DB[(Unified Agent Feed Table)];
      LegacyTriage[Legacy Triage Systems] --> |Migration Layer| DB;
      ActionRequests[Agent Action Requests] --> |Normalization Layer| DB;
      DB --> |Read| UnifiedAPI[GET /api/v2/unified-feed];
      UnifiedAPI --> |JSON Array| MobileUI[OHC Mobile App - 375px Viewport];
  ```

  ### Core Data Model Improvements
  - **Unified `AgentFeedItem` Entity:**
    Every actionable card must adhere to a single structure:
    `id`, `tenant_id`, `event_source` (enum: Operations, Marketing, Support, Advisory), `title`, `description`, `context_payload` (JSON), `proposed_action` (JSON), `lifecycle_state` (PENDING, APPROVED, DISMISSED, RESOLVED).
  - **Deprecation of Legacy Endpoints:**
    Phase out `/api/ui/triage`, `/api/ui/dashboard/unified-feed`, and merge them fully into the standardized `/api/v2/unified-feed` endpoint.
  - **Cache Consolidation:**
    Unify the Redis caching strategy to invalidate a single `ui_feed:tenant_id` key upon *any* action approval/dismissal.

  ### Mobile UX Flow (375px)
  - The default app view is a continuous vertical feed of cards.
  - Each card visually indicates its source via color/icon (e.g., Marketing = Purple Sparkle, Ops = Blue Wrench).
  - The card displays a concise title ("3 new orders to fulfill") and a summary.
  - A primary massive action button (minimum 44x44px touch target) for "Approve".
  - A secondary, less prominent action for "Dismiss" or "Edit".
  - Approving a card triggers an optimistic UI fade-out and an asynchronous backend state update.

  ### AI Agent Integration Points
  - Agents interact *only* with the unified message bus to request human approvals. They do not directly write to the legacy `triage` or `agent_action_requests` tables anymore.

  ## Implementation Prompt
  **Mission for Implementer:** Consolidate the Agent Feed architecture to provide a single, unified, and mobile-optimized stream of actionable cards for the business owner.

  **Critical User Journey (CUJ):**
  1. Maya (Baker) opens the OHC app on her phone.
  2. The app fetches `GET /api/v2/unified-feed`.
  3. Maya sees a clean list containing an Instagram DM draft (Marketing) and an Inventory alert (Ops).
  4. She taps "Approve" on the draft.
  5. The card instantly disappears from the UI (Optimistic update).
  6. The backend processes the approval, triggers the IG reply, and invalidates the single, unified feed cache.

  **Acceptance Criteria:**
  - Introduce a unified Rust struct/DTO for feed items that normalizes `agent_feed_items`, legacy `triage`, and `agent_action_requests`.
  - Create a single, canonical API endpoint (e.g., `/api/v2/unified-feed`) that returns this standardized array.
  - Refactor the frontend (`UnifiedAgentFeed.tsx` and related Tauri HTML views) to consume this single endpoint and remove the convoluted multi-array merging logic.
  - Ensure all feed action buttons (Approve/Dismiss) route to a unified state-update endpoint that handles the downstream routing (e.g., fulfilling orders vs. dispatching LLM responses).
  - Maintain 100% Playwright E2E test coverage across viewport and interaction flows. Zero UI mocked data.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, mobile-first, architecture]
assignees: []
