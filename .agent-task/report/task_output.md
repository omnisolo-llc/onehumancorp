issue_title: "[Architecture] Real-Time AI Autonomous Paid Advertising & Budget Optimizer"
issue_description: |
  # Title: Real-Time AI Autonomous Paid Advertising & Budget Optimizer

  **Problem Statement:**
  Small business owners like Priya (boutique owner) and Leo (music tutor) recognize that paid ads (Meta, Google, TikTok) are essential for growth, but creating, testing, and optimizing ad campaigns requires marketing expertise they don't have. They are overwhelmed by Facebook Ads Manager or Google Ads. They simply want to say "Spend $100 this week to get me more students/sales" and have the system handle the rest. They need an AI agent that autonomously generates ad creatives, tests copy, allocates budget to the best-performing channels, and protects them from wasting money on ineffective campaigns.

  **Research Report:**
  - *Current Landscape*: Small business owners either waste money boosting posts blindly, or pay expensive agencies. Native platform tools are too complex.
  - *Competitor Gap*: Shopify offers basic ad integrations but requires users to manage the campaigns. Wix has some AI features but they aren't truly autonomous multi-channel optimizers.
  - *OHC Advantage*: An AI Marketing Department that connects directly to the user's catalog and inventory, automatically drafting ad creatives using existing product photos, writing copy, and distributing the budget optimally across Meta, Google, and TikTok APIs.

  **Design Doc:**
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      Catalog[Catalog & Inventory Ledger] --> AdAgent[AI Advertising Agent];
      Budget[OHC Treasury/Wallet] --> AdAgent;
      AdAgent --> MetaAPI[Meta Ads API];
      AdAgent --> GoogleAPI[Google Ads API];
      AdAgent --> TikTokAPI[TikTok Ads API];
      MetaAPI -.-> PerformanceEvents[Performance Webhooks];
      GoogleAPI -.-> PerformanceEvents;
      TikTokAPI -.-> PerformanceEvents;
      PerformanceEvents --> AdAgent;
      AdAgent --> Dashboard[Mobile Ad Dashboard];
    ```
  - **Mobile UX Flow (375px)**:
    - **Screen 1**: "Grow Your Business". User selects a goal: "Get more bookings" or "Sell more products".
    - **Screen 2**: Budget Slider: "$50/week" to "$500/week". A button saying "Launch AI Campaign".
    - **Screen 3**: Active Campaign Card showing spend vs. return in plain language (e.g., "Spent $20, generated 3 new bookings worth $150").
    - **Design Decisions**: All technical terms like "CPC", "ROAS", "Campaign objectives", and "Pixel tracking" are hidden. The AI agent translates business goals into API parameters for the ad networks.
  - **AI Agent Integration Points**: The AI Advertising Agent listens to the catalog for new high-margin products or open booking slots. It requests budget approval from the AI Finance Agent. It analyzes daily performance events to halt losing ads and boost winning ones.

  **Implementation Prompt:**
  Design the data model and background worker system for the `Autonomous Advertising Engine`. The system must:
  1. Define a `CampaignIntent` entity that captures the user's high-level goal and budget (e.g., "$100/week for new bookings").
  2. Implement an AI worker that generates ad creative combinations (images + copy) based on the `CampaignIntent` and the user's catalog.
  3. Create an integration interface to push these campaigns to Meta and Google Ads APIs.
  4. Build a feedback loop worker that ingests daily performance metrics and adjusts the budget allocation between channels, killing underperforming ads.
  5. Expose plain-language metrics to the mobile UI (spend, resulting sales/bookings).
  Do not implement the UI. Focus on the core orchestration logic, the AI creative generation step, and the multi-tenant campaign management. Ensure all sensitive API keys (Meta/Google) are managed securely.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
