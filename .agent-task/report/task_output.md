issue_title: "Implement Agent Feed for Mobile-First Approval UI"
issue_description: |
  # Research Report: The Agent Feed (Mobile-First Agent Orchestration)

  ## 1. Problem Statement
  Legacy business platforms present users with complex dashboards designed for desktop. For OHC's mobile-first owner personas (Maya the baker, Carlos the handyman, Fatima the food cart operator), navigating deep menus to execute complex operations (e.g., launching a discount, replying to customer inquiries) on a 375px screen is a major friction point. They need actionable business insights and AI recommendations pushed to them, rather than having to seek them out.

  ## 2. Research Findings & Gap
  Based on the competitive audit (`docs/business/market_research/ohc_smb_mobile_first_design_research.md`) and the Agent Feed deep dive (`docs/business/market_research/agent_feed_deep_dive.md`), there is a critical need for an "Agent Feed."
  - **Gap**: OHC currently lacks a unified feed UI on the frontend and an event-driven agent orchestration mechanism on the backend to propose actionable drafts to the user.
  - **Goal**: Implement a mobile-first (375px) feed of "Action Cards" that present agent-generated proposals (draft replies, order fulfillments, marketing promos) with one-tap approval actions.

  ## 3. Design Doc: The Unified Agent Feed

  ### Architecture
  - **Backend (Go)**:
    - `AgentFeedService`: Manages the retrieval and state (pending, approved, dismissed) of action cards for a tenant.
    - **Event Pipeline**: Listen to business events (new order, customer message) via Kafka/Redis PubSub.
    - **Agent Coordination**: When an event occurs, the relevant agent (e.g., Operations Agent for orders, Customer Success Agent for messages) processes it, utilizes RAG for context, and generates a proposed action.
    - **PostgreSQL**: Store action card entities with `tenant_id` for row-level security.
  - **Frontend (Flutter/PWA)**:
    - **Mobile-First Layout (375px)**: A vertical scrolling feed of cards. No complex navigation menus.
    - **Action Cards**: Each card displays:
      - Source Agent (Icon/Label)
      - Context (e.g., "Customer asked about vegan cakes")
      - Proposed Action/Draft (e.g., "Yes, we have them. Would you like to order?")
      - Primary Action Button (e.g., "Approve & Send" - minimum 44x44px touch target)
      - Secondary Action (e.g., "Edit" or "Dismiss")
    - **UI Tokens**: Apply OHC Premium Tokens (Glassmorphism, clean typography) for a professional look.

  ### Mobile UX Flow
  1. Owner opens the app and sees the "Today" feed.
  2. A new card from the Customer Success Agent appears: "Maya, 3 inquiries about custom cakes overnight. I drafted replies."
  3. Maya taps the card to expand it, seeing the first drafted reply.
  4. She taps the prominent "Approve" button. The message is sent, and the card is dismissed.

  ### AI Agent Integration Points
  - **Triggers**: Connect internal events (new chat message, low inventory) to trigger agent workflows.
  - **Context Gathering**: Agents query the tenant's knowledge base (policies, inventory) before drafting.
  - **Output**: Agents generate a structured payload representing the "Action Card" (title, body, proposed action, callback payload) to be inserted into the feed.

  ## 4. Implementation Prompt
  **Target Implementer**: Frontend/Backend Fullstack Engineer
  **Task**: Build the full-stack MVP of the "Agent Feed" for the OHC mobile app.
  **Acceptance Criteria**:
  - Create the necessary database schemas (with multi-tenant isolation) to store Agent Action Cards.
  - Implement a backend API endpoint (`/api/feed`) to fetch pending action cards for the authenticated user.
  - Implement a mobile-first Flutter/PWA UI that displays these cards in a vertical list, ensuring all interactive elements are at least 44x44px and fit within a 375px viewport without horizontal scrolling.
  - Implement the "Approve" action flow that updates the card status and triggers a simulated backend callback.
  - Include comprehensive Playwright E2E tests verifying the feed rendering and interaction flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
