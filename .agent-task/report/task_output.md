issue_title: "OHC Architectural Gap: Mobile-First Plain-Language Analytics & Insight Briefings"
issue_description: |
  # Research Report: Mobile-First Plain-Language Analytics & Insight Briefings

  ## 1. Problem Statement
  Small business owners like Priya (boutique owner) are currently underserved by traditional analytics platforms (like Google Analytics or Shopify Dashboards) which present overwhelming charts, metrics, and complex filter settings. Priya needs to understand how her business is doing—sales trends, inventory velocity, and marketing ROI—while running her physical store, strictly from her mobile phone. She requires an autonomous system that proactively tells her, in plain language, "Your summer dresses are selling 30% faster than last week, consider ordering more," rather than making her interpret a funnel drop-off chart. OHC lacks a mobile-first, AI-driven analytics engine that delivers zero-friction, actionable insights directly to the user.

  ## 2. Research & Competitive Analysis
  - **Shopify/Wix:** Rely on heavy, desktop-first UI dashboards displaying raw metrics. The user is required to dig for insights.
  - **Google Analytics:** Overwhelmingly complex. Requires extensive setup and training, making it completely unsuited for non-technical SMB owners.
  - **OHC Opportunity:** We must leverage our AI Swarm (The Translator/Analyst Agent) to abstract raw data into actionable business intelligence invisibly and deliver it as a daily plain-text briefing notification.

  ## 3. Design Doc & Architecture
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as OHC Mobile App
      participant Engine as Analytics Engine
      participant DB as Data Lake / Core Postgres
      participant AI as Insight Agent (Translator)

      Engine->>DB: Aggregate daily events (sales, traffic, bookings)
      DB-->>Engine: Raw metrics (revenue, top items)
      Engine->>AI: Analyze metrics & generate plain-text insights
      AI-->>Engine: Actionable recommendations (e.g. "Reorder blue dress")
      Engine->>App: Push Daily Briefing (JSON formatted for UI)
      App-->>User: Display Translucent Glass Insight Card
  ```

  ### Mobile-First UI (375px First)
  - **The Morning Briefing:** A simple, elegant, macOS-style Translucent Glass card displaying 3-4 bullet points in plain language (e.g., "You made $1,250 yesterday. Your new organic cotton line is driving most of the growth.").
  - **Actionable Insight:** The briefing includes primary action buttons (e.g., "Draft Reorder Email" or "Apply Discount").
  - **Deep Dive:** Only if requested does the user see simplified sparkline charts.

  ## 4. Implementation Prompt
  **To the Implementer Swarm:**
  Implement the backend aggregation logic and API to deliver the Plain-Language Daily Business Briefing.
  1. Create a background service/scheduler that aggregates daily core metrics from the `orders`, `order_line_items`, and `bookings` tables for a given `tenant_id`.
  2. Implement an integration with the AI LLM Provider (`Translation Agent`) to transform the raw metrics into a concise, human-readable 2-3 sentence summary with at least one actionable recommendation.
  3. Expose a secure, tenant-isolated REST/gRPC endpoint that the mobile client can poll (or be pushed to) to retrieve the daily `InsightBriefing` payload.
  4. Ensure strict multi-tenant isolation in all aggregation queries.
  5. Include full unit and E2E test coverage asserting the AI generation and data isolation.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
