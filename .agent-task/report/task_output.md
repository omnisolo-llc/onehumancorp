issue_title: "Unified Agent Feed Mobile Parity & UI Enhancement"
issue_description: |
  **Problem Statement**
  The "Unified Agent Feed" is OHC's core differentiator, replacing complex dashboards with a mobile-first "Approval" interface. However, our competitive research indicates that legacy platforms force users back to desktop for complex operations. While we have the backend architecture (Agent Feed Repository & Service), the current implementation in `src/ui/tauri/src/ui/dashboard.html` falls short of the intended "Premium Translucent Glass" mobile experience. The UI can be clunky, touch targets are not explicitly robust across all dynamic elements, and the data-binding between the backend feed and the visual layer lacks the absolute seamlessness required for users like Maya (Home Baker) to manage their business entirely from a 375px viewport. The feed must be the singular "Work Command Center."

  **Research Report**
  As detailed in `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`, the Agent Feed must proactively push critical updates and drafted communications. Legacy platforms (Shopify, Wix) treat mobile apps as supplementary. OHC's Agent Feed must allow 100% of business operations via simple "Approve/Edit/Discard" cards. Our codebase currently maps these states (`lifecycle_state`, `payloads`), but the presentation and offline/sync resilience on mobile viewports need refinement to meet the "grandmother test."

  **Design Doc**
  *   **Architecture**:
      *   **Backend**: `src/server/services/agent_feed/service.rs` ingests events and uses the LLM (Minimax/Gemini) to classify intent and draft actions. `agent_feed_repo.rs` manages the persistence (`agent_feed_items`, falling back to legacy tables).
      *   **Frontend**: `src/ui/tauri/src/ui/dashboard.html` consumes `/api/ui/dashboard/unified-agent-feed` and renders the queue.
  *   **Mobile UX Flow (375px)**:
      *   User opens the app and lands directly on the Agent Feed (Triage section).
      *   Vertical feed of "Agent Proposals" (e.g., "Drafted response for customer X", "Low inventory alert").
      *   Each card features large (min 44x44px) touch targets for "Approve" (Primary Action) and "Dismiss" (Secondary Action).
      *   Cards utilize the OHC Premium Token library (glassmorphism: `rgba(255, 255, 255, 0.65)`, backdrop-blur).
  *   **AI Integration**: The feed visually distinguishes between departments (Operations, Sales, Customer Service) using subtle color coding or icons, maintaining the illusion of a coordinated team of assistants.
  *   **Key Decisions**:
      *   Enforce strict 375px layout constraints without horizontal scrolling.
      *   Ensure offline-tolerant rendering (cache feed state locally).

  **Implementation Prompt**
  **Objective**: Refine and harden the Unified Agent Feed in `dashboard.html` (and associated Tauri/Next.js components if applicable) to fully realize the mobile-first "Approval UI" paradigm.
  **Target Persona**: Maya the Home Baker, operating exclusively on her iPhone.
  **Critical User Journey (CUJ)**:
  1. Maya opens the OHC app and sees the "Needs Attention Today" feed.
  2. The feed displays a card: "New DM from Customer regarding Vegan Cake. [Drafted Response: 'Yes, we have vegan cakes!']".
  3. Maya taps the large, prominent "Approve & Send" button (min 44x44px touch target).
  4. The card animates out, the state updates to 'APPROVED' in the backend, and a subtle success toast appears.
  **Acceptance Criteria**:
  *   Verify and enforce that all interactive elements within the `triageQueue` container have a minimum touch target of 44x44px.
  *   Ensure the layout strictly adheres to 375px width constraints (no horizontal scrolling issues on mobile).
  *   Implement robust empty states ("All caught up!") that align with the premium glassmorphism design.
  *   Ensure the UI correctly maps and displays the `lifecycle_state` transitions (PENDING_APPROVAL -> APPROVED/DISMISSED) seamlessly.

  **Priority**: P0
  **Estimated Scope**: Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
