issue_title: "Implement Mobile-First Unified Agent Feed for OHC"
issue_description: |
  # Mission Queue Protocol: Mobile-First Unified Agent Feed

  ## Problem Statement
  Legacy e-commerce and business management platforms (Shopify, Wix) treat mobile apps as supplementary "dashboards" for viewing stats while requiring a desktop for actual store building and complex management. For OHC's target personas (e.g., Fatima the food cart owner, Carlos the handyman), operations happen 100% on a 375px mobile screen in the field. They cannot navigate complex hamburger menus with 20 toggles to set up discounts or reply to inquiries. They need an AI work assistant that unified all operational, marketing, and customer service tasks into a single actionable feed.

  ## Research Report
  **Market Gap Analysis**
  - **Shopify & Wix**: Onboarding is designed for desktop. The mobile apps are companions (good for checking stats, poor for building/editing).
  - **Link-in-Bio (Linktree, Stan Store)**: Mobile-first and simple, but lack robust inventory, booking, and agentic workflows.
  - **OHC Differentiator**: Instead of complex responsive design, OHC uses a **Chat & Approval UI** powered by Agents.

  ## Design Doc
  ### Mobile UX Flow (375px first)
  1. **Home/Feed Screen**: The user opens the app and sees a vertical stack of "Cards".
  2. **Card Structure**: Each card represents an action item from a specific AI Agent (e.g., Operations, Marketing, Advisory).
  3. **Interaction**: Instead of forms, cards have massive "Approve" or "Reject/Edit" buttons (touch target > 44px).

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile UI - Flutter/Tauri] -->|Fetch Feed| B(Feed Aggregator Service)
      B --> C{Agent Event Bus}
      C --> D[Operations Agent]
      C --> E[Marketing Agent]
      C --> F[Advisory Agent]
      D -->|Proposes Action| B
      E -->|Proposes Action| B
      F -->|Proposes Action| B
  ```

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors inventory and incoming orders, generating cards like "3 new orders to fulfill. [Fulfill Now]".
  - **Advisory Agent**: Monitors business health, generating cards like "It's been 30 days since your last promo. Should I draft an email? [Yes, draft it]".
  - **Marketing Agent**: Drafts copy and images for social media, generating cards for approval.

  ### Key Design Decisions
  - **Agent Proposals over Forms**: Moving away from complex admin dashboards to an inbox-style approval feed.
  - **Strict Mobile Width**: Enforced 375px width constraints to ensure accessibility on all entry-level devices.
  - **Glassmorphism & OHC Premium Tokens**: Clean UI that builds trust without overwhelming the user.

  ## Implementation Prompt
  **Objective**: Build the "Unified Agent Feed" mobile UI and the backend feed aggregator service.

  **CUJ**:
  1. User opens the app on a 375px screen.
  2. Sees a vertical feed of 3 agent proposal cards.
  3. Taps the primary action on a Marketing Agent card (e.g., "Yes, draft it").
  4. The card transitions to show the AI-drafted content with an "Approve & Send" button.

  **Acceptance Criteria**:
  - The feed UI is implemented using OHC Premium Tokens (translucent glass styling, correct typography).
  - Backend service aggregates at least two types of agent proposals.
  - 100% unit and Playwright E2E test coverage for the feed interactions.
  - No horizontal scrolling on 375px width. Touch targets >= 44x44px.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
