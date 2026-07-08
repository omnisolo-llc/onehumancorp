issue_title: "Design Unified Agent Feed Architecture for Mobile-First (375px) Operations"
issue_description: |
  # Research Report: Unified Agent Feed Architecture for Mobile-First (375px) Operations

  ## Problem Statement
  Legacy platforms (like Shopify and Wix) treat mobile apps as supplementary dashboards for viewing stats while requiring desktop usage for store building and complex management. Business owners (like Fatima, the food cart operator, or Maya, the baker) require the ability to run their operations—from onboarding to daily management—entirely from their smartphones. When managing complex tasks (like setting up promotions or resolving support issues) on mobile, traditional dense settings menus fail due to screen constraints. A new UX paradigm is necessary for OneHumanCorp to achieve its "Mobile-First" and "Invisible AI Automation" goals.

  ## Research Report
  - **The Legacy Paradigm (Shopify, Wix):** Onboarding and complex tasks are desktop-oriented. While their mobile apps are good for viewing stats or fulfilling simple orders, configuring third-party apps, running promotions, or changing designs requires returning to a desktop browser.
  - **The "Link-in-Bio" Paradigm:** Tools like Linktree and Stan Store succeeded because they designed exclusively for mobile creators. However, they lack robust operations and agentic capabilities.
  - **The OHC Opportunity:** Replace the complex mobile administrative dashboard with an **"Agent Feed"**. Instead of navigating 20 toggles to run a discount, the user communicates intent, and the Agent drafts the logic. The UI simply presents a feed of "Action Cards" demanding approval. This reduces cognitive load and makes complex operations manageable on a 375px viewport.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[System Events / External Webhooks] --> B[Event Mesh]
      B --> C[AI Agents: Operations, Marketing, CS]
      C -->|Evaluate & Draft Context| D[Agent Action Queue]
      D --> E[Mobile Gateway API]
      E --> F[Unified Agent Feed UI - 375px]
      F -->|1-Tap Approve/Edit| G[State Mutation / Dispatch]
  ```

  ### Mobile UX Flow (375px First)
  - **Feed View:** A vertical stack of Action Cards. The first screen the owner sees upon opening the app.
  - **Card Design:** Each card uses macOS-style Translucent Glass materials (e.g., `backdrop-filter: blur(30px)`, rounded corners `16px`).
  - **Content Types:**
    - *Operations Card:* "3 new orders to fulfill. [Fulfill Now]"
    - *Marketing Card:* "Here is your generated Instagram post for the new cake. [Approve & Post]"
    - *CS/Advisory Card:* "It's been 30 days since your last promo. Should I draft an email? [Yes, draft it]"
  - **Interaction:** Tapping an action button (minimum touch target 44x44px) either executes the pre-drafted action or opens an inline expansion for quick edits. No deep menu navigation is required.

  ### AI Agent Integration Points
  - **Proactive Proposal Generation:** The `Marketing Agent` and `Customer Success Agent` continuously monitor tenant data (inventory deltas, communication latency) and generate structured "Proposals" placed onto the `Agent Action Queue`.
  - **Contextual Execution:** When an owner taps "Approve," the relevant agent executes the API calls, updates PostgreSQL, and invalidates necessary caches.

  ### Key Design Decisions
  - **Feed over Dashboards:** The primary interface is a chronologically and priority-sorted feed of proposed actions, not a grid of static charts.
  - **Read-Approve paradigm:** Move the user from creating data/configs to approving AI-drafted configs.
  - **Zero Trust:** Ensure strict multi-tenant isolation so an owner's feed only contains their specific agent proposals.

  ## Implementation Prompt
  **User-Facing Outcome:** When Fatima opens her OHC app, she doesn't see a complex menu of "Settings," "Products," and "Marketing." Instead, she sees a simple feed: "You sold out of chicken rice. Mark as Out of Stock? [Approve]." She taps it, and the system updates the storefront instantly.

  **CUJ & Acceptance Criteria:**
  1.  **Backend Services:** Implement a central `AgentFeedService` that aggregates proposals from various internal agents (Marketing, Operations) for a specific `tenant_id`.
  2.  **Mobile UI Shell:** Develop the "Unified Agent Feed" view using Flutter (or web equivalent targeting 375px), enforcing the Translucent Glass styling, and ensuring all buttons are ≥ 44x44px.
  3.  **Interaction:** Build the interactive logic where a user can tap "Approve" on an action card, which successfully dispatches a mutation to the backend and removes the card from the feed.
  4.  **Automated Verification:** Write Playwright E2E tests validating that the feed renders correctly on a 375px viewport, correctly displays 3 distinct types of agent cards, and processes a mock approval action successfully.
  5.  **Quality:** Achieve 100% unit test coverage for the new Feed Service. No horizontal scrolling should exist on the 375px display.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
