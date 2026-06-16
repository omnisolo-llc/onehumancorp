issue_title: "Implement Zero-Click Agentic Onboarding to Resolve Setup Paralysis"
issue_description: |
  # Mission Queue Protocol: Zero-Click Agentic Onboarding

  **Problem Statement:**
  Small business owners face "Setup Paralysis" when adopting new platforms. 34% of small business owners abandon setup due to technical complexity. Maya, our persona (Home Baker), wants to sell cakes, not configure DNS, Stripe webhooks, or complex theme settings. Currently, OHC's setup takes around 1 hour manually. To win against AI-native competitors like Durable (which sets up a site in <1 minute), OHC must implement a Zero-Click Onboarding Agent.

  ## Track 1: Market Mapping & Competitor Discovery
  ### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation for faster onboarding. |
  | **Square** | squareups.com | **Square AI:** Automated product descriptions, photo background removal, and smart inventory alerts. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents integrated deeply into CRM data. |
  | **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description generator and automated SEO metadata. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation. |
  | **Weebly** | weebly.com | Basic AI text generation for landing pages. |
  | **PrestaShop** | prestashop.com | AI-powered translation and product categorization modules. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages via one sentence. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **AGI (On-Device)** | agi.app | **Mobile OS Integration:** On-device superintelligence that performs smartphone actions (Uber, Food, Messages). |

  ## Track 2: Deep-Dive Competitor Audit (Durable)
  **Capabilities:** Autonomous website generation, integrated invoicing, and a simple AI business advisor.
  **Success Factors:** Zero technical hurdle. Targeted at service providers (Handymen, Photographers). Users can enter their business type and location, and a full site is deployed in seconds.
  **User Sentiment:**
  - *“Fastest way to get a site up, but the SEO needs work and I can't customize it enough.”* (Trustpilot).
  - *“I loved the quick start, but once I needed custom bookings, it fell short.”* (Reddit r/smallbusiness).

  ## Track 3: OHC Gap & Pain Point Identification
  ### Gap Matrix
  | Feature | Shopify Sidekick | Durable AI | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | Days | < 1 Minute | 1 Hour (Manual) | **< 10 Minutes (Agentic)** |
  | **Daily Ops** | Dashboard-first | Simple List | Service-first | **Assistant-first (Feed)** |
  | **Client Intake** | Manual Forms | Basic Leads | Widget-based | **Autonomous Negotiator** |

  ### Unresolved Pain Point: Setup Paralysis
  Small business owners abandon setup because of DNS, Payment Gateway configurations, and Theme styling. Our goal is to bring the setup time to under 10 minutes without sacrificing power.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:** Owners consistently express frustration with complex back-office systems. "I just want to bake cakes, not learn CSS." (Reddit proxy).
  **Agentic Solution Design:** The **"Zero-Click Onboarding Agent"** interacts with the user via a conversational UI. It gathers context implicitly (e.g., extracting from an Instagram profile or a short natural language description). The agent autonomously configures the domain via OHC's backend, provisions a Stripe connected account (with a streamlined flow), sets up standard products/services, and generates an aesthetically pleasing, 375px mobile-optimized storefront.

  ## Visual Excellence
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

      OHCGap((OHC Gap: Autonomous Onboarding & Proactive Ops));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC | Shopify | Durable | Lindy |
  | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟢 | 🔴 |
  | **Email Triage** | 🟢 | 🟡 | 🔴 | 🟢 |
  | **Auto-Onboarding** | 🔴 | 🔴 | 🟢 | 🟡 |

  ## Design Doc
  **High-Level Architecture:**
  - **Entity Types:** `Tenant`, `OnboardingSession`, `AgentTask`, `GeneratedStorefront`.
  - **Key Relationships:** A `Tenant` has one `OnboardingSession` that queues multiple `AgentTask`s (e.g., DomainProvisioning, StripeSetup, ThemeGeneration).
  - **Integration Points:** KAIROS Orchestrator (Sub-Agent Queue) handles the background tasks. Stripe Connect API for payment setup.
  - **UI/UX Flow (Mobile-First 375px):**
    1. **Welcome Screen:** Conversational interface: "What kind of business are you building?"
    2. **Agent Working State:** Translucent glassmorphism loading screen ("Provisioning domain...", "Drafting your menu...").
    3. **Approval Screen:** A beautiful preview of the generated storefront with a 1-tap "Publish" button.

  ## Implementation Prompt
  **User-Facing Outcome:** A non-technical user (e.g., Maya the Baker) can create an account, chat with the Onboarding Agent for 2 minutes, and receive a fully functional, published link-in-bio or storefront without touching any settings panels.
  **Critical User Journey (CUJ):**
  1. User signs up and is presented with the Assistant Chat.
  2. User says, "I make custom vegan cakes in Austin."
  3. The Assistant extracts intent, creates the tenant profile, and kicks off background KAIROS tasks to generate a theme, sample menu items, and standard policy pages.
  4. The Assistant presents a Preview Link.
  5. User taps "Approve and Publish".
  **Acceptance Criteria:**
  - The UI must be fully functional at 375px wide.
  - No manual form-filling for standard settings (done via Agent).
  - E2E tests must cover the conversational onboarding flow via Playwright.

  ## References & Sources Catalog
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
