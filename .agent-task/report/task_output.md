issue_title: "[SEO] Autonomous SEO & Local Discovery Agent"
issue_description: |
  # Autonomous SEO & Local Discovery Agent

  ## Mission Queue Protocol
  Implement the "AI Discovery Agent" feature to optimize the business storefront for Generative Engine Optimization (GEO). The system should automatically generate and inject LLM-friendly structured data and natural language summaries into the user's public storefront without requiring any user input or SEO knowledge. Additionally, create a monthly reporting mechanism that translates discovery metrics into plain-language insights for the mobile dashboard.

  ## Problem Statement
  Small business owners (like Carlos the handyman) suffer from "Invisible Discovery" (52% frequency). They build a website but nobody comes. Traditional SEO is seen as a confusing "black art" filled with technical jargon. Users need to be easily found by potential customers using AI search engines (like ChatGPT, Perplexity, Gemini).

  ## Research Report
  - **Market Shift**: Search is transitioning from traditional keyword-based links to Generative Engine Optimization (GEO) where LLMs provide direct answers.
  - **Competitor Gaps**: Shopify, Wix, and Squarespace focus on legacy SEO (meta tags, sitemaps, keywords) and require the user to understand SEO concepts.
  - **AI Differentiation**: The 'AI Discovery Agent' works silently in the background, automatically structuring business data for LLM crawlers. No SEO knowledge is required from the user.

  ## Design Doc
  ### High-Level Architecture
  - **Trigger**: New website launch or content update (e.g., adding a new service area).
  - **Agent Action**: The AI Discovery Agent analyzes the business profile and automatically generates/updates structured JSON-LD data and natural language summaries optimized for LLMs.
  - **Integration**: The generated structured data is automatically injected into the live storefront's `<head>`.

  ### Mobile UX Flow (375px)
  1. **Zero Setup**: The user does nothing. It happens automatically.
  2. **Notification/Report**: Once a month, the user receives a simple notification: "Your AI Discovery Report is ready!"
  3. **Report Screen**: A plain-language summary showing how often they were recommended by AI search engines, e.g., "ChatGPT recommended your handyman services 15 times this week to locals in your area."

  ## Implementation Prompt
  Implement the "AI Discovery Agent" feature to optimize the business storefront for Generative Engine Optimization (GEO). The system should automatically generate and inject LLM-friendly structured data and natural language summaries into the user's public storefront without requiring any user input or SEO knowledge. Additionally, create a monthly reporting mechanism that translates discovery metrics into plain-language insights for the mobile dashboard. Do not prescribe specific database schemas, API contracts, or function signatures.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
