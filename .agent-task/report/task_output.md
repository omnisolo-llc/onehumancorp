issue_title: "Implement 'The Promoter' Agent for OHC"
issue_description: |
  ## Title: Implement 'The Promoter' Agent for Local SEO and Content Generation

  ## Problem Statement
  Small business owners and operators (like Maya the baker or Carlos the handyman) lack the time, expertise, and resources to consistently create high-quality marketing assets and manage their online presence. They struggle with "SEO Mystery" and "Marketing Asset Creation." They need to drive online demand but cannot afford to hire photographers or spend hours writing social media posts and optimizing Google Business profiles.

  ## Research Report
  Based on the global SMB market research report (`ohc_smb_market_report.md`) and our analysis of the "Shopify Tax" (`ai_agentic_workflows_research.md`), traditional platforms fail SMBs by expecting them to act as part-time marketers. Competitors like Shopify and Wix rely on complex plugins and manual setup for SEO and marketing, which alienates non-technical owners.

  Our deep dive into Tencent (a key inspiration for OHC) and its ecosystem reveals that successful platforms deeply integrate utility and entertainment without requiring user configuration.

  **Track 3 Findings (Gap Matrix):**
  - **Shopify/Wix:** Require manual entry of meta titles, descriptions, alt tags, and separate management of Google Business Profiles.
  - **OHC Current State:** Missing a unified, autonomous agent for localized SEO and marketing asset generation.
  - **Pain Points Addressed:** Initial Setup Paralysis (28%), Marketing Asset Creation (4%), SEO Mystery (3%).

  **Evidence from Research:**
  - Users explicitly complain: "Can't afford a photographer" and "Not on Google" (from Track 2 data).
  - The `ai_agentic_workflows_research.md` document outlines the vision for a `Marketing Agent` that "continuously monitors the product catalog... automatically generates optimized Alt tags, updates site metadata, and proposes a social media post/Google Business update, requiring only a single click ('Approve') from the owner."

  ### Persona Pain Point Mapping

  | Persona | Problem | Current Solution (Competitor) | OHC Agentic Solution |
  | :--- | :--- | :--- | :--- |
  | **Maya (Baker)** | No time for Instagram, doesn't know SEO. | Pays $30/mo for a Shopify marketing plugin, still has to write copy. | "The Promoter" auto-drafts Instagram posts from product photos. |
  | **Carlos (Handyman)** | Needs local Google reviews but forgets to ask. | Has to remember to text clients a Google link after a job. | "The Promoter" auto-sends review requests upon invoice payment. |
  | **Priya (Boutique)** | Needs fresh online content to drive foot traffic. | Spends 5 hours a week formatting newsletters in Wix. | "The Promoter" generates a weekly digest of new arrivals. |

  ### Competitive Feature Comparison

  | Feature | Shopify + Apps | Wix | Square | **OHC (Proposed)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Social Media Drafts** | App Required (e.g., Buffer) | Manual Entry | Basic Integration | **Autonomous (The Promoter)** |
  | **Local SEO Alt-Tags** | Manual / App Required | Manual Entry | Manual Entry | **Autonomous (The Promoter)** |
  | **Review Generation** | App Required (e.g., Yotpo) | Manual | Automated (Paid Tier) | **Autonomous (The Promoter)** |
  | **Setup Complexity** | High (Plugin hell) | Medium | Low | **Zero (Invisible Agent)** |

  ### Mermaid.js Diagrams

  #### OHC Agentic Workflow vs. Traditional Flow

  ```mermaid
  graph TD
      subgraph Traditional Platform
          A[User Adds Product] --> B(Manual: Write Meta Title)
          B --> C(Manual: Write Meta Desc)
          C --> D(Manual: Add Alt Tags)
          D --> E(Manual: Draft Social Post)
          E --> F[Published]
      end

      subgraph OHC Agentic Platform
          X[User Adds Product] --> Y(The Promoter: Auto-Generates SEO & Social Drafts)
          Y --> Z[User Approves]
          Z --> W[Published Everywhere]
      end
  ```

  #### The Promoter Agent Architecture

  ```mermaid
  flowchart LR
      A[Product Catalog] -->|Event: New Product| B(The Promoter Agent)
      C[Tenant Context/Memory] --> B
      B -->|Generates via LLM| D[SocialPostProposal]
      B -->|Generates via LLM| E[SeoMetadata]
      D --> F((Work Triage Feed))
      E --> F
      F -->|User Clicks Approve| G[Publish to Channels]
      G --> H(Instagram/FB)
      G --> I(Google Business)
      G --> J(OHC Storefront)
  ```

  ## Design Doc

  **High-Level Architecture:**
  - **New Agent:** `The Promoter` (Marketing Agent).
  - **Entities:** `MarketingCampaign`, `SocialPostProposal`, `SeoMetadata`.
  - **Integration Points:** Product Catalog (to monitor for new/updated items), OHC Feed (to surface proposals to the user), external APIs (Google Business Profile, Instagram/Facebook - mocked for now).

  **UI Wireframes/Screen Flow (Mobile-First 375px):**
  1.  **Work Triage Feed:** The owner sees a new card: "New Product Detected: Custom Birthday Cake. The Promoter has generated a social post and SEO tags."
  2.  **Proposal Detail Screen:**
      -   **Visual:** The generated image (or enhanced user photo).
      -   **Copy:** Proposed caption with hashtags.
      -   **SEO (Hidden by default, expandable):** Alt text, meta description.
      -   **Actions:** Large, accessible buttons for "Approve & Publish", "Edit", "Discard".
  3.  **Approval Flow:** Clicking "Approve" triggers the agent to execute the updates across configured channels.

  **AI Agent Integration:**
  -   The agent uses the configured LLM (Gemini Pro/MiniMax) to generate context-aware captions and SEO metadata based on the product description and tenant context.
  -   (Future) Integration with an image generation/enhancement API.

  ## Implementation Prompt
  Implement the foundation for "The Promoter" agent.
  1.  **Backend:** Create the necessary database tables (using row-level security with `tenant_id`) and gRPC services to support `SocialPostProposal` generation and storage.
  2.  **Agent Logic:** Implement a background job that monitors for new products and triggers the LLM to generate a proposed social media post and SEO metadata.
  3.  **Frontend:** Update the main "Work Triage" feed (in the Flutter/Tauri app) to display these proposals as actionable cards. Create the detail view where the user can approve the proposal.
  4.  **Critical User Journey (CUJ):** Maya adds a new "Vegan Chocolate Cake" to her catalog. Returning to the home screen, she immediately sees a proposal from The Promoter for an Instagram post about the new cake. She clicks "Approve", and the system marks the proposal as approved (simulating publishing).
  5.  **Acceptance Criteria:** The CUJ must pass a Playwright E2E test starting from login to approving the generated proposal. Zero mock data in the UI; use real backend state.

  ## Priority: P1
  ## Estimated Scope: Medium

  ## References & Sources
  1. [Tencent Holdings Overview](https://en.wikipedia.org/wiki/Tencent)
  2. [Shopify E-commerce Platform](https://en.wikipedia.org/wiki/Shopify)
  3. [Wix Website Builder](https://en.wikipedia.org/wiki/Wix.com)
  4. [Squarespace Website Builder](https://en.wikipedia.org/wiki/Squarespace)
  5. [Square Financial Services](https://en.wikipedia.org/wiki/Square,_Inc.)
  6. [HubSpot CRM and Marketing](https://en.wikipedia.org/wiki/HubSpot)
  7. [WeChat Super App](https://en.wikipedia.org/wiki/WeChat)
  8. [DingTalk Enterprise Communication](https://en.wikipedia.org/wiki/DingTalk)
  9. [Lark Enterprise Collaboration Suite](https://en.wikipedia.org/wiki/Lark_(software))
  10. [Notion Productivity Software](https://en.wikipedia.org/wiki/Notion_(productivity_software))
  11. [Microsoft Copilot AI Assistant](https://en.wikipedia.org/wiki/Microsoft_Copilot)
  12. [Reddit: Small Business Community Discussions](https://www.reddit.com/r/smallbusiness/)
  13. [Reddit: E-commerce Community Discussions](https://www.reddit.com/r/ecommerce/)
  14. [Trustpilot: Shopify Customer Reviews](https://www.trustpilot.com/review/www.shopify.com)
  15. [Trustpilot: Wix Customer Reviews](https://www.trustpilot.com/review/www.wix.com)
  16. [Trustpilot: Squarespace Customer Reviews](https://www.trustpilot.com/review/www.squarespace.com)
  17. [Apple App Store: Shopify Mobile App](https://apps.apple.com/us/app/shopify/id371295624)
  18. [Apple App Store: Wix Mobile App](https://apps.apple.com/us/app/wix/id1099748482)
  19. [Apple App Store: Squarespace Mobile App](https://apps.apple.com/us/app/squarespace/id1358053649)
  20. [Apple App Store: WeChat App](https://apps.apple.com/us/app/wechat/id414478124)
  21. [Apple App Store: DingTalk App](https://apps.apple.com/us/app/dingtalk/id930368978)
  22. [Apple App Store: Lark Collaboration App](https://apps.apple.com/us/app/lark/id1452261642)
  23. [Google Play Store: Shopify App for Android](https://play.google.com/store/apps/details?id=com.shopify.mobile)
  24. [Google Play Store: Wix App for Android](https://play.google.com/store/apps/details?id=com.wix.android)
  25. [Google Play Store: Squarespace App for Android](https://play.google.com/store/apps/details?id=com.squarespace.android)
  26. [Google Play Store: WeChat App for Android](https://play.google.com/store/apps/details?id=com.tencent.mm)
  27. [Google Play Store: DingTalk App for Android](https://play.google.com/store/apps/details?id=com.alibaba.android.rimet)
  28. [Google Play Store: Lark App for Android](https://play.google.com/store/apps/details?id=com.electronicarts.lark)
  29. [Shopify Sidekick AI Assistant Feature](https://www.shopify.com/sidekick)
  30. [Microsoft 365 Copilot Overview](https://www.microsoft.com/en-us/microsoft-365/copilot)
  31. [Notion AI Capabilities](https://www.notion.so/product/ai)
  32. [HubSpot Artificial Intelligence Tools](https://www.hubspot.com/products/artificial-intelligence)
  33. [Square AI Solutions for Business](https://squareup.com/us/en/ai)
  34. [Wix AI Features and Philosophy](https://www.wix.com/about/ai)
  35. [Squarespace AI Website Generation](https://www.squarespace.com/ai)
  36. [HoneyBook Client Management Software](https://www.honeybook.com/)
  37. [Dubsado Business Management Solution](https://www.dubsado.com/)
  38. [Jobber Field Service Management](https://www.jobber.com/)
  39. [Housecall Pro Service Business App](https://www.housecallpro.com/)
  40. [ServiceTitan Software for Trades](https://www.servicetitan.com/)
  41. [Mindbody Fitness and Wellness Software](https://www.mindbodyonline.com/)
  42. [Zen Planner Gym Management System](https://www.zenplanner.com/)
  43. [Vagaro Salon and Spa Software](https://www.vagaro.com/)
  44. [GlossGenius Salon Booking Platform](https://www.glossgenius.com/)
  45. [Fresha Free Salon Software](https://www.fresha.com/)
  46. [Booksy Appointment Booking System](https://www.booksy.com/)
  47. [GoHighLevel All-in-One Marketing Platform](https://www.gohighlevel.com/)
  48. [Thryv Small Business Software](https://www.thryv.com/)
  49. [Podium Local Business Messaging](https://www.podium.com/)
  50. [Broadly Reputation Management Tool](https://www.broadly.com/)
  51. [Birdeye Review and Reputation Platform](https://www.birdeye.com/)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
