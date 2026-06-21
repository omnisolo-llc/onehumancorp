issue_title: "Implement Unified Agent Feed for Mobile Operations"
issue_description: |
  # Unified Agent Feed for Mobile Operations

  ## Problem Statement
  Currently, complex business operations (like setting up promotions, reviewing agent-generated content, and managing inventory) are difficult to execute on a small 375px mobile screen. Legacy paradigms rely on complex dashboards and deep navigation trees, which are poorly suited for the mobile-first operational reality of modern creators and business owners (like Maya the baker or Fatima the food cart operator). The lack of a streamlined, action-oriented mobile interface creates friction and prevents owners from executing business-critical tasks smoothly while away from a desktop.

  ## Research Report
  Our competitive analysis of platforms like Shopify and Wix reveals a significant gap: they treat their mobile apps as secondary "companion apps," often forcing users back to a desktop browser for complex configurations. Conversely, "link-in-bio" tools like Linktree and Stan Store succeed because they are built purely for mobile, but they lack the depth needed for full business operations.

  To bridge this gap, OHC needs to pioneer the **"Approval Interface Paradigm."** Instead of navigating complex forms, the owner should interact with proactive Agents through a **Unified Agent Feed**. This feed will present prioritized, contextual cards (e.g., an Operations Agent highlighting orders, an Advisory Agent suggesting promotions) where complex background tasks are summarized into simple, one-tap approvals.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Mobile UI 375px] -->|Fetches Feed| B(Unified Feed API)
      B --> C{Agent Orchestrator}
      C -->|Retrieves Tasks| D[Operations Agent]
      C -->|Retrieves Prompts| E[Marketing Agent]
      C -->|Retrieves Alerts| F[Advisory Agent]
      D --> G[(Task DB)]
      E --> G
      F --> G
      A -->|Approves Action| B
      B --> C
      C -->|Executes| H[Execution Engine]
  ```

  ### UI Wireframes & Mobile UX Flow
  *   **Viewport**: Strict 375px width (no horizontal scrolling).
  *   **Layout**: A vertical, scrollable list of "Agent Proposal Cards."
  *   **Card Structure**:
      *   **Header**: Agent Avatar/Icon and Name (e.g., "Operations", "Marketing").
      *   **Body**: Concise summary of the situation or proposal (e.g., "3 new orders need fulfillment," "Drafted a 20% off promo email").
      *   **Action Area**: Large, touch-friendly primary button (minimum 44x44px target) for "Approve," "Fulfill," or "Review." Secondary actions (like "Dismiss" or "Edit") should be accessible but less prominent.
  *   **Interactions**: Tapping "Approve" triggers the background execution and provides immediate, optimistic UI feedback (e.g., card transitions to a "Done" state). Tapping to edit expands the card to reveal deeper, but still mobile-optimized, controls.
  *   **Styling**: Utilize OHC Premium Tokens, including Glassmorphism effects and clean typography, to ensure a high-quality feel.

  ### AI Agent Integration Points
  The feed acts as the presentation layer for the underlying Agent Orchestration system. It relies on:
  1.  **Agent Task Ingestion**: Agents (Marketing, Ops, etc.) must publish actionable items to a centralized feed datastore.
  2.  **Contextual Summarization**: Agents must provide concise, owner-friendly summaries of complex tasks.
  3.  **Approval Callbacks**: The system needs a robust mechanism to handle the "Approve" action, triggering the corresponding Agent's execution flow reliably.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the presentation and API layer for the "Unified Agent Feed," focusing strictly on a mobile-first (375px) experience.

  **Critical User Journey (CUJ):**
  1.  A user logs into the OHC mobile app.
  2.  Instead of a traditional dashboard, they land directly on the Unified Agent Feed.
  3.  They see prioritized cards representing pending actions from different agents (e.g., an Operations card for new orders, a Marketing card proposing a social post).
  4.  The user taps the primary "Approve" button on a Marketing proposal card.
  5.  The UI optimistically updates to reflect the approval, and the backend triggers the corresponding agent workflow.

  **Acceptance Criteria:**
  *   Implement a vertical feed layout that is strictly constrained to 375px width and passes visual inspection for mobile usability.
  *   Design and implement reusable, responsive "Agent Proposal Cards" with minimum 44x44px touch targets for all interactive elements.
  *   Apply OHC Premium Design Tokens (typography, colors, Glassmorphism).
  *   Develop the API endpoints to serve feed items and handle approval actions.
  *   Ensure the feed accurately distinguishes between different types of agent proposals.
  *   Verify the implementation with thorough UI/Playwright tests asserting the presence and functionality of the feed and interactive buttons on a mobile viewport.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
