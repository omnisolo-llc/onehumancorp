issue_title: "[Architecture] Global Multi-Lingual Hybrid AI Translation Mesh"
issue_description: |
  # Global Multi-Lingual Hybrid AI Translation Mesh

  ## Problem Statement
  Small business owners like Fatima (food cart, limited English) and global creators like Leo need to serve diverse populations without manually translating everything. Current platforms like Shopify force owners to use expensive plugins (like Langify) or manually duplicate stores. An owner should never have to manually write translations for every product description, review, and chat message. They need an invisible mesh that translates intent, not just words, seamlessly adapting UI, content, and messaging based on the customer's and owner's locale.

  ## Research Report
  - **Competitor Analysis:**
    - Shopify: Relies heavily on 3rd-party apps (e.g., Translate&Adapt, Langify). Adds complexity and cost.
    - Wix: Has native translation but it's largely manual or a one-time AI pass. It doesn't dynamically translate ongoing chat or dynamically generated product variants well.
  - **OHC Opportunity:** Implement an edge-based, hybrid translation mesh. It uses fast, rule-based localization for UI, and an asynchronous LLM-based agent for deep context translation (e.g., translating a colloquial Instagram DM from a Spanish-speaking customer into English for Maya, and her English reply back into colloquial Spanish).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Device - Locale: ES] -->|Request Storefront| B[Edge CDN]
      B --> C{Translation Cached?}
      C -- Yes --> D[Serve Cached ES Page]
      C -- No --> E[API Gateway]
      E --> F[Hybrid Translation Engine]
      F --> G[Fast: Dictionary/UI Strings]
      F --> H[Slow: LLM Agent Contextual Translation]
      H --> I[(Vector/Translation Cache)]
      I --> B

      J[Customer DM - Spanish] --> K[Omnichannel Inbox]
      K --> L[Translation Agent]
      L -->|Translates to EN| M[Owner Dashboard - Locale: EN]
      M -->|Replies in EN| L
      L -->|Translates to ES| J
  ```
  ### Mobile UX Flow (375px)
  1. **Owner Setup:** A simple toggle in "Settings > Global Audience": "Auto-Translate my store and messages." No mapping fields or dictionaries.
  2. **Customer View:** The storefront auto-detects browser locale. A subtle floating button allows manual override.
  3. **Inbox View:** An incoming message shows in the owner's native language with a small "Translated from [Language]" badge. Tapping it shows the original text.

  ### AI Agent Integration
  - **The Ambassador (Customer Success):** Detects incoming message language, translates to owner's locale, drafts reply in owner's locale, and translates final approved message back to customer's locale.
  - **The Promoter (Marketing):** Translates new product descriptions and social media posts into enabled target languages asynchronously and caches them at the edge.

  ## Implementation Prompt
  Implement the Hybrid AI Translation Mesh.
  1. Create a `TranslationAgent` that hooks into the Event Mesh.
  2. For the Omnichannel Inbox: When an incoming message event is detected, if the language differs from the tenant's primary locale, the agent translates it before presenting it in the UI. When the owner replies, translate it back.
  3. For the Storefront: Implement an asynchronous translation worker that listens for `ProductCreated` or `ProductUpdated` events and pre-translates critical text fields (title, description) into configured secondary languages, storing them in a translation cache table for instant edge retrieval.
  4. Ensure strict multi-tenant isolation.
  5. Do NOT hardcode LLM providers; use the existing abstraction. Add Playwright E2E tests for the Inbox translation flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
