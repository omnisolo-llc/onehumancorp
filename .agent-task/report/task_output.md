issue_title: "Implement Zero-Click Agentic Onboarding Flow"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## Mission Queue Protocol Brief

  **Title**: Implement Zero-Click Agentic Onboarding Flow
  **Problem Statement**: 34% of small business owners abandon setup due to technical complexity. Our target persona, Maya (Home Baker), wants to sell custom cakes via Instagram and manage custom-order deposits, but finds configuring DNS, connecting payment gateways, and creating structured products overwhelming. She needs to move from a scattered setup to a clear action plan in minutes, without any technical hurdles. OHC currently takes ~1 hour of manual setup, while competitors like Durable achieve this in under 1 minute.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the 2025 landscape of owner/operator work assistants, analyzing traditional giants and AI-native pioneers.

  **Top 10 General Competitors**
  * **Shopify (Sidekick)**: Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing.
  * **Wix (Studio AI)**: Generative website creation from prompts, AI-powered section generator.
  * **Squarespace (Blueprint)**: AI-guided design and content generation for faster onboarding.
  * **Square (AI)**: Automated product descriptions, photo background removal, and smart inventory alerts.
  * **HubSpot (Breeze)**: AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data.
  * **WooCommerce (AI)**: Product description generator and automated SEO metadata.
  * **BigCommerce (AI Predictive Analytics)**: Proactive sales forecasting and customer churn prediction.
  * **GoDaddy (Airo)**: Automated brand identity creation including logos and social media ads.
  * **Weebly**: Basic AI text generation for landing pages.
  * **PrestaShop**: AI-powered translation and product categorization modules.

  **Top 10 AI-Native Competitors**
  * **Durable**: Generates a complete business website, CRM, and invoicing in under a minute.
  * **10Web**: Instantly recreates any website design on WordPress using AI agents.
  * **Mixo**: Targeted at pre-revenue startups to launch lead-capture pages via one sentence.
  * **Framer AI**: High-end design output from natural language prompts, bypassing designers.
  * **Lindy.ai**: Handles email triage, scheduling, and admin tasks via iMessage/SMS.
  * **Relevance AI**: Allows non-technical owners to build autonomous agentic teams for sales and ops.
  * **Skyvern**: AI browser agents that can log into any portal to download invoices or fill forms.
  * **11x.ai**: Autonomous digital workers for outbound sales and inbound phone handling.
  * **Intercom Fin**: AI agent that resolves 50%+ of support queries without human intervention.
  * **AGI (On-Device)**: On-device superintelligence that performs smartphone actions (Uber, Food, Messages).

  ### Track 2: Deep-Dive Competitor Audit (Shopify & Durable)
  * **Shopify Sidekick & Magic**: Powerful integration with 8,000+ apps, proactive health signals. However, user sentiment shows setup remains highly complex ("Setup is still a nightmare. I spent 4 hours trying to fix shipping zones for local delivery.").
  * **Durable.co**: Zero technical hurdle, autonomous website generation targeted at service providers. User sentiment praises speed but notes limitations in customization ("Fastest way to get a site up, but the SEO needs work and I can't customize it enough.").

  ### Track 3: OHC Gap & Pain Point Identification
  OHC currently lacks the "Zero-to-One" autonomous experience found in Durable and the deep "Invisible Automation" of HubSpot Breeze.

  **Feature Gap Heatmap**

  | Capability | OHC | Shopify | Durable | Lindy |
  | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟢 | 🔴 |
  | **Email Triage** | 🟢 | 🟡 | 🔴 | 🟢 |
  | **Booking Logic** | 🟢 | 🟡 | 🟡 | 🟢 |
  | **Auto-Onboarding** | 🔴 | 🔴 | 🟢 | 🟡 |
  | **Agentic Ops** | 🟢 | 🟡 | 🔴 | 🟡 |

  **Gap Matrix**

  | Feature | Shopify Sidekick | Durable AI | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | Days | < 1 Minute | 1 Hour (Manual) | **< 10 Minutes (Agentic)** |
  | **Daily Ops** | Dashboard-first | Simple List | Service-first | **Assistant-first (Feed)** |
  | **Client Intake** | Manual Forms | Basic Leads | Widget-based | **Autonomous Negotiator** |
  | **Inventory** | Manual Sync | Manual | Database-backed | **Predictive Auto-restock** |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  * **Pain Point**: Setup Paralysis (Persona: Maya - Home Baker). 34% of small business owners abandon setup due to technical complexity. Maya wants to sell cakes, not configure DNS.
  * **Solution**: "Zero-Click Onboarding Agent". An AI agent that provisions domains, configures Stripe for custom deposits, and creates the first product from a photo using only natural language interactions.

  ## Design Doc

  **Architecture:**
  *   **Agent Interaction**: A conversational UI in the Flutter app (targeting 375px width first) powered by Gemini Pro (with fallback). The agent acts as a guide during initial login.
  *   **Backend Orchestration**: The KAIROS engine orchestrates tasks: `ProvisionDomain`, `ConfigureStripe`, `GenerateProduct`.
  *   **Data Models**: Expand tenant provisioning to include `OnboardingState` and `AgentConversations`.

  **UI/UX Flow (Mobile-First):**
  1.  **Welcome Screen**: Clean, Apple/Ubiquiti-style design. A simple chat interface: "Hi Maya, what kind of business are we starting today?"
  2.  **Photo Upload**: The agent prompts Maya to upload a photo of her cake.
  3.  **Processing State**: Translucent materials and clear status tokens indicate the agent is generating a product description, setting up a payment link, and reserving a subdomain.
  4.  **Confirmation**: A finalized card showing the product link, ready to be shared on Instagram DMs.

  ## Implementation Prompt

  **Critical User Journey (CUJ):**
  A new user (Maya) downloads the app and logs in. Instead of a complex dashboard, she is greeted by an AI assistant. The assistant asks for her business name and a photo of her product. The assistant then automatically creates a basic storefront, connects a test Stripe account, and generates a shareable payment link for her product. The user navigates this entire flow without filling out traditional multi-step forms.

  **Acceptance Criteria:**
  - [ ] A new user can complete the onboarding flow exclusively through chat interactions.
  - [ ] The agent correctly extracts business details and creates a tenant record.
  - [ ] Uploading a photo triggers the creation of a product entity with AI-generated title and description.
  - [ ] A shareable payment link/storefront URL is generated at the end of the flow.
  - [ ] The entire flow is fully functional and visually appealing on a 375px mobile screen.

  **Priority:** P0
  **Estimated Scope:** Large

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

  ## References & Sources
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
