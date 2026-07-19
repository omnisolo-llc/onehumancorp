issue_title: "Implement Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Mission Queue Protocol: Unified Agent Feed (Mobile MVP)

  ## Problem Statement
  Small business owners and operators (our personas like Maya, Carlos, Fatima) operate primarily from mobile devices (375px screens). However, existing commerce platforms force them into complex, desktop-oriented dashboards filled with graphs and deep navigation menus to manage their operations, marketing, and customer relationships. They don't want a "dashboard"; they want an assistant that tells them what to do next. The current OHC dashboard needs to evolve from a passive reporting tool into an active, agent-driven "Approval Feed."

  ## Research Report
  Based on our market research (`ohc_smb_mobile_first_design_research.md` and `[research]_ohc_smb_mobile_first_agentic_workflows.md`), legacy systems like Shopify and Wix offer excellent desktop functionality but fail the "in-the-field" test for micro-SMEs. Conversely, Link-in-Bio tools are simple but lack operational depth.
  The critical differentiator for OHC is **Invisible AI Automation**. Instead of configuring complex settings, the system should generate actionable proposals. Our agents (Operations, Marketing, Advisory) generate these proposals, which must be surfaced in a unified, mobile-first feed where the primary user interaction is simply "Approve" or "Dismiss."

  ## Design Doc
  ### Architecture Diagram (Concept)
  ```mermaid
  graph TD
      A[Operations Agent] -->|Generates Tasks| D(Unified Agent Feed DB/Cache)
      B[Marketing Agent] -->|Drafts Posts/Emails| D
      C[Advisory Agent] -->|Suggests Actions| D
      D --> E{OHC Mobile Client 375px}
      E -->|User taps Approve| F[Agent Execution Pipeline]
      E -->|User taps Dismiss| G[Dismissal/Learning]
  ```

  ### Mobile UX Flow
  1.  **Launch**: User opens the OHC PWA/App. The default view is the **Unified Agent Feed**.
  2.  **Triage**: Instead of a "Dashboard" with charts, the screen is a vertical, scrollable list of "Cards".
  3.  **Interaction**: Each Card represents a discrete agent proposal (e.g., "Drafted Instagram Post for new cake", "3 orders need fulfillment", "Suggest 10% discount to inactive subscribers").
  4.  **Action**: The cards feature massive, touch-friendly buttons (minimum 44x44px). The primary button is "Approve" or a contextual equivalent ("Yes, draft it", "Publish").
  5.  **Completion**: Upon approval, the card animates away (or shows a success state), and the agent handles the background execution.

  ### Key Design Decisions
  *   **Mobile-First Strictness**: The UI must be perfectly usable on a 375px width. No horizontal scrolling.
  *   **Touch Targets**: All interactive elements (especially the primary 'Approve' action) must be at least 44x44px to accommodate "fat-finger" interactions in busy environments (e.g., a food truck).
  *   **Visual Language**: Employ the OHC Premium Tokens—macOS-style Translucent Glass materials (`backdrop-blur`, `rgba(255,255,255,0.65)`) over the dark/light mode background.

  ### AI Agent Integration Points
  *   The feed acts as the presentation layer for the `ui_dashboard_unified_agent_feed_handler` (API `/api/v1/ui/dashboard/unified-agent-feed`).
  *   It must gracefully handle different `feature_type` payloads from various agents (e.g., `proactive_ops`, `social_post_draft`, `subscription_replenishment`).

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Refine and harden the "Unified Agent Feed" on the mobile dashboard (specifically targeting the Tauri/Web client at `src/ui/tauri/src/ui/dashboard.html` and related frontend assets).

  **User-Facing Outcome**: When the user logs in, they see a clean, glassmorphic feed of actionable items proposed by their AI agents. They can easily tap "Approve" on a 375px screen to execute complex background tasks.

  **Critical User Journey (CUJ)**:
  1.  User opens the app on a simulated 375px screen.
  2.  The feed displays actionable cards from various agents (Operations, Marketing).
  3.  The user taps the primary "Approve" button on a specific card (e.g., to approve a drafted social media post or an operational checklist).
  4.  The system acknowledges the action, and the agent executes the task in the background.

  **Acceptance Criteria**:
  -   The feed layout strictly adheres to 375px width constraints (no horizontal scrolling).
  -   All interactive buttons (Approve, Dismiss) MUST have minimum 44x44px touch targets.
  -   The design must utilize OHC Premium Tokens (Glassmorphism, clean typography, UniFi-style modular cards).
  -   Playwright E2E tests (`src/e2e/tests/unified_agent_feed_mobile.spec.ts`) must pass, specifically verifying the presence of the feed, the minimum touch targets (`min-h-[44px]`), and the successful interaction with an approval flow. Ensure NO mock data is used; rely on the `e2e-seed.sql` generated feed items.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []