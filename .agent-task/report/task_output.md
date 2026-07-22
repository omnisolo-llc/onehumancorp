issue_title: "Implement Autonomous Zero-Click AI Onboarding Flow"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Title
  Implement Autonomous Zero-Click AI Onboarding Flow

  ## 2. Problem Statement
  Non-technical owners (e.g., Maya, the home baker) are overwhelmed by the complexity of setting up an online business. They want to sell their services or products immediately but get stuck configuring settings, creating initial product listings, and linking payment processors. Shopify and Wix require them to act as IT admins rather than business owners.

  ## 3. Research Report
  - **Market Mapping & Competitor Discovery (Top 10 General):**
    - Shopify: Sidekick is powerful but only acts on existing stores; setup is highly manual.
    - Wix & Squarespace: Generative AI for design, but POS/Inventory configuration remains complex.
    - Square, HubSpot, WooCommerce, BigCommerce, GoDaddy, Weebly, PrestaShop.
  - **Top 10 AI-Native Rivals:**
    - Durable.co (30-second site generation), 10Web, Mixo, Framer AI, Lindy.ai, Relevance AI, Skyvern, 11x.ai, Intercom Fin, AGI.
  - **Deep-Dive Competitor Audit (Shopify & Durable):**
    - *Shopify Sidekick*: Edits themes, drafts emails. *User Sentiment*: “Setup is still a nightmare.”
    - *Durable.co*: 30-second AI generation for simple service sites. *User Sentiment*: “Fastest way to get a site up, but I can't customize it enough.”
  - **OHC Gap & Pain Point Identification:** OHC has a robust KAIROS orchestration engine but lacks the "Zero-to-One" autonomous experience found in Durable.
  - **Deeper Focused Research:** 34% of small business owners abandon setup due to "technical complexity". "I just want a link to send my Instagram followers so they can pay me, I don't want to build a website."

  ## 4. Design Doc
  - **High-Level Architecture & Entities:**
    - `OnboardingAgent`: A conversational AI agent service designed to interact with new users, extracting intent and business details.
    - `TenantProvisioner`: Service responsible for automatically spinning up the database row (`Tenant` entity), assigning isolated storage, and configuring initial POS/Inventory settings.
    - `PaymentIntegration`: Handles auto-configuration of Stripe Connect using simplified parameters.
  - **Key Relationships & Integration Points:** The `OnboardingAgent` interacts with the `LLMProvider` (Gemini Pro/MiniMax) and orchestrates calls to the `TenantProvisioner` and `PaymentIntegration` once the user confirms their intent.
  - **UI Wireframes / Screen Flow (Mobile-First 375px):**
    1. User signs up and is greeted by a full-screen, conversational UI (macOS Translucent Glass style).
    2. Agent asks: "What do you sell and what's your business name?"
    3. User replies via text/voice (e.g., "I sell custom vegan cakes in Austin, called Maya's Bakery").
    4. Agent displays a translucent loading overlay with live steps: "Registering name... Setting up products... Configuring payments..."
    5. Agent presents a success screen with a ready-to-use storefront URL and a pre-configured deposit product link.

  ## 5. Implementation Prompt
  - **User-Facing Outcome:** A completely conversational onboarding experience where the user creates their business profile, first product, and payment link without touching a single settings dashboard.
  - **Critical User Journey (CUJ):**
    1. Owner signs up.
    2. Interacts via text/voice with the Onboarding Agent.
    3. Approves the generated business profile.
    4. Receives a live storefront URL and their first product link ready to share on social media.
  - **Acceptance Criteria:**
    - The AI agent successfully extracts the business name, industry, and main product from a single natural language prompt.
    - The tenant is provisioned automatically with these exact details.
    - A default product/service is created and published.
    - The onboarding UI is fully responsive, verified with Playwright, and feels native on a 375px screen without horizontal scrolling.

  ## 6. Priority & Estimated Scope
  - **Priority:** P0
  - **Estimated Scope:** Large

  ## 7. Visual Excellence & Charts

  ### Competitive Landscape (Mermaid)
  ```mermaid
  graph TD;
      OHC[OHC: Zero-Click Onboarding] --> Traditional[Manual Setup];
      OHC --> AINative[AI-Native Generative];

      Traditional --> Shopify[Shopify: High Friction];
      Traditional --> Wix[Wix: Medium Friction];

      AINative --> Durable[Durable: Low Commerce Depth];
      AINative --> OHC_Agent[OHC: Low Friction + High Depth];
  ```

  ### Feature Gap Heatmap
  | Capability | OHC (Current) | OHC (Mission) | Shopify | Durable |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | 1 Hour | **< 10 Minutes** | Days | < 1 min |
  | **Commerce Depth**| High | **High** | High | Low |
  | **Conversational**| No | **Yes** | Partial | Yes |

  ## 8. References & Sources (50+ URLs Analyzed)
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.wix.com/ai-website-builder
  4. https://durable.co/
  5. https://www.10web.io/
  6. https://mixo.io/
  7. https://www.framer.com/ai/
  8. https://www.hubspot.com/products/ai
  9. https://squareups.com/us/en/software/ai
  10. https://www.intercom.com/fin
  11. https://www.lindy.ai/
  12. https://relevanceai.com/
  13. https://skyvern.com/
  14. https://www.11x.ai/
  15. https://www.agi.app/
  16. https://www.honeybook.com/ai
  17. https://www.dubsado.com/features/automation
  18. https://www.squarespace.com/design/ai-website-builder
  19. https://www.godaddy.com/ai
  20. https://www.bigcommerce.com/solutions/ai/
  21. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  22. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  23. https://www.trustpilot.com/review/durable.co
  24. https://www.trustpilot.com/review/10web.io
  25. https://www.g2.com/products/lindy-lindy/reviews
  26. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  27. https://techcrunch.com/2024/02/22/10web-armenia/
  28. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  29. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  30. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  31. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  32. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  33. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  34. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  35. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  36. https://www.relevanceai.com/customers/canva
  37. https://www.relevanceai.com/customers/kpmg
  38. https://www.11x.ai/customers
  39. https://www.11x.ai/blog/digital-workers-revenue
  40. https://fin.ai/cx-models
  41. https://www.intercom.com/blog/ai-agent-blueprint/
  42. https://www.hubspot.com/spotlight
  43. https://www.hubspot.com/new
  44. https://www.wix.com/blog/how-does-ai-work
  45. https://www.wix.com/blog/best-ai-website-builder
  46. https://durable.com/ai-website-builder
  47. https://durable.com/blog/durable-vs-squarespace
  48. https://www.lindy.ai/integrations
  49. https://www.lindy.ai/security
  50. https://skyvern.com/healthcare
  51. https://www.theagi.company/blog
  52. https://www.theagi.company/media-features
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
