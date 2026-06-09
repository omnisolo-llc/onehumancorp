issue_title: "Implement the Promoter Agent for Proactive Social Media Copy Generation"
issue_description: |
  # Research Report: The Promoter Agent Implementation

  ## Problem Statement
  Users launch an OHC store but struggle to gain traffic because they do not know what to post on social media, or they lack the time to draft engaging marketing content. Specifically, for personas like Priya (Boutique Owner), every new product addition should ideally be accompanied by a social media announcement, but this is often forgotten or skipped due to time constraints.

  ## Market Research & Context
  Based on our analysis of the SMB platform market, traditional platforms (like Shopify or Wix) rely on third-party apps for social media scheduling, which don't automatically draft content based on new product additions. OHC's unique value proposition is **Invisible AI Automation**, where agents proactively push drafted content to the user's feed for approval, rather than waiting for the user to initiate the process. The "Promoter Agent" embodies this by acting as a proactive social media manager.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[ProductCreated/Updated Event] -->|Event Bus| B(Promoter Agent Worker)
      B --> C{LLM Processing}
      C -->|Product Details| D[Generate Platform-Specific Captions]
      D -->|Drafted Content| E[Agent Feed Database]
      E --> F[Owner's Mobile Agent Feed]
      F -->|1-Tap Approve| G[Publish via Social Integrations]
  ```

  ### Mobile UX Flow
  1.  **Trigger:** A new product is added to the store (e.g., via the mobile app or web interface).
  2.  **Notification:** The "Promoter" agent surfaces an Action Card in the user's unified Agent Feed.
  3.  **Card UI (375px optimized):**
      -   **Header:** "New product detected! Schedule a post to drive sales?"
      -   **Content:** Preview of the generated captions (e.g., one for Instagram, one for TikTok).
      -   **Actions:** "Approve & Schedule", "Edit Drafts", "Dismiss".
  4.  **Approval:** Tapping "Approve & Schedule" confirms the intent, and the system queues the posts for optimal delivery times.

  ### AI Agent Integration
  -   The Promoter agent will listen to product lifecycle events.
  -   It will use the LLM provider (configured via `OHC_LLM_PROVIDER`) to generate engaging, platform-specific copy based on the product's name, description, and price.
  -   The generated draft is stored as a pending action in the Agent Feed for the specific tenant.

  ## Implementation Prompt
  -   **Trigger Mechanism:** Implement an asynchronous worker or listener that triggers when a new product is created.
  -   **Generative Pipeline:** Implement the logic to generate multi-platform variant copy using the designated LLM based on the new product's details.
  -   **Agent Feed Integration:** Create a new card type in the Agent Feed (both in the backend data model and the Next.js frontend if applicable, or Tauri if that's the primary target for this specific task) to display the drafted social media posts.
  -   **Approval Flow:** Implement the endpoint to handle the "Approve" action from the feed, which should (in this iteration) mark the draft as approved/scheduled.
  -   **Verification:** Ensure there are Playwright E2E tests covering the flow: Product creation -> Agent Feed card appearance -> Approval interaction.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
