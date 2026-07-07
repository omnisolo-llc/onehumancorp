issue_title: "Implement the Unified Agent Feed (Mobile-First 375px MVP)"
issue_description: |
  ## Issue Title
  Implement the Unified Agent Feed (Mobile-First 375px MVP)

  ## Problem Statement
  Legacy business platforms require non-technical owners to navigate complex dashboards, tabs, and analytics graphs on small screens. For personas like Fatima (food cart) or Carlos (field service) running operations entirely from a 375px mobile device, this interface is overwhelming and unusable. They need to know *what to do right now* and have a simple way to take action, rather than being forced to extract insights from raw data displays.

  ## Research Report
  Researching traditional platforms (Shopify, Wix) vs creator tools (Linktree) reveals a core "Mobile Management Gap". Legacy platforms rely heavily on web applications to complete setup or review actionable changes, using companion mobile apps mostly for read-only tracking or single-task workflows.

  Our target is a "Chat & Approval UI" built natively for a 375px mobile viewport. In this paradigm, complex configurations (e.g., setting up discounts or replying to multiple Instagram DMs) are condensed into actionable "Agent Proposals." Instead of modifying settings manually, the owner reads a summary and taps a prominent "Approve" button.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Agent Departments: Marketing, Ops, CS] -->|Generate Proposals| B[Unified Feed Service]
      B --> C[Mobile App Feed UI]
      C -->|Owner Reviews Card| D{User Action}
      D -->|Tap Approve| E[Agent Executes Action]
      D -->|Tap Dismiss| F[Agent Learns/Discards]
      E --> G[Business State Updated]
  ```

  ### Mobile UX Flow (375px Width Target)
  1. **Home/Feed Screen:** A clean vertical feed of "Agent Cards" instead of a static dashboard or standard navigation menu.
  2. **Card Structure:**
     - **Context/Origin:** Which agent generated this? (e.g., Marketing, Ops).
     - **Summary Message:** Simple text explaining the current situation and the proposed action (e.g., "3 new cake orders need delivery assignment. [Assign Now]").
     - **Action Buttons:** Large touch targets (≥ 44x44px). Primary action (Approve/Accept), secondary action (Edit/Dismiss).
  3. **Interaction:** Tapping "Approve" transitions the card to a "Processing..." state, then to a success state.
  4. **Glassmorphism Theme:** Cards use OHC Premium Tokens—translucent backgrounds overlaying a subtle dynamic gradient to maintain visual clarity without feeling heavy.

  ### AI Agent Integration Points
  - **Operations Agent:** Pushes cards for inventory alerts, fulfillment tasks, and schedule conflicts.
  - **Marketing Agent:** Pushes cards with pre-generated social media posts or email drafts based on inventory changes or time since last promo.
  - **CS Agent (Ambassador):** Pushes cards with drafted replies to urgent DMs or customer emails.

  ## Implementation Prompt
  Implement the "Unified Agent Feed" mobile interface MVP for the OHC Flutter/Tauri app. The objective is to construct a responsive, vertical list of action cards on the primary dashboard, specifically tested for a 375px screen width constraint.

  **CUJ & Acceptance Criteria:**
  - Build the main Feed View showing distinct agent proposal cards.
  - Construct real data bindings for at least 3 distinct card types (Operations, Marketing, Customer Success) using database seeds or real backend integrations (ZERO Mock Data allowed).
  - Every card must have a clear actionable button (e.g., "Approve") with a touch target of at least 44x44px.
  - Ensure horizontal scrolling is completely disabled.
  - Apply the OHC design system's translucent materials (glassmorphism) and spacing guidelines.
  - Clicking "Approve" should immediately update the UI state to reflect a successful action.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
