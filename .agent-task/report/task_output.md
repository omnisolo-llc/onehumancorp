issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  ## Research Report: Unified Agent Feed (Mobile MVP)

  ### Title
  Implement the Unified Agent Feed (Mobile MVP)

  ### Problem Statement
  Small business owners are currently forced to manage their operations through complex desktop dashboards or multi-layered mobile menus that are difficult to navigate on a small screen (375px). Legacy platforms treat mobile apps as supplementary tools for viewing stats, while OHC needs to enable 100% of business operations—from setup to execution—on a phone. The current UI paradigm requires the owner to seek out information or initiate actions manually, which is counter to the "Invisible AI Automation" philosophy.

  ### Research Report
  - **Market Context**: Legacy platforms like Shopify and Wix offer mobile apps, but these apps often redirect users to a web browser view for complex tasks (like setting up discounts or editing store design), creating a frustrating experience for users who only have a phone (e.g., Fatima the food cart operator). Link-in-bio tools succeed because of their simplicity on mobile, but lack full business platform capabilities.
  - **The OHC Opportunity**: OHC can differentiate by replacing the traditional complex admin dashboard with a **Unified Agent Feed**. Instead of a static graph and a hamburger menu, the feed presents actionable cards proactively pushed by AI agents (Marketing, Operations, Advisory, Customer Success). The owner simply reviews and approves actions with a single tap.
  - **Competitor Gaps**: Shopify's Sidekick is a reactive chatbot; users have to know what to ask. Wix's AI is focused on initial setup. No platform currently offers a proactive, feed-based approval interface that acts as the central nervous system for running the business.

  ### Design Doc
  - **Architecture**:
    - **Event Ingestion Pipeline**: Ingests events from webhooks (Stripe, Instagram), internal state changes (inventory, orders), and scheduled jobs.
    - **Intent & Context Resolution (LLM Layer)**: Classifies intent, queries user's business data (RAG), and generates drafted responses/actions.
    - **Notification & Approval UX**: Pushes "Action Cards" to the Agent Feed.
  - **Mobile UX Flow (375px First)**:
    - **Home Feed**: The first screen after login is the Agent Feed. It displays a vertical list of cards.
    - **Action Cards**: Each card represents a proposed action or urgent update from an agent (e.g., "3 new orders to fulfill", "Drafted reply to Sarah's Instagram DM", "Let The Promoter optimize your store for local Google searches?").
    - **Interaction**: Cards have clear, massive buttons (touch targets > 44px) like "Fulfill Now", "Approve & Send", "Yes, draft it".
    - **Visual Design**: Adheres to OHC Premium Tokens (Glassmorphism, clean typography, clear status tokens). No horizontal scrolling. Distinct visual cues for different agent types.

  ### Implementation Prompt
  **User-Facing Outcome**: When the user opens the OHC app, instead of seeing a static graph and a complex hamburger menu, they see a vertical feed of "Agent Proposals" and "Urgent Items." They can run their entire business by simply scrolling and tapping "Approve" on their phone.

  **CUJ & Acceptance Criteria**:
  1. User opens the app on a simulated 375px screen.
  2. The feed displays a list of simulated Action Cards from different agents (e.g., Operations: "New Order", Advisory: "Draft Promo Email", Customer Success: "Approve Instagram Reply").
  3. The layout strictly adheres to 375px width constraints (no horizontal scrolling).
  4. All interactive elements (buttons, cards) have minimum 44x44px touch targets.
  5. User can tap an action button (e.g., "Approve") on a card, and the UI provides visual feedback (e.g., card disappears or shows a success state).
  6. Include automated E2E Playwright tests verifying the feed rendering and the approval interaction flow on a mobile-sized viewport.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
