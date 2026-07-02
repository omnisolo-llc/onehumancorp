issue_title: "Implement 'The Promoter' Agent: Autonomous Marketing Content Generation"
issue_description: |
  # Research Report: The Promoter Agent - Autonomous Marketing Content Generation

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Owner) and Maya (Home Baker) often struggle with marketing. After launching an OHC store or adding new products, they lack the time, expertise, or inspiration to consistently create engaging social media content. Traditional platforms (like Shopify or Wix) provide tools to connect social accounts but rely entirely on the user to manually draft, format, and schedule posts. This leads to the "Now What?" syndrome: a beautiful storefront with zero traffic because the owner isn't actively marketing.

  ## 2. Research Report
  - **Market Context:** Competitors offer integrations (e.g., Shopify's App Store has numerous social media schedulers like Buffer or Hootsuite) but these are passive tools. AI-native tools (like Jasper or Copy.ai) exist for copywriting, but they require the user to context-switch, prompt the AI manually, copy-paste the result, and then use a third tool to schedule.
  - **The OHC Opportunity:** By leveraging our "Agent Feed" architecture, OHC can proactively generate marketing content the moment a new product is added or inventory changes. "The Promoter" agent turns a passive catalog update into an active marketing campaign, requiring only a single tap of approval from the owner.
  - **Competitive Differentiation:** OHC moves from an advisory role (chatbots) to an autonomous execution role, saving the owner hours of manual work and eliminating the need for third-party marketing apps.

  ## 3. Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[ProductCreated/Updated Event] --> B(Event Ingestion Pipeline)
      B --> C{The Promoter Agent Worker}
      C --> D[Gemini Vision API: Analyze Images]
      C --> E[Gemini Pro API: Analyze Text/Context]
      D --> F[Draft Generation Engine]
      E --> F
      F --> G[Generate 3 Platform-Specific Variants]
      G --> H[Action Card Push to Agent Feed]
      H --> I[User 1-Tap Approval via Mobile App]
      I --> J[Post Scheduled/Published via Social APIs]
  ```

  ### Mobile UX Flow (375px)
  1. **Trigger:** Priya adds a new "Summer Floral Dress" to her inventory.
  2. **Notification:** She receives a push notification: "The Promoter drafted a new post for 'Summer Floral Dress'. Tap to review."
  3. **Action Card:** Opening the app, Priya sees a beautiful card in her Agent Feed. It displays the product image and three caption variants (e.g., a short punchy one for TikTok, a descriptive one for Instagram, a promotional one for Facebook).
  4. **Action:** Priya selects her favorite variant (or edits it slightly) and taps the prominent "Approve & Schedule" button (≥44x44px touch target).
  5. **Result:** The system handles the scheduling and publishing automatically. The card transitions to a success state.

  ### AI Agent Integration Points
  - **Trigger:** System event listener hooked into the product catalog CRUD operations.
  - **Processing:** Asynchronous worker utilizing LLMs (Gemini/MiniMax) for multi-modal analysis (image + text) to generate high-converting copy.
  - **Output:** Structured data payload (image URL, caption variants, suggested posting times) delivered to the Agent Feed service.

  ## 4. Implementation Prompt
  **Feature Name:** The Promoter Agent - Autonomous Social Post Generation
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** An automated workflow where adding a new product triggers the AI to draft multi-platform social media posts, presenting them in the Agent Feed for 1-tap approval.

  **Next Actions for Engineering:**
  1.  **Event Listener:** Implement an asynchronous worker that listens for `ProductCreated` and `ProductUpdated` events within the central PostgreSQL database. Ensure multi-tenant isolation.
  2.  **Generative AI Pipeline:** Integrate with the configured LLM provider (using the existing `OHC_LLM_PROVIDER` abstraction) to analyze product details (and images, if supported/configured) and generate 3 distinct marketing caption variants.
  3.  **Agent Feed Integration:** Create a new Action Card type for the Agent Feed that surfaces these drafted posts to the user.
  4.  **Mobile UX (Tauri/Flutter):** Implement the UI for the Action Card, ensuring it fits perfectly on a 375px viewport with clear, large touch targets for selecting a variant and approving the post.
  5.  **Scheduling Logic:** Implement the backend logic to handle the "Approve & Schedule" action, queuing the post for delivery (mocked social API integration is acceptable for this initial phase, focusing on the core agent workflow).
  6.  **E2E Testing:** Write a Playwright test simulating a user adding a product, receiving the Promoter card, and approving a post.

  **Acceptance Criteria:**
  - The feature must be fully functional and visually pristine on a 375px mobile viewport.
  - Zero mock data in the UI; drafts must be generated by the real backend/LLM.
  - The user should only have to tap "Approve" to complete the core journey.
  - Unit and Playwright E2E tests must pass.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
