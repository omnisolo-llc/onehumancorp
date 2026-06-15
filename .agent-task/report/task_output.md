issue_title: "Implement Invisible AI Cart Recovery Workflow"
issue_description: |
  # Research Report: Automated Cart Recovery via Invisible Agents

  ## Problem Statement
  Small business owners (SMBs) struggle with abandoned shopping carts, losing significant potential revenue. Existing platforms (like Shopify or Wix) either require complex manual setup of recovery emails or force the merchant to install and configure expensive third-party apps (e.g., Klaviyo). This "App Tax" and setup paralysis prevent non-technical owners from recovering sales.

  ## Research Report
  - **Shopify:** Relies on third-party apps for advanced recovery logic, or offers very basic, rigid templates natively. Requires manual configuration.
  - **Wix:** Basic recovery exists but lacks deep personalization or autonomous decision-making on incentives.
  - **OHC Opportunity:** Leverage the "Customer Success Agent" to proactively monitor session state and autonomously trigger personalized follow-ups. The agent should understand what was left in the cart, the customer's history, and the store's tone to draft a compelling re-engagement message, completely invisibly to the store owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Session] -->|Adds to Cart| B(Redis/Postgres Session State)
      B -->|Session Inactive 1hr| C[PostgreSQL Job Queue with SKIP LOCKED]
      C --> D[Customer Success Agent Worker]
      D -->|Query History| E[(Unified Customer Graph)]
      D -->|Draft Message| F[LLM Pipeline]
      F --> G{Channel Preference}
      G -->|Email| H[Transactional Email Service]
      G -->|SMS| I[SMS Gateway]
  ```

  ### Mobile UX Flow (375px First)
  - **Merchant View:** Zero configuration required. It works out of the box. The merchant only sees the results in their weekly summary feed: *"The Ambassador Agent recovered 4 carts this week, saving $150."*
  - **Optional Merchant Control:** A simple toggle in settings: "Allow Agent to offer up to 10% discount on abandoned carts."

  ### AI Agent Integration Points
  - **Customer Success Agent:** Listens for `CartAbandoned` events from the scheduling queue. Uses RAG against the store's product catalog to write specific, enticing copy (e.g., highlighting product benefits rather than just saying "you forgot this").

  ### Key Design Decisions
  - **Proactive & Invisible:** The default state is ON. The system should start recovering revenue without the merchant lifting a finger.
  - **Delayed Execution:** Requires a robust background job scheduling mechanism (utilizing the existing PostgreSQL SKIP LOCKED pattern) to process carts exactly X hours after abandonment.

  ## Implementation Prompt
  **User-Facing Outcome:** Customers who abandon their carts receive a highly personalized, AI-drafted email or SMS encouraging them to complete their purchase, resulting in passive revenue recovery for the merchant.
  **CUJ & Acceptance Criteria:**
  1. Implement a background worker queue capable of scheduling future tasks (e.g., process this cart in 1 hour).
  2. Implement session monitoring logic that queues a `CartAbandoned` event if a cart is not checked out within a specific timeframe.
  3. Extend the Customer Success Agent to consume these events, query the LLM to generate personalized recovery copy based on cart contents, and dispatch the message.
  4. Ensure the system tracks whether a recovered cart eventually converts to attribute revenue to the agent.
  5. Provide automated tests verifying that an abandoned session automatically triggers the agent pipeline.

  ## Priority
  P0

  ## Estimated Scope
  Medium-Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
