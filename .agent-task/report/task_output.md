issue_title: "Implement Agentic Search Analytics & SEO Health Reporting"
issue_description: |
  ## Problem Statement
  Non-technical small business owners (like Maya the Baker or Carlos the Handyman) struggle to understand their search performance and local SEO visibility. They do not know how to read Google Analytics, Search Console, or complex SEO tools, leaving them blind to how customers actually discover them.

  Our research ("[research]_universal_edge_cached_dynamic_storefront_seo.md", "ohc_smb_market_report.md") identified "SEO Mystery" as a top 10 SMB pain point. While the platform handles edge caching and basic SEO tags, the owner lacks actionable insight.

  ## Research Report
  - **The Gap**: Currently, OHC lacks a unified way to report search visibility, keyword performance, and local discovery (e.g. Google Maps views) back to the owner in plain language.
  - **Competitor Flaws**: Shopify and Wix offer complex dashboards or require external plugins like Google Analytics, which overwhelm the non-technical persona.
  - **OHC Differentiator**: Instead of showing a dashboard with bounce rates and impressions, OHC should use the **Business Advisory Agent** to proactively tell the owner: "Your search traffic is up 15% this week! People are finding you by searching for 'Vegan Cakes Austin'. You should add a new post about your vegan menu."

  ## Design Doc
  ### Architecture
  - **Data Ingestion**: A new `search_analytics` pipeline that aggregates organic search metrics (which can later integrate with GSC/Google Business API, but for now relies on internal pageview referrers and search queries).
  - **Database Schema**: A new `seo_metrics` table or expanded `analytics` view scoped by `tenant_id` to store daily aggregated search impressions, clicks, and top keywords.
  - **Agent Integration**: The `Business Advisory Agent` (via a scheduled cron job or the `agent_feed` pipeline) reads the aggregated SEO metrics weekly.
  - **Mobile UX Flow (375px)**:
    - The Agent Feed receives a new Action Card: "Weekly Search Insights".
    - The card displays a plain-language summary (e.g., "50 new people found your store on Google this week").
    - The card includes an actionable recommendation button (e.g., "Draft an SEO-friendly blog post").
    - Glassmorphism design tokens applied to the card.

  ### Implementation Prompt
  **Feature Name**: Agentic Search Analytics & SEO Health Reporting
  **Target Persona**: Maya the Baker
  **Outcome**: Maya receives a simple, plain-language weekly card in her Agent Feed telling her how many people found her business via search and suggesting an action to improve it, without ever opening a chart or graph.

  **Next Actions for Engineering**:
  1. Add a basic `search_analytics` tracker or schema to aggregate inbound search referrers and keywords.
  2. Create a weekly job that the Business Advisory Agent can consume to analyze the `search_analytics` data.
  3. Generate an `Agent Feed` Action Card containing the LLM-summarized insights and a suggested next action (e.g., draft a post).
  4. Build Playwright E2E tests verifying the card renders correctly on mobile (375px) and handles the "Approve" action.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
