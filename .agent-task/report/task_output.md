issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  **Mission Queue Protocol Report**

  **Problem Statement**
  The legacy paradigm for managing a business (Shopify, Wix) relies heavily on complex desktop dashboards. Mobile apps for these platforms are often "companion apps" meant for viewing stats or fulfilling orders, not for primary operations. This forces owners (like Fatima, who runs a food truck and only has an Android phone) to navigate tiny, complex UI or wait until they have a laptop. We need a way to manage a complex business on a small screen without clutter.

  **Research Report**
  - Our research (Track 1 & 2) in `ohc_smb_mobile_first_design_research.md` highlights that link-in-bio tools succeed because of absolute simplicity and touch-friendly components. However, they lack robust business features.
  - The solution to complex mobile UI is not traditional responsive forms but an **Approval UI** powered by Agents.
  - OHC must enable 100% of operations on a 375px screen.

  **Design Doc**
  - **Architecture Diagram (Mental Model)**:
    - `Event Pipeline (Webhooks, internal state)` -> `Intent & Context Resolution (LLM)` -> `Agent Feed Action Cards`.
    - AI Agents (Operations, Marketing, Advisory) process events and generate actionable proposals ("Action Cards").
  - **Mobile UX Flow (375px)**:
    - Instead of a complex dashboard, the default view is a vertical feed of Action Cards.
    - Each Card presents a drafted action or urgent item (e.g., "3 new orders to fulfill", "Drafted Instagram post for new cake").
    - Cards have massive (min 44x44px) actionable buttons: "Approve", "Edit", "Discard".
  - **AI Agent Integration**:
    - The feed aggregates outputs from all OHC Agents.
    - When "Approve" is clicked, the specific agent executes the workflow.

  **Implementation Prompt**
  **Objective**: Build a mobile-first (375px) "Unified Agent Feed" that replaces the traditional complex admin dashboard. This feed presents actionable cards from all OHC Agents.
  **User-Facing Outcome**: When the user opens the OHC app, they see a vertical feed of "Agent Proposals" and "Urgent Items" instead of a static graph.
  **Critical User Journey (CUJ)**:
  1. User opens the app on a simulated 375px screen.
  2. The feed displays action cards (e.g., Operations: Fulfill orders, Advisory: Draft email).
  3. User taps an action (e.g., "Yes, draft it").
  4. The card expands to show the drafted content with an "Approve & Send" button.
  **Acceptance Criteria**:
  - Layout strictly adheres to 375px width constraints (no horizontal scrolling).
  - All interactive elements have minimum 44x44px touch targets.
  - Uses OHC Premium Tokens (Glassmorphism, specific typography).
  - Feed clearly distinguishes between different agent types.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
