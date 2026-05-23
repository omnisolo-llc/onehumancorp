issue_title: "[Architecture] Autonomous Social Ad Campaign Engine"
issue_description: |
  # Autonomous Social Ad Campaign Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) rely heavily on Instagram and local Facebook groups to find new customers. However, navigating Facebook Ads Manager or Instagram "Boost" features is overwhelming. They don't know what a "lookalike audience" is, how to set daily budgets to maximize ROI without overspending, or how to A/B test ad copy. They just want to spend $50 to "get more local orders this weekend." Currently, small business owners either waste money on poorly optimized boosted posts or pay expensive agencies. They need an invisible, zero-config ad manager that takes their catalog, auto-generates creatives, targets the right local demographics, and optimizes the budget in the background.

  ## Research Report
  **Market Gap & Competitor Analysis:**
  - **Shopify:** Integrates with Facebook/Instagram, but still requires the user to manage pixels, audiences, and ad campaigns within the Meta ecosystem or via complex third-party apps.
  - **Wix / Squarespace:** Offer basic social posting and some rudimentary ad creation, but lack autonomous, AI-driven budget optimization and multi-channel (TikTok + Meta) dynamic reallocation based on real-time ROAS (Return on Ad Spend).
  - **Agency Alternatives:** Services like Madgicx are powerful but built for digital marketers, not a 50-year-old food cart owner.

  **Opportunity:** By abstracting ad creation and management into a conversational interface ("I want to spend $50 this week to promote my new vegan cake"), OHC can capture the massive long-tail ad spend of micro-businesses. Our unique advantage is the Marketing Agent having direct access to inventory and sales data, allowing for hyper-contextual, high-converting ad generation.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ AD_CAMPAIGN : creates
      AD_CAMPAIGN ||--o{ AD_CREATIVE : uses
      AD_CAMPAIGN ||--o{ PERFORMANCE_METRIC : tracks
      TENANT ||--o{ CATALOG_ITEM : owns
      AD_CREATIVE }o--|| CATALOG_ITEM : promotes

      AD_CAMPAIGN {
          string status "draft, active, paused, completed"
          float total_budget
          string goal "awareness, conversions, messages"
      }

      AD_CREATIVE {
          string platform "meta, tiktok"
          string copy
          string image_url
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Merchant
      participant UI_Dashboard
      participant MarketingAgent
      participant AdEngine
      participant Meta_API

      Merchant->>UI_Dashboard: "Promote new product for $50"
      UI_Dashboard->>MarketingAgent: Intent: Promote, Budget: $50, Target: Local
      MarketingAgent->>AdEngine: Generate ad copy & select best product image
      AdEngine->>MarketingAgent: Proposed Creatives & Audience (Radius: 5 miles)
      MarketingAgent->>UI_Dashboard: Show preview for approval
      Merchant->>UI_Dashboard: Approves (1-tap)
      UI_Dashboard->>AdEngine: Launch Campaign
      AdEngine->>Meta_API: Create Campaign, Adset, Ad (via API)

      loop Daily Optimization
          Meta_API-->>AdEngine: ROAS & CTR Metrics
          AdEngine->>MarketingAgent: Analyze Performance
          MarketingAgent->>AdEngine: Adjust budget allocation
      end
  ```

  ### Business Journey Mapping
  - **Acquisition:** Users drawn to OHC because it promises "agency-level marketing" included for free.
  - **Onboarding:** Zero friction. The Marketing Agent automatically analyzes the catalog and pre-generates 3 ad ideas on day one.
  - **Activation:** The merchant runs their first $10 ad campaign with 1 tap.
  - **Retention:** Weekly plain-language SMS updates from the Marketing Agent showing real ROI ("You spent $10 and made $40 in new sales").
  - **Revenue:** OHC can offer tiered ad-budget financing (Capital) based on ROAS.
  - **Referral:** High ROI users become extreme evangelists in local business networks.

  ### UI Wireframes / Screen Flow Description (375px first)
  1. **Ad Intent View (Merchant):**
     - Clean, conversational card: "Want more customers this week?"
     - Simple sliders/inputs: Budget ($) and Goal ("Get Messages", "Sell Product").
     - Translucent Glass material background.
  2. **AI Preview View:**
     - AI generates 2-3 ad variations showing the selected product photo and snappy copy.
     - A single, prominent `[ Approve & Launch ]` button.
  3. **Active Campaign View:**
     - Simple progress bar showing budget spent vs. results (e.g., "Spent $15 - Got 3 new messages!").
     - "Stop Campaign" button prominently displayed.

  ### Mobile UX Flow
  - The entire flow must be completable in under 30 seconds on a mobile device.
  - No mention of "CPM", "Lookalike Audiences", or "Pixels". The language is plain: "We will show this to people within 10 miles of your shop who like baked goods."

  ### Performance & Offline Targets
  - **Latency:** Initial AI creative generation must return options in <3 seconds.
  - **Payload:** Image assets sent to social APIs must be heavily compressed at the edge.
  - **Offline:** The core Ad Engine and Meta_API integration must operate fully decoupled from the mobile app. The merchant's app acts purely as a command interface.

  ### Zero Trust & Security
  - Multi-tenant isolation is guaranteed by ensuring the Ad Engine uses short-lived, tenant-scoped OAuth tokens via SPIFFE/SPIRE when communicating with Meta/TikTok APIs.
  - Cross-tenant data bleed during AI generation is prevented by hard-partitioning the vector context windows.

  ### AI Agent Integration Points
  - **Marketing Agent:** Handles creative generation (copywriting), audience targeting parameters based on tenant metadata, and budget pacing.
  - **Operations Agent:** Ensures ads are paused automatically if the promoted inventory item goes out of stock.
  - **Finance Agent:** Tracks ad spend against the tenant's ledger and generates simple ROI reports.

  ### Key Design Decisions
  - **Zero-Config Meta/TikTok Integration:** The platform handles the underlying API complexities (OAuth, Business Manager setup) invisibly.
  - **Outcome-Based Inputs:** Users buy "results" (messages, sales) rather than "ads".
  - **Auto-Pause on Stockout:** Tight integration with inventory prevents merchants from paying for ads on sold-out items, a common pain point.

  ## Implementation Prompt
  **Task:** Build the Autonomous Social Ad Campaign Engine.
  **Context:** Small businesses need to run effective social media ads but lack the technical expertise to use Meta/TikTok Ads Manager.
  **Outcome:**
  - A simple, conversational UI for merchants to set a budget and goal.
  - The Marketing Agent automatically generates creatives and manages audience targeting via external social APIs.
  - Background jobs continuously monitor ROAS and adjust spend, pausing ads automatically if inventory runs out.

  **Acceptance Criteria:**
  1. A tenant can launch an ad campaign by only providing budget and goal.
  2. The Marketing Agent successfully generates ad creatives (copy + image selection) using the tenant's catalog.
  3. The Ad Engine integrates with a mock/sandbox Social API to create campaigns, adsets, and ads.
  4. The system automatically pauses campaigns if linked inventory drops to zero.
  5. All UI follows the mobile-first, grandmother-test standard with Translucent Glass materials.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
