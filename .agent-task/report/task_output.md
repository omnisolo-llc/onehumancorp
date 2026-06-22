issue_title: "Implement Autonomous AI Setup Wizard (Zero-Click Onboarding Agent)"
issue_description: |
  # Mission Queue Protocol: OHC Autonomous AI Setup Wizard

  ## Problem Statement
  Small business owners (like Maya the Baker or Carlos the Handyman) suffer from "Setup Paralysis." Our research shows that 34% of small business owners abandon software setup due to technical complexity. While competitors like Shopify provide powerful tools (e.g., Sidekick), setting up shipping zones, DNS, and initial inventory remains a multi-hour manual chore. Owners want to operate their business, not configure software. OHC must provide a "Zero-to-One" onboarding experience that gets them to value in under 10 minutes.

  ## Priority
  P1

  ## Estimated Scope
  Large

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery (Top 20)
  **Top 10 General Competitors:** Shopify, Square, HubSpot, Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Notion, Microsoft Copilot, Wix.
  **Top 10 AI-Native Competitors:** Shopify Sidekick, Notion AI, HubSpot Breeze, Agentforce, Harvey, Intercom Fin, Asana Intelligence, Zapier Central, ClickUp Brain, Durable.co.

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & Durable.co)
  **Shopify Sidekick:**
  - **Capabilities:** AI assistant that edits themes, writes product copy, and generates SQL-driven reports via ShopifyQL.
  - **Success Factors:** Deeply embedded in a massive ecosystem (8,000+ apps). Understands context because it lives in the admin dashboard.
  - **User Sentiment:** Users love the proactive insights ("suggest a discount code"), but complain about the rigid, traditional setup process that still exists underneath the AI layer.

  **Durable.co:**
  - **Capabilities:** Autonomous website generation, integrated invoicing, simple AI advisor.
  - **Success Factors:** The "30-Second Setup" hook. Zero technical knowledge required.
  - **User Sentiment:** Fast time-to-value, but lacks deep operational logic (inventory, complex routing) needed for growing businesses.

  ### Track 3: OHC Gap & Pain Point Identification
  OHC currently offers robust backend orchestration (KAIROS) and specialized services (booking, pos). However, compared to Durable's 30-second setup and Shopify's Sidekick, OHC lacks a unified, conversational **Autonomous Onboarding** flow. A user currently takes an hour to manually configure their workspace.

  **Pain Point to Solve:** The owner must configure settings before they can see the assistant working.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:** Reviews across Trustpilot and Reddit (r/smallbusiness) consistently highlight the frustration of mapping DNS and setting up payment processors.
  **Agentic Solution Design:** A "Zero-Click Onboarding Agent" that initiates immediately upon signup.
  - The agent asks the user "What do you do?" and "Upload a picture of something you sell."
  - Behind the scenes, the agent configures the tenant profile, sets up a landing page, creates the first product via image recognition, and generates a Stripe deposit link.

  ## Design Doc

  ### Architecture
  - **Entities:** `Tenant`, `AgentSession`, `OnboardingTask`.
  - **Key Relationships:** `Tenant` owns `AgentSession` which coordinates `OnboardingTask`s (e.g., Generate Logo, Draft Policies, Create First Product).
  - **Integration Points:** Gemini Pro (for parsing the user's initial prompt and image), Stripe (for automated payment link generation), and the OHC internal File Storage / POS services.

  ### UI/UX Flow (Mobile-First 375px)
  1. **Greeting Screen:** Minimalist chat interface. "Hi, I'm your OHC Assistant. What kind of business are we building today?"
  2. **Intake Chat:** The user types or speaks (e.g., "I bake vegan cakes in Austin").
  3. **Magic State:** A beautiful translucent glass loading state shows the agent working ("Writing copy...", "Setting up booking link...", "Designing storefront...").
  4. **The Reveal:** The user is presented with a fully functional feed/dashboard pre-populated with a sample booking widget, a generated logo, and a payment link ready to share on Instagram.

  ## Implementation Prompt
  Create the "Zero-Click Onboarding Agent" flow.
  **Critical User Journey (CUJ):**
  1. A new user signs up and lands on the onboarding screen.
  2. The user types a single sentence describing their business.
  3. The system processes the input via an AI agent, which autonomously populates the user's tenant with a generated business name, a draft product/service, and a ready-to-use payment link.
  4. The user is redirected to the Assistant Feed, where the first card is a success summary of what the agent built.
  **Acceptance Criteria:**
  - The UI must render perfectly at 375px.
  - The agent must successfully mutate the backend to create at least one product/service entity.
  - The flow must contain zero technical jargon (no "configure DNS" or "setup API keys" during this phase).
  - E2E Playwright tests must verify the entire flow from text input to the populated Assistant Feed.

  ## Visual Artifacts

  ### Competitive Landscape
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

      OHCGap((OHC Gap: Autonomous Onboarding));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC (Current) | Shopify | Durable |
  | :--- | :--- | :--- | :--- |
  | **Site/Store Gen** | 🟡 | 🟢 | 🟢 |
  | **Email Triage** | 🟢 | 🟡 | 🔴 |
  | **Booking Logic** | 🟢 | 🟡 | 🟡 |
  | **Auto-Onboarding**| 🔴 | 🔴 | 🟢 |

  ## References & Sources
  1. Shopify Magic (https://www.shopify.com/magic)
  2. Shopify Sidekick (https://www.shopify.com/sidekick)
  3. Wix AI Website Builder (https://www.wix.com/ai-website-builder)
  4. Durable (https://durable.co/)
  5. 10Web (https://www.10web.io/)
  6. Mixo (https://mixo.io/)
  7. Framer AI (https://www.framer.com/ai/)
  8. HubSpot AI (https://www.hubspot.com/products/ai)
  9. Square AI (https://squareups.com/us/en/software/ai)
  10. Intercom Fin (https://www.intercom.com/fin)
  11. Lindy (https://www.lindy.ai/)
  12. Relevance AI (https://relevanceai.com/)
  13. Skyvern (https://skyvern.com/)
  14. 11x (https://www.11x.ai/)
  15. AGI (https://www.agi.app/)
  16. HoneyBook AI (https://www.honeybook.com/ai)
  17. Dubsado Automation (https://www.dubsado.com/features/automation)
  18. Squarespace AI Website Builder (https://www.squarespace.com/design/ai-website-builder)
  19. GoDaddy AI (https://www.godaddy.com/ai)
  20. BigCommerce AI (https://www.bigcommerce.com/solutions/ai/)
  21. Reddit: Shopify Setup Struggles (https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/)
  22. Reddit: Wix AI vs Shopify (https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/)
  23. Trustpilot: Durable Review (https://www.trustpilot.com/review/durable.co)
  24. Trustpilot: 10Web Review (https://www.trustpilot.com/review/10web.io)
  25. G2: Lindy Reviews (https://www.g2.com/products/lindy-lindy/reviews)
  26. Forbes: Shopify vs Competition AI 2025 (https://www.forbes.com/sites/shopify-vs-competition-ai-2025/)
  27. TechCrunch: 10Web Armenia (https://techcrunch.com/2024/02/22/10web-armenia/)
  28. Search Engine Journal: 10Web releases API (https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/)
  29. LA Times: AGI Snapdragon Partnership (https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/)
  30. Toms Guide: Future of Siri AGI Android App (https://www.tomsguide.com/phones/future-of-siri-agi-android-app/)
  31. Yahoo Finance: Qualcomm says Agentic AI turns devices into operators (https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/)
  32. Investing.com: Qualcomm Agentic AI MWC (https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/)
  33. Shopify Changelog: Create customers and companies with Sidekick (https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick)
  34. Deeplearning.ai: Building AI Browser Agents (https://www.deeplearning.ai/short-courses/building-ai-browser-agents/)
  35. NY Times: Artificial Intelligence Amazon Gmail (https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html)
  36. Relevance AI Customers: Canva (https://www.relevanceai.com/customers/canva)
  37. Relevance AI Customers: KPMG (https://www.relevanceai.com/customers/kpmg)
  38. 11x Customers (https://www.11x.ai/customers)
  39. 11x Blog: Digital Workers Revenue (https://www.11x.ai/blog/digital-workers-revenue)
  40. Intercom Fin: CX Models (https://fin.ai/cx-models)
  41. Intercom Blog: AI Agent Blueprint (https://www.intercom.com/blog/ai-agent-blueprint/)
  42. HubSpot Spotlight (https://www.hubspot.com/spotlight)
  43. HubSpot New (https://www.hubspot.com/new)
  44. Wix Blog: How does AI work (https://www.wix.com/blog/how-does-ai-work)
  45. Wix Blog: Best AI Website Builder (https://www.wix.com/blog/best-ai-website-builder)
  46. Durable AI Website Builder (https://durable.com/ai-website-builder)
  47. Durable Blog: Durable vs Squarespace (https://durable.com/blog/durable-vs-squarespace)
  48. Lindy Integrations (https://www.lindy.ai/integrations)
  49. Lindy Security (https://www.lindy.ai/security)
  50. Skyvern Healthcare (https://skyvern.com/healthcare)
  51. The AGI Company Blog (https://www.theagi.company/blog)
  52. The AGI Company Media Features (https://www.theagi.company/media-features)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
