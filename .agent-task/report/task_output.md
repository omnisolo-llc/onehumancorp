issue_title: "[Architecture] Autonomous Cross-Channel Ad Buying Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) rely heavily on new customer acquisition but are entirely overwhelmed by the complexity of Meta Ads Manager, Google Ads, and TikTok Ads. They don't know what a "CPA" is, how to set up targeting audiences, or how to allocate budgets across different platforms. Setting up tracking pixels, testing creatives, and monitoring ad spend is a full-time job. They need an invisible, AI-driven marketing agent that takes a simple goal (e.g., "Get me 5 more custom cake orders this week for $50") and autonomously generates, deploys, and optimizes ad campaigns across all major platforms directly from their mobile device.

  ## Research Report
  *   **Current Architecture Limits:** OHC currently lacks any native capability to purchase, deploy, or optimize paid advertising. Merchants would have to leave the platform, manually export assets, and navigate complex third-party ad interfaces.
  *   **Competitor Analysis:**
      *   *Shopify:* Integrates with Facebook/Google, but primarily for catalog sync. The user still has to configure targeting, budget, and campaign structures manually within the ad platforms or through rigid Shopify interfaces.
      *   *Wix:* Offers basic Facebook ad setup, but lacks autonomous multi-channel optimization and dynamic creative generation.
      *   *Dedicated Ad Tech (e.g., AdRoll, Madgicx):* Built for professional marketers, extremely complex, and completely inaccessible to a 375px mobile-first micro-merchant.
  *   **Discovery:** OHC needs an "Autonomous Cross-Channel Ad Buying Engine." This system should allow a merchant to specify a budget and a plain-language goal. The AI Marketing Department will then dynamically generate ad creatives (using the existing catalog), automatically bid across Meta, Google, and TikTok APIs, and reallocate spend in real-time based on conversion data, all completely invisible to the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MOBILE-APP ||--o{ AI-MARKETING-AGENT : "Sets Plain-Language Goal & Budget"
      AI-MARKETING-AGENT ||--o{ CREATIVE-GENERATOR : "Requests Ad Variants"
      CREATIVE-GENERATOR }|--|| ASSET-LIBRARY : "Pulls Product Images/Videos"
      AI-MARKETING-AGENT ||--o{ AD-ROUTING-ENGINE : "Allocates Budget"
      AD-ROUTING-ENGINE ||--o{ META-ADS-API : "Deploys & Monitors"
      AD-ROUTING-ENGINE ||--o{ GOOGLE-ADS-API : "Deploys & Monitors"
      AD-ROUTING-ENGINE ||--o{ TIKTOK-ADS-API : "Deploys & Monitors"
      AD-ROUTING-ENGINE }|--|| PERFORMANCE-CACHE : "Streams Live ROAS Data"
      PERFORMANCE-CACHE ||--o{ AI-MARKETING-AGENT : "Triggers Reallocation"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Goal Setting Screen (Mobile):** A clean card interface. "What do you want to achieve?" Options: "More Website Visits", "More Messages/DMs", "More Direct Sales". Maya selects "More Messages/DMs".
  *   **Budget Slider (Mobile):** A simple slider: "How much do you want to spend this week?" ($10 to $500). "We estimate this will bring 10-15 new inquiries."
  *   **Creative Approval (Mobile):** A Tinder-style swipe interface showing AI-generated ad previews. Maya swipes right to approve the creatives.
  *   **Active Campaigns Dashboard (Mobile):** A minimalist overview. "Spending $5/day. Generated 3 new DMs today." No CPM/CTR jargon unless an "Advanced Settings" switch is toggled.

  ### AI Integration Points
  *   **AI Marketing Agent:** Acts as the central brain. It takes the plain-language goal and translates it into specific API payloads for Meta/Google/TikTok.
  *   **Creative Generation Agent:** Automatically crops product photos, writes persuasive ad copy, and generates video slideshows optimized for each platform's formats.
  *   **Optimization Agent:** A background worker running continuously. It monitors ROAS (Return on Ad Spend) across channels. If TikTok is underperforming and Meta is overperforming, it autonomously shifts the budget without bothering the user.

  ### Key Design Decisions
  *   **Abstracted Metrics:** All complex marketing metrics (CPC, CPA, ROAS) are hidden by default. The primary metric shown to the user is the direct business outcome (e.g., "Orders Generated" or "Messages Received").
  *   **Unified Billing:** The merchant is billed a single amount by OHC. OHC handles the multi-currency payouts to the respective ad networks invisibly on the backend.
  *   **Zero-Config Tracking:** Tracking pixels (Meta Pixel, Google Tag) are automatically injected into the OHC storefront and synced with the ad accounts via Server-Side API, eliminating the need for manual setup.

  ### Technical Integrity & Mobile-First Targets
  *   **Performance & Offline Targets:** Ad metrics (spend, conversions) are cached locally so the "Active Campaigns Dashboard" loads in <100ms and remains visible even offline. Background sync batches API requests to external networks to minimize latency. Creative rendering for approvals uses heavily compressed WEBP placeholders, downloading full assets only if the user expands.
  *   **Zero Trust & Security:** Multi-tenant isolation is strict: Maya cannot access Carlos's ad budgets or data. The connection between the `AD-ROUTING-ENGINE` and external APIs uses SPIFFE/SPIRE for mutual TLS (mTLS) authentication to ensure secure, cryptographically signed requests when managing external budgets. Tokens are never exposed to the frontend.

  ## Implementation Prompt
  Implement the Autonomous Cross-Channel Ad Buying Engine. The backend must orchestrate calls to Meta, Google, and TikTok Ad APIs, translating a unified "Campaign Intent" object into platform-specific ad sets and budgets. Build the background job queue that continuously ingests performance data and reallocates budget dynamically. For the frontend, create the 375px mobile flow allowing a user to launch a campaign with just a goal and a budget slider. Ensure all technical ad jargon is completely hidden from the primary UI. Provide tests validating correct multi-tenant budget routing.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
