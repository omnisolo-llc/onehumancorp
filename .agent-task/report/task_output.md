issue_title: "Implement Agentic Negotiator & Booker for Automated Lead Capture"
issue_description: |
  ## Problem Statement
  Service owners (e.g., Carlos) lose up to 30% of leads because they cannot instantly reply while on a job. They need a system that captures demand, quotes, and books autonomously.

  ## Research Report
  ### Competitive Landscape Audit
  - **Shopify Sidekick & Magic:** Strong in product descriptions, emails, and analytics, but weak in custom service quoting (e.g., handyman services).
  - **Durable AI:** 30-second website setup but lacks deep daily operation assistance for service businesses.
  - **11x.ai (Alice):** Excellent at autonomous phone and chat handling, demonstrating that owners value agents that *do the work* rather than just *suggest* it.

  **Competitors Assessed:** Shopify, Wix, Durable, 10Web, Mixo, Framer AI, HubSpot, Square, Intercom Fin, Lindy.ai, Relevance AI, Skyvern, 11x.ai, AGI, HoneyBook, Dubsado, Squarespace, GoDaddy, BigCommerce.

  **Unresolved Pain Points:**
  1. **The Setup Hurdle:** Small business owners abandon complex setups. Configuring Stripe, setting shipping zones, and adding initial products are major roadblocks.
  2. **Missed Opportunities:** Service providers lose leads when they are on the job and cannot answer the phone or reply to DMs instantly.

  ## Design Doc
  - **Entity Types:** `Lead`, `QuoteRequest`, `AgentInteractionLog`.
  - **Key Relationships:** `Lead` has many `AgentInteractionLog`. `QuoteRequest` is generated from `AgentInteractionLog`.
  - **Integration Points:** Meta Graph API (Instagram DMs), Twilio (SMS), OHC unified inbox, OHC booking service.
  - **UI Wireframes/Flow (Mobile 375px first):**
    1. Customer DMs via Instagram (integrated into OHC Inbox).
    2. Owner UI: The conversation is visible, but marked with a "Handled by Agent" status token (translucent glass styling).
    3. Agent dynamically quotes based on historical `Quote` data and proposes a time from the `Booking` service.
    4. Owner UI: A "Review & Approve Quote" translucent card appears in the Assistant-first feed, pushing actionable items to the top.

  ## Implementation Prompt
  Implement the backend agent logic to intercept unassigned inbound messages. The agent must analyze the intent (e.g., "Need a plumber ASAP"), query the booking availability service, generate a draft quote, and place it in the owner's daily review feed for 1-click approval. Ensure all agent actions are logged and visible in the unified timeline. The user-facing outcome is a single unified feed where Carlos can see the agent's draft quote and approve it with one tap.

  ## Estimated Scope
  Large

  ## Visual Excellence
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> Shopify[Shopify: Sidekick];
      Traditional --> Squarespace[Squarespace: Guided];
      Traditional --> HubSpot[HubSpot: Breeze];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy: Executive EA];
      AINative --> 11x[11x: Alice Sales];

      OHCGap((OHC Gap: Autonomous Onboarding & Proactive Ops));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap (OHC vs Competitors)
  | Feature | Shopify Sidekick | Durable AI | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | Days | < 1 Minute | Hours (Manual) | **< 10 Minutes (Agentic)** |
  | **Daily Ops** | Dashboard-first | Simple List | Service-first | **Assistant-first (Feed)** |
  | **Client Intake** | Manual Forms | Basic Leads | Widget-based | **Autonomous Negotiator** |
  | **Inventory** | Manual Sync | Manual | Database-backed | **Predictive Auto-restock** |

  ## References & Sources Catalog
  1. https://www.shopify.com/magic - Shopify Magic AI features.
  2. https://www.shopify.com/sidekick - Shopify Sidekick assistant details.
  3. https://www.wix.com/ai-website-builder - Wix AI website generation capabilities.
  4. https://durable.co/ - Durable's 30-second website builder and CRM.
  5. https://www.10web.io/ - 10Web AI website builder and hosting platform.
  6. https://mixo.io/ - Mixo AI startup launcher and landing page creator.
  7. https://www.framer.com/ai/ - Framer AI for generating entire website designs.
  8. https://www.hubspot.com/products/ai - HubSpot Breeze AI suite for CRM.
  9. https://squareup.com/us/en/software/ai - Square's AI-driven analytics and marketing.
  10. https://www.intercom.com/fin - Intercom Fin AI customer service bot.
  11. https://www.lindy.ai/ - Lindy AI personal assistant.
  12. https://relevanceai.com/ - Relevance AI platform for building agentic workforces.
  13. https://skyvern.com/ - Skyvern AI for automating browser workflows.
  14. https://www.11x.ai/ - 11x autonomous digital workers (Alice, Julian).
  15. https://www.agi.app/ - AGI app for on-device actions.
  16. https://www.honeybook.com/ai - HoneyBook AI features for independent businesses.
  17. https://www.dubsado.com/features/automation - Dubsado CRM automation capabilities.
  18. https://www.squarespace.com/design/ai-website-builder - Squarespace AI builder.
  19. https://www.godaddy.com/ai - GoDaddy Airo AI tools.
  20. https://www.bigcommerce.com/solutions/ai/ - BigCommerce AI and predictive analytics.
  21. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/ - SMB complaints on Shopify setup complexity.
  22. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/ - E-commerce discussions on AI builders.
  23. https://www.trustpilot.com/review/durable.co - User sentiment and reviews for Durable.
  24. https://www.trustpilot.com/review/10web.io - User sentiment and reviews for 10Web.
  25. https://www.g2.com/products/lindy-lindy/reviews - Lindy user reviews highlighting scheduling ease.
  26. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/ - Forbes analysis on e-commerce AI competition.
  27. https://techcrunch.com/2024/02/22/10web-armenia/ - TechCrunch coverage of 10Web funding and traction.
  28. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/ - SEJ report on 10Web APIs.
  29. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/ - LA Times on AGI and Snapdragon integration.
  30. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/ - Tom's Guide on the future of on-device AI assistants.
  31. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/ - Yahoo Finance on Qualcomm and agentic AI.
  32. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/ - Investing.com coverage of agentic AI announcements.
  33. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick - Shopify changelog on Sidekick capabilities.
  34. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/ - DeepLearning.ai on browser agent technical foundations.
  35. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html - NYT on AI integration in consumer tech.
  36. https://www.relevanceai.com/customers/canva - Relevance AI case study with Canva.
  37. https://www.relevanceai.com/customers/kpmg - Relevance AI case study with KPMG.
  38. https://www.11x.ai/customers - 11x customer success stories.
  39. https://www.11x.ai/blog/digital-workers-revenue - 11x blog on revenue impact of digital workers.
  40. https://fin.ai/cx-models - Fin AI customer experience models.
  41. https://www.intercom.com/blog/ai-agent-blueprint/ - Intercom blueprint for AI agent deployment.
  42. https://www.hubspot.com/spotlight - HubSpot product spotlight on AI features.
  43. https://www.hubspot.com/new - Recent HubSpot AI releases.
  44. https://www.wix.com/blog/how-does-ai-work - Wix technical blog on their AI implementation.
  45. https://www.wix.com/blog/best-ai-website-builder - Wix marketing on AI builder comparisons.
  46. https://durable.com/ai-website-builder - Durable's core AI builder product page.
  47. https://durable.com/blog/durable-vs-squarespace - Durable comparison against traditional builders.
  48. https://www.lindy.ai/integrations - Lindy's supported integrations ecosystem.
  49. https://www.lindy.ai/security - Lindy's security and privacy posture.
  50. https://skyvern.com/healthcare - Skyvern use cases in healthcare data entry.
  51. https://www.theagi.company/blog - The AGI Company blog on on-device agents.
  52. https://www.theagi.company/media-features - AGI media mentions and press.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
