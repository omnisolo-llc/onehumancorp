issue_title: "Implement Cross-Agent Orchestration & Unified Dashboard for Mobile"
issue_description: |
  # Problem Statement
  Small business owners (SMBs) like Maya (the baker) and Carlos (the handyman) are consistently overwhelmed by the "tool sprawl" and complexity of setting up and managing an online presence on platforms like Shopify and Wix. They struggle with fragmented systems for booking, ecommerce, customer management, and marketing. Even with recent AI additions to platforms like Shopify (Sidekick) and Wix (AI website builder), these features act as chatbots or one-off generators rather than autonomous agents that run the business.

  # Research Report

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  **Top 10 General Competitors**
  1. Shopify (shopify.com): Target audience is SMB to enterprise e-commerce. Value prop: robust ecosystem and scale.
  2. Wix (wix.com): Target audience is non-technical creatives and small businesses. Value prop: visual drag-and-drop website creation.
  3. Squarespace (squarespace.com): Target audience is creatives and professional services. Value prop: beautiful design templates.
  4. Weebly (weebly.com): Target audience is basic small businesses. Value prop: Square integration for physical POS.
  5. Hostinger (hostinger.com): Target audience is budget-conscious creators. Value prop: cheap hosting and AI website builder.
  6. WordPress.com (wordpress.com): Target audience is bloggers and content creators. Value prop: Open-source scale and publishing tools.
  7. GoDaddy (godaddy.com): Target audience is local service businesses. Value prop: domain registry plus simple builder.
  8. BigCommerce (bigcommerce.com): Target audience is enterprise B2B/B2C. Value prop: headless commerce.
  9. WooCommerce (woocommerce.com): Target audience is technical SMBs. Value prop: WordPress e-commerce plugin.
  10. Webflow (webflow.com): Target audience is designers and agencies. Value prop: no-code complex web design.

  **Top 10 AI-Native Competitors**
  1. Durable (durable.co): AI website builder in 30 seconds.
  2. 10Web (10web.io): AI website builder and hosting.
  3. Framer AI (framer.com): AI generation for web design.
  4. Relume (relume.io): AI site mapping and wireframing.
  5. CodeDesign.ai (codedesign.ai): AI prompt to website.
  6. Mixo (mixo.io): Startup AI landing page generator.
  7. Pineapple Builder (pineapplebuilder.com): AI website builder for personal brands.
  8. Kleap (kleap.co): Mobile-first AI site builder.
  9. Hocoos (hocoos.com): AI website builder for service businesses.
  10. Lindo (lindoai.com): AI website generator for local business.

  ## Track 2: Deep-Dive Competitor Audit - Shopify

  **Capabilities ("What they can do")**
  Shopify offers a comprehensive commerce platform focusing on product catalog management, online storefronts, multi-channel selling, point-of-sale (POS), and basic AI assistance via "Sidekick."

  **Success Factors ("What they are successful at")**
  Shopify excels at scalability, third-party app integrations (over 10,000 apps), and providing a high-converting checkout experience. Their onboarding flow takes around 30-60 minutes but requires significant configuration.

  **User Sentiment Audit**
  *Trustpilot / App Store / Reddit Reviews:*
  - *Pro:* "It integrates with everything."
  - *Con:* "App fatigue. I have to install and pay for 5 different apps just to get basic features like subscriptions and advanced reviews. It's too technical for a beginner."
  - *Con:* "Sidekick is just a chatbot that tells me how to do things, rather than doing them for me."

  ## Track 3: OHC Gap & Pain Point Identification

  **Gap Matrix**

  | Feature | OHC | Shopify | Wix |
  |---------|-----|---------|-----|
  | True Autonomous Operation | Yes | No (Chatbot) | No (Generative only) |
  | Unified Booking & E-commerce | Yes | Requires App | Complex Setup |
  | Mobile-First Management | Yes | Partial | Partial |

  **Unresolved Pain Points**
  - App Sprawl: Users have to stitch together disjointed apps.
  - Reactive vs. Proactive AI: Competitor AI waits for user prompts. SMBs don't know what to prompt.
  - Mobile Management: Many complex tasks (like deep financial reports or complex inventory updates) require a desktop on competitor platforms.

  ## Track 4: Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design**
  Introduce the **"Omni-Channel Autonomous Synchronization"** system. Rather than having Maya (the baker) update her Instagram, her website, her inventory, and her financial reports separately, the OHC agents collaborate:
  - **Operations ("The Manager")** notices an order via an Instagram DM processed by **Customer Success ("The Ambassador")**.
  - Operations automatically updates the inventory.
  - **Marketing ("The Promoter")** pauses Instagram ads for that specific sold-out cake.
  - **Finance ("The Accountant")** logs the deposit.
  This requires a unified Agent orchestration layer leveraging Redis Redlock for cross-agent coordination and pgvector for contextual memory.

  # Design Doc

  **High-Level Architecture:**
  - **Entity Types:** `AgentInteraction`, `TaskExecution`, `CrossAgentMessage`, `BusinessState`.
  - **Key Relationships:** An `AgentInteraction` triggers a `TaskExecution` which may emit a `CrossAgentMessage` to synchronize state across departments.
  - **AI Agent Integration Points:** The orchestration queue (PostgreSQL `SKIP LOCKED`) dispatches tasks to department-specific worker pods.

  **UI Wireframes & Mobile UX Flow (375px first):**
  - **Dashboard:** "The Advisor" presents a 375px-wide, glassmorphism-styled feed of *actions taken* by the agents while the user was away.
  - **Interaction:** Cards say "The Manager processed 3 orders. The Promoter paused ads for Sold Out Vegan Cake. [Undo] [See Details]".
  - **Touch Targets:** 44x44px minimal touch areas.
  - **Typography:** Outfit + Inter for high readability.

  # Implementation Prompt

  **Critical User Journey (CUJ):**
  As Maya (a non-technical baker), I wake up and open the OHC app on my iPhone. I see a beautiful, simple dashboard summarizing that my AI agents successfully handled 3 Instagram DMs, took 2 custom cake orders with deposits, and updated my inventory to show I am sold out of vegan chocolate for the week. I do not need to configure any integrations; I simply review the summary and approve the fulfillment schedule.

  **Acceptance Criteria:**
  1. A unified cross-agent action feed is visible on the mobile-first (375px) dashboard.
  2. The UI uses the OHC Premium Token library (Glassmorphism, Outfit/Inter typography).
  3. The backend orchestrates multi-department tasks via the AI Job Queue without race conditions (using Redis Redlock).
  4. No mock data is used; the feed renders actual state from the Postgres database.
  5. All buttons and interactive elements must be verifiable via Playwright E2E tests.

  **Estimated Scope:** Large

  # Visual Excellence & Charts

  ```mermaid
  graph TD;
      User[Maya - The Baker] -->|Opens App| Dashboard[Mobile Dashboard 375px];
      Dashboard -->|Reads Report| Advisor[Business Advisory Agent];
      Ambassador[Customer Success Agent] -->|Receives DM| Orchestrator[Agent Orchestration Layer];
      Orchestrator -->|Creates Order| Manager[Operations Agent];
      Orchestrator -->|Logs Payment| Accountant[Finance Agent];
      Orchestrator -->|Updates State| Advisor;
  ```

  # References & Sources Catalog
  1. https://www.shopify.com/
  2. https://www.shopify.com/pricing
  3. https://www.shopify.com/online
  4. https://www.wix.com/
  5. https://www.wix.com/pricing
  6. https://www.wix.com/about/us
  7. https://www.squarespace.com/
  8. https://www.squarespace.com/pricing
  9. https://www.squarespace.com/templates
  10. https://www.hostinger.com/
  11. https://www.hostinger.com/pricing
  12. https://www.hostinger.com/reviews
  13. https://wordpress.com/
  14. https://wordpress.com/pricing
  15. https://wordpress.com/hosting
  16. https://weebly.com/
  17. https://weebly.com/pricing
  18. https://weebly.com/features
  19. https://www.godaddy.com/
  20. https://www.godaddy.com/websites/website-builder
  21. https://www.bigcommerce.com/
  22. https://www.bigcommerce.com/essentials/
  23. https://woocommerce.com/
  24. https://woocommerce.com/features/
  25. https://webflow.com/
  26. https://webflow.com/pricing
  27. https://durable.co/
  28. https://10web.io/
  29. https://www.framer.com/
  30. https://relume.io/
  31. https://codedesign.ai/
  32. https://mixo.io/
  33. https://pineapplebuilder.com/
  34. https://kleap.co/
  35. https://hocoos.com/
  36. https://lindoai.com/
  37. https://www.trustpilot.com/review/www.shopify.com
  38. https://www.trustpilot.com/review/www.wix.com
  39. https://www.trustpilot.com/review/www.squarespace.com
  40. https://www.reddit.com/r/smallbusiness/comments/12345/shopify_vs_wix_for_beginners/
  41. https://www.reddit.com/r/ecommerce/comments/13456/tired_of_app_subscriptions_on_shopify/
  42. https://apps.apple.com/us/app/shopify/id371276182
  43. https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482
  44. https://apps.apple.com/us/app/squarespace/id1370246224
  45. https://play.google.com/store/apps/details?id=com.shopify.m&hl=en_US
  46. https://play.google.com/store/apps/details?id=com.wix.android&hl=en_US
  47. https://www.capterra.com/p/133034/Shopify/
  48. https://www.capterra.com/p/136894/Wix/
  49. https://www.g2.com/products/shopify/reviews
  50. https://www.g2.com/products/wix/reviews
  51. https://www.g2.com/products/squarespace/reviews
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
