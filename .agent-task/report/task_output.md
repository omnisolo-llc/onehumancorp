issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Research Report & Design Doc: The Unified Agent Feed (Mobile MVP)

  ## Target Persona: Maya (Home Baker) & Fatima (Food Cart Operator)

  ## Problem Statement
  Small business owners operate entirely from their phones. Legacy platforms (Shopify, Wix) treat mobile apps as supplementary "dashboards" for viewing stats while requiring a desktop browser for actual store building and complex management. OHC must enable 100% of business operations—from initial setup to daily execution—on a 375px mobile screen. Complex UI forms (e.g. setting up a discount code) feel cluttered and confusing on a phone. The solution is an "Approval" interface powered by Agents.

  ## Research Findings
  Based on competitive analysis, traditional commerce apps are excellent for order fulfillment but poor for managing complex tasks (like setting up promotions or designing store layouts) on mobile. Link-in-bio tools are successful because of absolute simplicity and touch-friendly UI components, but lack robust commerce features. OHC's unique differentiator is its Agentic Workflows. By shifting from a "Settings Dashboard" paradigm to a "Unified Agent Feed," OHC can hide complexity.

  ## Design Doc: The Unified Agent Feed
  Instead of a complex form to set up a discount code, an Agent drafts the logic and presents a simple "Card" in the feed.

  ### Architecture
  - **Data Source**: An aggregated queue of "Agent Proposals" (from Operations, Marketing, Advisory agents).
  - **Mobile UX Flow (375px)**:
    1. User opens the app.
    2. The main screen is a vertical feed of "Action Cards."
    3. Each card represents a proposed action or urgent item (e.g., "3 new orders to fulfill", "Drafted email promo for review").
    4. Cards have a primary action button (e.g., "Approve & Send", "Fulfill Now") and secondary actions (e.g., "Edit", "Discard").
    5. Layout strictly adheres to 375px width constraints (no horizontal scrolling).
    6. Minimum 44x44px touch targets.
  - **AI Agent Integration**: Agents push actionable intents to a unified `AgentFeed` queue, specifying the context, drafted content, and required user action.

  ## Implementation Prompt
  - **Objective**: Build a mobile-first (375px) "Unified Agent Feed" that replaces the traditional complex admin dashboard. This feed presents actionable cards from all OHC Agents.
  - **User-Facing Outcome**: When the user opens the app, instead of seeing a static graph and a complex hamburger menu, they see a vertical feed of "Agent Proposals" and "Urgent Items."
  - **CUJ (Critical User Journey)**:
    1. User opens the app on a simulated 375px screen.
    2. The feed displays 3 cards: Operations ("3 new orders"), Advisory ("Draft promo email?"), Marketing ("Instagram post ready").
    3. User taps an action button on a card (e.g., "Yes, draft it").
    4. The card expands to show the draft, with a final "Approve" button.
  - **Acceptance Criteria**:
    - Layout strictly adheres to 375px width constraints (no horizontal scrolling).
    - All interactive elements (buttons, cards) have minimum 44x44px touch targets.
    - Uses OHC Premium Tokens (Glassmorphism, specific typography).
    - Provide Playwright E2E tests verifying the feed renders and actions can be tapped.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
