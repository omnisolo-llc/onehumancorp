issue_title: "[Architecture] Autonomous AI Business Advisor & Plain-Language Briefing Engine"
issue_description: |
  ## Problem Statement
  Small business owners suffer from "Financial Fog" and are overwhelmed by complex dashboards with raw metrics. They need actionable insights in human language, not charts. A baker or a handyman doesn't have time to interpret retention cohorts, LTV/CAC ratios, or inventory turnover velocity. They need to be told, "You had 8 orders this week. Vegan cake requests doubled. Consider adding a vegan chocolate option!"

  ## Research Report
  - Competitors (Shopify, Wix) provide traditional analytics dashboards that require interpretation.
  - OHC's "Business Advisory" department must translate data into simple English.
  - Mobile-first approach: A daily push notification leading to a single "Briefing" screen.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[Metrics Service] -->|Raw Data| B[AI Advisor Agent]
      B -->|Context & History| C[Memory Vector Store]
      B -->|Generates| D[Plain-Language Briefing]
      D -->|Push & UI| E[Mobile Client - 375px]
  ```
  ### Components
  - **Data Aggregator**: Pulls daily metrics (sales, interactions, inventory levels).
  - **Advisor LLM Agent**: Uses the system prompt for the "Business Advisory" department to interpret data and generate a concise, encouraging briefing.
  - **Mobile UI**: A clean, glassmorphism-styled daily briefing view. 3-4 bullet points max.

  ## Implementation Prompt
  Implement the backend aggregation service and the Business Advisor AI agent integration to generate a daily briefing. Create a mobile-first (375px) UI component that displays the briefing using the platform's standard token library (Glassmorphism, 20px blur).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
