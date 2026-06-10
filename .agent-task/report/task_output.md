issue_title: "Implement 'The Promoter' Agent for Automated Social Media Marketing Copy Generation"
issue_description: |
  # Research Report: The Promoter Agent - Automated Social Media Marketing Copy Generation

  ## 1. Problem Statement
  Small business owners (e.g., Priya the boutique operator, Maya the home baker) struggle with marketing. When they add a new product to their catalog, they often lack the time, expertise, or creativity to draft engaging social media posts to announce it across different platforms (Instagram, TikTok, Facebook). Traditional platforms (Shopify, Wix) do not offer native, proactive AI content generation out of the box, relying on third-party apps or manual effort. This results in newly added products not being effectively promoted, leading to lost sales and poor traffic.

  ## 2. Research Report
  - **Shopify:** Provides tools to manage products but relies heavily on apps (e.g., for email marketing, social posting) or basic AI text suggestions (Sidekick) that are reactive rather than proactive.
  - **Wix/Squarespace:** Offer basic social post creation tools, but they still require manual initiation and effort to craft platform-specific copy.
  - **The OHC Opportunity:** OHC's value proposition is "AI Does Useful Work." The system should proactively detect when a user adds a new product and autonomously draft marketing copy optimized for different platforms, presenting it to the owner for a 1-tap approval.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Product Added/Updated in OHC] -->|Event Trigger| B(Event Bus / Queue)
      B --> C[The Promoter Agent Worker]
      C -->|Query| D[Product Catalog DB]
      C -->|Analyze Image/Text| E[LLM: Gemini Vision / Pro]
      E -->|Draft Copy| F[Platform Variants: IG, TikTok, FB]
      F --> G[Agent Feed / Action Required]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Social Media API / Scheduler]
  ```

  ### Mobile UX Flow (375px First)
  1. **Event Trigger:** Priya adds a new "Summer Floral Dress" to her catalog.
  2. **Home Feed (Mobile):** The next time she opens OHC, the top card in her feed says "New Product Detected: Drafted 3 social posts for 'Summer Floral Dress'."
  3. **Interaction:** Tapping the card opens a unified view showing the product image and three variants of the drafted copy (e.g., short/punchy for TikTok, visual/descriptive for Instagram).
  4. **Action:** A prominent primary button "Schedule All" or "Approve & Post Now", and a secondary "Edit" button for each variant.
  5. **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **The Promoter (Marketing Agent):** Listens for `tenant.catalog.product.created` or `tenant.catalog.product.updated` events. It uses Gemini Vision (if images are present) and Gemini Pro to analyze the product details (name, description, price, category) and generate high-converting, platform-specific marketing copy.

  ### Key Design Decisions
  - **Proactive Generation:** The agent drafts the content automatically upon product creation. The user does not need to ask for it.
  - **Multi-Platform Variants:** The LLM prompt must explicitly ask for variations tailored to the distinct styles of different social platforms (e.g., hashtag-heavy for IG, hook-driven for TikTok).
  - **1-Tap Approval:** The core UX philosophy is to reduce cognitive load. The user reviews and approves, rather than creating from scratch.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** As a business owner, when I add a new product to my catalog, I want my assistant (The Promoter) to automatically draft engaging social media posts for it. When I open my app, I see the drafts ready for me to approve and schedule with one tap.
  **CUJ & Acceptance Criteria:**
  1. Implement an event listener/worker that triggers when a product is created or significantly updated.
  2. The worker invokes The Promoter Agent (LLM integration) to generate marketing copy based on the product's data.
  3. The Agent generates at least two distinct variants (e.g., for Instagram and Facebook/TikTok).
  4. The generated drafts are placed into the `agent_feed_items` table for the specific tenant, creating an "Action Card".
  5. Provide Playwright E2E tests: A user logs in, adds a new product, navigates to the Home/Agent Feed, sees the "Drafted Social Posts" action card, taps "Approve," and the system marks the action as complete.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
