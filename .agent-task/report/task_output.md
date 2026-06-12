issue_title: "Implement Agentic Local SEO & Reputation Management"
issue_description: |
  # Research Report: Agentic Local SEO & Reputation Management

  ## 1. Problem Statement
  Small business owners (especially service businesses, local shops, and food vendors like Fatima the Food Cart Operator or Carlos the Field Service Owner) rely heavily on local discoverability. However, setting up and maintaining Google Business Profiles, managing local citations, and responding to reviews is time-consuming and technically daunting. Existing solutions require the user to actively monitor and manually update information across platforms, or they involve expensive third-party tools that don't integrate with their core operations. As a result, businesses lose local search visibility and miss out on potential customers.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Wix offer basic SEO tools (editing meta tags) but lack deep, automated integration with local search ecosystems (like Google My Business). While specialized reputation management tools exist (e.g., Yext, Podium), they are often too expensive and disconnected from the SMB's central operating system.
  - **The OHC Opportunity**: OHC can differentiate by making local SEO and reputation management completely autonomous. By leveraging the Marketing and Customer Success Agents, OHC can automatically sync business information, generate local content, and draft review responses without the user needing to learn SEO mechanics.
  - **Competitor Gaps**:
    - *Shopify*: Focuses on global e-commerce; local SEO requires apps.
    - *Wix*: Provides SEO wizards but still requires manual execution and monitoring.
    - *Yext/Podium*: Powerful but act as separate silos, adding to the "app tax" and operational fragmentation.

  ## 3. Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[OHC Business Profile (PostgreSQL)] --> B(Local SEO Sync Engine)
      B --> C[Google Business Profile API]
      B --> D[Other Local Directories]

      E[New Review Received (Webhook)] --> F(Reputation Management Pipeline)
      F --> G[Customer Success Agent (The Ambassador)]
      G -->|RAG against FAQs/Policies| H[Draft Review Response]
      H --> I[Action Required Queue]
      I --> J[Mobile App Feed (375px)]
      J -->|1-Tap Approve| K[Publish Response via API]

      L[New Product/Service Added] --> M[Marketing Agent (The Promoter)]
      M --> N[Draft Local Update/Post]
      N --> I
  ```

  ### Data Model (PostgreSQL)
  - `LocalProfile`: Stores the canonical business information (Name, Address, Phone, Hours, Categories) linked to a `tenant_id`.
  - `Review`: Stores customer reviews fetched from external platforms, including rating, text, and response status.
  - `ReviewResponseDraft`: Stores the AI-generated draft response pending owner approval.

  ### AI Agent Integration
  - **The Ambassador (Customer Success Agent)**: Monitors incoming reviews. Uses sentiment analysis and RAG against the business's data to draft personalized, professional responses to both positive and negative reviews.
  - **The Promoter (Marketing Agent)**: Monitors the product/service catalog and business hours. Automatically suggests "Google Posts" or updates when new offerings are added or hours change (e.g., holiday hours).

  ### Mobile UX Flow (375px)
  1. **Notification**: The owner receives a push notification: "New 5-star review on Google! Tap to reply."
  2. **Agent Feed**: The owner taps and opens the Agent Feed. A card shows the review text and the AI-drafted response (e.g., "Hi [Name], thank you so much for the kind words about our vegan cake! We hope to see you again soon.").
  3. **Action**: The card has a large "Approve & Publish" button and an "Edit" button.
  4. **Publishing**: Tapping "Approve" publishes the response directly to Google Business Profile via the API.

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Local SEO & Reputation Management
  **Target Persona**: Carlos the Field Service Owner
  **Outcome**: Carlos's business information is automatically kept consistent across local search. When a customer leaves a review, Carlos gets a pre-drafted response in his OHC app that he can approve with one tap, improving his local ranking without requiring him to be an SEO expert.

  **Next Actions**:
  1. Implement the `LocalProfile` and `Review` data models with strict multi-tenant isolation.
  2. Create a sync engine (e.g., a background worker) that can push `LocalProfile` updates to the Google Business Profile API.
  3. Develop the integration for the Ambassador agent to ingest new reviews via webhook/polling and draft responses.
  4. Build the Mobile UX component for the Agent Feed to display review response cards and handle the 1-tap approval flow.
  5. Add E2E Playwright tests to verify the review approval flow from the mobile feed.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []