issue_title: "Implement the Unified Mobile-First Agent Feed (375px MVP)"
issue_description: |
  ## Problem Statement
  Legacy commerce platforms (Shopify, Wix) treat mobile apps as supplementary dashboards for viewing stats or fulfilling simple orders, forcing users to rely on desktop environments for complex business operations. Small business owners, creators, and operators—like Fatima the food truck owner or Maya the baker—often manage their entire businesses from a smartphone. They need an application that brings all critical operations into a single, mobile-first view. The current OHC lacks a unified interface where owners can quickly view, approve, and execute AI-drafted tasks without navigating through complex menus or desktop web interfaces.

  ## Research Report
  - **The Legacy Paradigm**: Shopify and Wix separate complex actions from their mobile apps. Setting up a discount code or managing an email campaign requires a desktop browser.
  - **The Link-in-Bio Paradigm**: Tools like Linktree and Stan Store succeed because they are built exclusively for mobile creators, using simple, touch-friendly UI. However, they lack robust operations and agentic capabilities.
  - **The OHC Differentiator**: OHC uses autonomous agents (Marketing, Operations, Advisory) to handle complex work. Instead of making the user fill out complex forms on a phone, OHC agents propose actions. The UX gap is the lack of a centralized "Agent Feed" to display these proposals.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Agent Backend: Marketing, Operations, Success] -->|Draft Action| B(Event Bus / Queue)
      B --> C[Agent Feed Service]
      C -->|Action Proposal| D(Mobile Client - 375px)
      D -->|User Taps 'Approve'| C
      C -->|Dispatch Execution| E(KAIROS Orchestrator)
      E --> F[Database / APIs]
  ```

  ### UI Wireframes or Screen Flow Description (375px first)
  - **Main Feed View (375px width)**: A vertically scrollable list. The top has a "Good morning, Maya" greeting and a summary of urgent tasks.
  - **Card Component**: Full-width cards (with 16px margins), displaying:
    - **Header**: Icon (e.g., Megaphone for Marketing) + "Marketing Agent".
    - **Body text**: E.g. "We noticed you haven't posted in 3 days. I drafted a post for the new vegan cupcakes."
    - **Action Row**: A large primary button (e.g. "Review Post") measuring at least 44px in height.

  ### Mobile UX Flow
  1. **Home Screen (The Feed)**: The user opens the app and sees a scrolling feed of agent-drafted action cards.
  2. **Interaction**: User taps "Review Post" on a Marketing card.
  3. **Detail Overlay**: A translucent modal slides up over the feed showing the drafted Instagram post and an image.
  4. **Approval**: User taps a massive "Approve & Post" button (44x44px minimum touch target).
  5. **Completion**: A success toast appears, the card disappears from the feed, and the user returns to the remaining feed items.

  ### AI Agent Integration Points
  - **Event Bus**: The feed subscribes to a central event stream or database table containing actions drafted by various agents.
  - **Agent Backends**: Marketing, Operations, and Customer Success agents must expose an API or publish structured data (Action Proposals) to the feed system.

  ### Key Design Decisions and Why
  - **Glassmorphism & OHC Premium Tokens**: Applying Apple-style translucent blur (`backdrop-filter: blur(20px)`) ensures a modern, premium feel that doesn't overwhelm the user with stark blocks.
  - **Action-Oriented Feed instead of Dashboards**: We avoid charts and lists. Business owners like Maya need to *do* things, not *analyze* things on a 375px screen. Thus, the feed pushes concrete proposals.

  ## Implementation Prompt
  1. Build a new mobile-first interface component: the "Unified Agent Feed".
  2. Implement the UI using standard responsive web technologies, ensuring perfect rendering on a 375px viewport.
  3. Create a data structure and mock integration (or real integration if backend exists) for "Action Cards". A card should have a title, an agent source (e.g., Marketing, Operations), description text, and primary/secondary action buttons.
  4. Ensure all touch targets (buttons) are at least 44x44px.
  5. Apply the required "Glassmorphism" styling (backdrop blur, subtle borders, 16px border radii).
  6. **Acceptance Criteria**: The user can open the feed on a mobile device (375px), scroll through at least three different types of agent cards, and tap a button on a card to see a mock success/approval state. Do not prescribe specific backend implementations.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
