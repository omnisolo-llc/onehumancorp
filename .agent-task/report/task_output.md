issue_title: "Implement the AI-Powered Local SEO Profile Agent"
issue_description: |
  # Issue Brief: AI-Powered Local SEO Profile Agent

  ## Problem Statement
  Small business owners (like Carlos the Field Service Owner) consistently rank "Not on Google" and "SEO Mystery" as a top pain point. They lack the time and technical knowledge to claim, optimize, and maintain their Google Business Profile and local directory listings. Legacy platforms expect them to learn SEO metadata, whereas they just want more local customers.

  ## Research Report
  - **Market Context**: According to the global SMB market report, 3% of top pain points explicitly mention SEO, while many others indirectly suffer from poor local visibility.
  - **Competitor Analysis**: Shopify relies on third-party apps for SEO. Wix offers limited AI meta tag generation. GoDaddy Airo creates basic brand identity but lacks proactive local profile management. No competitor offers an "invisible agent" that continuously monitors and updates local presence.
  - **The OHC Opportunity**: Building an "AI Local SEO Optimizer" fulfills Pillar 2 of our AI Differentiation Manifesto. It transforms a complex, manual task into a passive, agent-managed benefit.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      OHC_Core[OHC Core System] --> |Business Data| Local_SEO_Agent[Local SEO Agent]
      Local_SEO_Agent --> |Generates| Optimized_Content[Optimized Descriptions & Posts]
      Local_SEO_Agent --> |Publishes via API| Google_Business[Google Business Profile]
      Local_SEO_Agent --> |Provides Summaries| Owner_Feed[Owner Mobile Feed]
      Owner_Feed --> |Approves Action| Local_SEO_Agent
  ```
  ### Mobile UX Flow (375px First)
  1.  **Feed Notification**: The owner sees a card in their daily feed: "Your local profile needs an update. Want me to draft a new post about your latest service?"
  2.  **Review**: Tapping the card shows a clean, translucent glass-styled preview of the AI-generated Google Business update (photo + text).
  3.  **Action**: The owner taps a prominent "Approve & Publish" or "Edit" button.

  ### AI Integration Points
  - The agent periodically scans the tenant's product/service catalog and recent activity.
  - It uses the configured LLM provider to draft localized, keyword-optimized content.
  - It integrates with the KAIROS orchestration engine to present the draft to the owner before finalizing the API call to Google/directories.

  ## Implementation Prompt
  **Target Persona**: Carlos (Field Service Owner).
  **Objective**: Create the core background worker and UI feed card for the "Local SEO Agent".
  **Acceptance Criteria**:
  - Implement a background job (using PostgreSQL SKIP LOCKED) that identifies tenants needing a local profile update.
  - The job must generate a draft update using the AI service.
  - Create a Flutter mobile-first feed card component that displays the proposed update with "Approve" and "Edit" actions.
  - Ensure the feature is fully functional on a 375px screen and adheres to the Translucent Glass / UniFi design system.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
