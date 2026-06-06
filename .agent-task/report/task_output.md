issue_title: "Implement 'The Promoter' Agent: Autonomous Social Media Manager"
issue_description: |
  # Research Report: The Promoter Agent - Autonomous Social Media Manager

  ## Problem Statement
  Users like Priya (Boutique Owner) and Maya (The Baker) launch their OHC stores but struggle to gain traffic because they do not know what to post on social media, or they lack the time to draft engaging marketing content. The "App Tax" of Shopify and manual integrations create a barrier to entry.

  ## Research Findings
  Competitor platforms require users to install third-party plugins or manually write metadata, captions, and schedule posts. OHC aims to solve this with native "Agentic Workflows" where "The Promoter" agent listens for changes in inventory (e.g., new product added) and autonomously generates multi-platform marketing content.

  ## Architecture & Design Flow
  - **Data Ingestion**: System event listener for `ProductCreated` and `ProductUpdated` events in the OHC backend.
  - **Processing Layer**: Generative AI pipeline analyzes product descriptions to generate marketing copy.
  - **Draft Generation**: Agent generates variant captions optimized for different platforms (e.g., short/punchy for TikTok, visual/descriptive for Instagram).
  - **Mobile UX**: "The Promoter" agent surfaces a card in the user's Agent Feed suggesting "New product detected! Schedule a post to drive sales?" Users tap to preview the variants and hit "Schedule".

  ```mermaid
  sequenceDiagram
      participant OHC Backend
      participant Event Bus
      participant Promoter Agent
      participant AI Provider
      participant Mobile Client
      participant Social API

      OHC Backend->>Event Bus: Emit ProductCreated
      Event Bus->>Promoter Agent: Trigger workflow
      Promoter Agent->>AI Provider: Request social copy based on product details
      AI Provider-->>Promoter Agent: Return 3 copy variants
      Promoter Agent->>Mobile Client: Push notification & Feed card
      Mobile Client->>Promoter Agent: User approves "Schedule"
      Promoter Agent->>Social API: Dispatch post
  ```

  ## Implementation Prompt
  - Build an asynchronous worker that listens for product creation events.
  - Implement the generative AI pipeline to create multi-platform variant copy.
  - Implement scheduling logic so posts are pushed at optimal times.
  - Ensure the Mobile UX provides a 1-tap approval flow for the generated content.
  - Ensure all features operate optimally on a 375px viewport.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
