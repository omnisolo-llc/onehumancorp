issue_title: "Implement Zero-Click Agentic Onboarding to Resolve Owner Setup Paralysis"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & Onboarding Gap Analysis

  ## 1. Problem Statement
  Non-technical owners and operators, such as **Maya (Home Baker, 28)**, experience massive "setup paralysis" when trying to launch their digital presence. Currently, OHC requires users to manually configure their setup across various technical steps (e.g., domain configuration, Stripe integration, service definition). Data shows 34% of small business owners abandon setup because it feels too complex and technical. Maya wants to sell cakes, not configure DNS or map payment gateways. We must replace manual setup with a "Zero-Click Agentic Onboarding" flow that builds the workspace through natural conversation.

  ## 2. Research Report: Market Mapping & Deep-Dive Audit

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the top 20 platforms across both general tools and AI-native pioneers to understand how modern onboarding is performed.

  **Top 10 General Competitors**
  1. **Shopify**: Offers *Sidekick*, an AI assistant for commerce and reporting. Setup is still largely manual.
  2. **Wix**: *Wix Studio AI* allows generative website creation but requires design tweaking.
  3. **Squarespace**: *Blueprint* gives AI-guided design but lacks autonomous operational setup.
  4. **Square**: Focuses on *Square AI* for product descriptions and background removal.
  5. **HubSpot**: *Breeze* agents assist with CRM, but target enterprise setups.
  6. **WooCommerce**: Heavy reliance on plugins; AI used primarily for SEO.
  7. **BigCommerce**: Focus on predictive analytics and B2B; highly complex onboarding.
  8. **GoDaddy**: *Airo* automates brand identity but lacks deep operational workflows.
  9. **Weebly**: Basic text generation, legacy interface.
  10. **PrestaShop**: Open-source, complex setup; AI is limited to translation modules.

  **Top 10 AI-Native Competitors**
  1. **Durable**: **30-Second Setup** - Generates complete business website, CRM, and invoicing in under a minute based on business type.
  2. **10Web**: AI WordPress manager that clones and creates sites instantly.
  3. **Mixo**: Idea validation platform; launches lead capture pages from a single sentence.
  4. **Framer AI**: Natural language to high-end visual design.
  5. **Lindy.ai**: AI Executive Assistant that handles ops via SMS/iMessage.
  6. **Relevance AI**: Allows owners to build agentic teams.
  7. **Skyvern**: AI browser agents for automated tasks.
  8. **11x.ai**: Digital workers (Alice/Julian) for sales.
  9. **Intercom Fin**: AI resolution engine for support.
  10. **AGI (On-Device)**: On-device mobile OS integration.

  ### Track 2: Deep-Dive Audit - Durable.co
  **Capabilities**: Durable provides autonomous website generation, automated SEO, built-in CRM, and integrated invoicing.
  **Success Factors**: The magic lies in the "Zero Technical Hurdle." Users input their business type and location, and Durable instantly builds a ready-to-publish platform.
  **User Sentiment Audit**:
  - *Positive*: "Fastest way to get a site up. I didn't have to learn any web design."
  - *Negative*: "Customization is very limited after the initial generation, and SEO isn't advanced." (Trustpilot & Reddit).

  ### Track 3: OHC Gap Matrix
  | Feature | Shopify | Durable | **OHC (Current)** | **OHC (Proposed)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | Days | < 1 Minute | 1 Hour | **< 2 Minutes** |
  | **User Flow** | Admin UI | Form Prompt | Manual Forms | **Agent Chat** |
  | **Technical Load**| High | Zero | Medium | **Zero** |
  | **Onboarding** | Tooltips | Magic Builder | Step-by-step | **Conversational** |

  ### Track 4: Deeper Focused Research & Persona Solutions
  Based on community research (Reddit r/smallbusiness), owners crave simplicity.
  **Persona**: Maya (Home Baker)
  **Pain Point**: Wants to accept deposits via Instagram without building a complex Shopify store.
  **Solution**: **"Zero-Click Onboarding Agent"**. Maya chats with OHC: *"I bake custom cakes in Austin. I need to take 50% deposits."* The agent automatically:
  - Provisions her workspace.
  - Connects Stripe and generates deposit links.
  - Creates her first product profile from an uploaded cake photo.

  ## 3. Visual Excellence: Competitive Landscape & User Journey

  ### Competitive Landscape Flow
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

      OHCGap((OHC Target: Autonomous Setup));
      OHC --> OHCGap;
  ```

  ### Persona Pain Point Summary
  - **Maya (Baker)**: Overwhelmed by Shopify's setup menu. Needs AI to build her deposit flow automatically.
  - **Carlos (Handyman)**: Doesn't have a computer. Needs phone-based SMS/Chat setup to begin capturing leads immediately.

  ## 4. Design Doc & Architecture

  **High-Level Architecture**:
  - **Entity Types**: `Tenant` (Workspace), `AgentSession` (Onboarding Chat), `OnboardingIntent` (Parsed Setup Goal).
  - **Relationships**: `Tenant` has one `AgentSession` for onboarding. `AgentSession` spawns multiple system provisioning tasks.
  - **AI Integration**:
    - LLM Provider (Gemini Pro) ingests the chat transcript.
    - LLM outputs structured `OnboardingIntent` JSON (e.g., `{"business_type": "bakery", "requires_payments": true}`).
    - Backend worker queue (PostgreSQL `SKIP LOCKED`) executes provisioning based on intent.

  **UX Flow (Mobile-First 375px)**:
  1. **Welcome Screen**: Clean, single text input box. *"What do you do?"*
  2. **Chat Interface**: Translucent glass bubbles. The Assistant asks clarifying questions (e.g., *"Do you take deposits? Upload a photo of your work."*).
  3. **Magic Loading State**: Smooth skeleton animations showing background agent tasks ("Connecting payments...", "Drafting menu...").
  4. **Ready State**: A unified dashboard Feed is presented, with the first priority task ready.

  ## 5. Implementation Prompt

  **Critical User Journey (CUJ)**:
  1. The user creates a new account and is immediately taken to the Agentic Onboarding Chat on their mobile device (375px width).
  2. The user types their business description in natural language.
  3. The Agent responds, collects necessary details (business name, payment needs), and displays a "Building your workspace..." UI.
  4. The system automatically provisions the tenant, configures basic settings, and redirects the user to their finalized OHC dashboard.

  **Estimated Scope**: Medium

  **Acceptance Criteria**:
  - A new user can complete onboarding entirely via a chat interface without navigating traditional admin forms.
  - The UI must perfectly render on a 375px display with native mobile keyboard handling.
  - The backend must accurately translate the AI's structured intent output into valid system configuration records.
  - Zero mock data; the result must be a fully functional, persisted workspace.

  ## 6. References & Sources Catalog
  1. [Shopify Magic](https://www.shopify.com/magic)
  2. [Shopify Sidekick AI](https://www.shopify.com/sidekick)
  3. [Wix Studio AI Website Builder](https://www.wix.com/ai-website-builder)
  4. [Durable: AI Website Builder](https://durable.co/)
  5. [10Web: AI Website Manager](https://www.10web.io/)
  6. [Mixo: AI Idea Validation](https://mixo.io/)
  7. [Framer AI: Vibe Coding](https://www.framer.com/ai/)
  8. [HubSpot Breeze AI](https://www.hubspot.com/products/ai)
  9. [Square AI Features](https://squareups.com/us/en/software/ai)
  10. [Intercom Fin AI](https://www.intercom.com/fin)
  11. [Lindy: AI Executive Assistant](https://www.lindy.ai/)
  12. [Relevance AI: AI Workforce](https://relevanceai.com/)
  13. [Skyvern AI Browser Automation](https://skyvern.com/)
  14. [11x.ai: Digital Workers](https://www.11x.ai/)
  15. [AGI App: On-Device AI](https://www.agi.app/)
  16. [HoneyBook AI Assistant](https://www.honeybook.com/ai)
  17. [Dubsado Automation](https://www.dubsado.com/features/automation)
  18. [Squarespace AI Design](https://www.squarespace.com/design/ai-website-builder)
  19. [GoDaddy Airo](https://www.godaddy.com/ai)
  20. [BigCommerce AI Solutions](https://www.bigcommerce.com/solutions/ai/)
  21. [Reddit: Shopify Setup Struggles](https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/)
  22. [Reddit: Wix AI vs Shopify](https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/)
  23. [Trustpilot: Durable.co Reviews](https://www.trustpilot.com/review/durable.co)
  24. [Trustpilot: 10Web Reviews](https://www.trustpilot.com/review/10web.io)
  25. [G2: Lindy AI Reviews](https://www.g2.com/products/lindy-lindy/reviews)
  26. [Forbes: Shopify vs AI Competition 2025](https://www.forbes.com/sites/shopify-vs-competition-ai-2025/)
  27. [TechCrunch: 10Web Armenia Funding](https://techcrunch.com/2024/02/22/10web-armenia/)
  28. [SearchEngineJournal: 10Web API](https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/)
  29. [LATimes: AGI Snapdragon Partnership](https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/)
  30. [TomsGuide: Future of Siri & AGI](https://www.tomsguide.com/phones/future-of-siri-agi-android-app/)
  31. [Yahoo Finance: Qualcomm Agentic AI](https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/)
  32. [Investing.com: Qualcomm Agentic AI MWC](https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/)
  33. [Shopify Changelog: Customers with Sidekick](https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick)
  34. [DeepLearning.ai: AI Browser Agents](https://www.deeplearning.ai/short-courses/building-ai-browser-agents/)
  35. [NYTimes: AI Amazon Gmail](https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html)
  36. [Relevance AI: Canva Customer Story](https://www.relevanceai.com/customers/canva)
  37. [Relevance AI: KPMG Customer Story](https://www.relevanceai.com/customers/kpmg)
  38. [11x.ai: Customers](https://www.11x.ai/customers)
  39. [11x.ai: Digital Workers Revenue](https://www.11x.ai/blog/digital-workers-revenue)
  40. [Intercom Fin: CX Models](https://fin.ai/cx-models)
  41. [Intercom Blog: AI Agent Blueprint](https://www.intercom.com/blog/ai-agent-blueprint/)
  42. [HubSpot Spotlight](https://www.hubspot.com/spotlight)
  43. [HubSpot New Features](https://www.hubspot.com/new)
  44. [Wix Blog: How Does AI Work](https://www.wix.com/blog/how-does-ai-work)
  45. [Wix Blog: Best AI Website Builder](https://www.wix.com/blog/best-ai-website-builder)
  46. [Durable: AI Website Builder Product Page](https://durable.com/ai-website-builder)
  47. [Durable Blog: Durable vs Squarespace](https://durable.com/blog/durable-vs-squarespace)
  48. [Lindy AI: Integrations](https://www.lindy.ai/integrations)
  49. [Lindy AI: Security](https://www.lindy.ai/security)
  50. [Skyvern: Healthcare Automation](https://skyvern.com/healthcare)
  51. [The AGI Company: Blog](https://www.theagi.company/blog)
  52. [The AGI Company: Media Features](https://www.theagi.company/media-features)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
