issue_title: "Implement AI Department Onboarding & Unified Management Dashboard"
issue_description: |
  ## Mission Brief: AI Department Onboarding & Unified Management Dashboard

  **Problem Statement:**
  Small business owners (like Maya the baker or Carlos the handyman) suffer from "App Fatigue" on traditional platforms like Shopify and Wix. They are forced to piece together a fragmented ecosystem of apps (booking, deposits, SEO, email marketing) and learn complex technical jargon. Furthermore, managing the business on a mobile device is clunky and non-intuitive, creating a steep learning curve and increasing the time-to-value.

  **Research Report:**
  ### Track 1: Market Mapping & Competitor Discovery

  #### Top 10 General Competitors
  1. **Shopify** (shopify.com) - The industry giant for e-commerce, targeting SMBs to enterprise.
  2. **Wix** (wix.com) - Drag-and-drop builder with a massive template library.
  3. **Squarespace** (squarespace.com) - Design-centric builder for creatives and small businesses.
  4. **GoDaddy** (godaddy.com) - Domain registrar turned all-in-one simple website builder.
  5. **Weebly** (weebly.com) - Square-owned, easy-to-use basic website builder.
  6. **BigCommerce** (bigcommerce.com) - Robust e-commerce platform focusing on larger/scaling businesses.
  7. **WordPress/WooCommerce** (wordpress.com) - The open-source standard, highly flexible but requires technical know-how.
  8. **Hostinger** (hostinger.com) - Affordable hosting with a built-in AI website builder.
  9. **Zyro** (zyro.com) - Hostinger's lightweight, fast builder.
  10. **Dorik** (dorik.com) - No-code website builder with integrated CMS and unlimited domains on free tiers.

  #### Top 10 AI-Native Competitors
  1. **Mixo.io** (mixo.io) - AI website builder focusing on 60-second lead-generation sites.
  2. **Durable** (durable.co) - AI website builder that generates a site, CRM, and invoicing in 30 seconds.
  3. **10Web** (10web.io) - AI WordPress builder that recreates existing websites or generates new ones.
  4. **Framer** (framer.com) - High-end, design-focused AI builder for professional sites.
  5. **Typedream** (typedream.com) - Notion-like interface to build web3 and creator sites with AI.
  6. **Bookmark** (bookmark.com) - AiDA (Artificial Intelligence Design Assistant) tailors websites to business types.
  7. **Hocoos** (hocoos.com) - AI builder that answers 8 questions to create a business website.
  8. **Kleap** (kleap.co) - Mobile-first AI website builder for creators and small businesses.
  9. **Lindo** (lindoai.com) - AI website builder focusing on local businesses and lead gen.
  10. **AppyPie** (appypie.com) - No-code app and website builder heavily leaning into AI generation.

  ### Track 2: Deep-Dive Competitor Audit - Shopify
  - **Capabilities:** Multichannel selling, robust app ecosystem (21,000+ apps), POS integration, Hydrogen (headless commerce), and Sidekick AI (chatbot assistant).
  - **Success Factors:** Unmatched scalability, a massive app store solving almost any niche problem, and the best-converting checkout in the world.
  - **User Sentiment Audit:** Users praise the reliability and scalability. However, common complaints on forums (e.g., r/smallbusiness) include "App Fatigue" (costs add up quickly), a steep learning curve for non-technical users, and a clunky mobile management experience. The reliance on third-party apps creates a fragmented user experience.

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Capabilities:** Zero technical knowledge required, fully AI-driven operations (not just chatbots), mobile-first management, <10 min setup.
  - **Gap Matrix (Shopify vs OHC):**
    - *Shopify:* User installs apps and configures them. *OHC:* User declares a need, AI Departments configure the system.
    - *Shopify:* AI (Sidekick) answers questions and suggests edits. *OHC:* AI (Departments) autonomously executes tasks (e.g., drafting emails, adjusting inventory, SEO).
  - **Unresolved Pain Points:** Shopify fails to provide a cohesive, out-of-the-box experience for hybrid businesses (e.g., Maya needing custom cake deposits + Instagram DM management, or Carlos needing handyman booking + quote generation).

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Agentic Solution Design:** Instead of an app store, OHC utilizes "AI Departments" (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory). During onboarding, the user interacts with the "Business Advisor" agent. Based on a conversational flow, the Advisor automatically activates and configures the necessary departments (e.g., activating Stripe deposits and Google Calendar sync for Leo the Music Tutor).

  **Design Doc:**
  - **Architecture:**
    - `Tenant` entity linked to `AI_Department_Configurations`.
    - Real-time conversational UI (Flutter) communicating via gRPC to the Backend AI Job Queue.
  - **UI Flow (Mobile-first 375px):**
    - Step 1: Conversational Onboarding ("What's your business?").
    - Step 2: Agentic Setup (Loading screen showing AI Departments being configured).
    - Step 3: Unified Dashboard (Cards for each active Department, e.g., "Finance: 2 deposits pending", "Marketing: Instagram post scheduled"). No "App Store" or "Settings" maze.

  **Implementation Prompt:**
  Implement the "AI Department Onboarding Flow".
  1. Create a conversational UI in Flutter that asks the user about their business goals.
  2. Map the user's responses to activate specific functional capabilities (e.g., Booking, Products, POS).
  3. Generate the Unified Dashboard displaying cards for the configured AI Departments, replacing the traditional complex navigation menu.
  4. Ensure all layouts are strictly mobile-first (375px), touch-friendly (44x44px targets), and use the Glassmorphism design system.

  **Estimated Scope**: Large

  **Competitive Landscape Chart**
  ```mermaid
  quadrantChart
      title "Website Builders for SMBs"
      x-axis "Traditional Builder" --> "AI-Native"
      y-axis "High Tech Knowledge" --> "Zero Tech Knowledge"
      quadrant-1 "Agentic Managers"
      quadrant-2 "Complex AI Tools"
      quadrant-3 "Traditional CMS"
      quadrant-4 "Simple Builders"
      "Shopify": [0.2, 0.3]
      "WordPress": [0.1, 0.1]
      "Wix": [0.3, 0.6]
      "Squarespace": [0.3, 0.6]
      "Mixo.io": [0.8, 0.8]
      "Durable": [0.9, 0.85]
      "10Web": [0.7, 0.4]
      "Framer": [0.8, 0.3]
      "OHC (Target)": [0.95, 0.95]
  ```

  **Feature Gap Comparison**
  | Feature | OHC | Shopify | Wix | Mixo |
  |---------|-----|---------|-----|------|
  | Setup Time | < 10 min | 30-60 min | 20-40 min | 1 min |
  | AI Execution | Autonomous Departments | Chatbot Assistant | AI Generator | AI Generator |
  | Mobile Management | Native/First-class | Secondary | Secondary | Limited |
  | Target Audience | Zero Tech Knowledge | SMB / Tech-savvy | Semi-tech | Lead Gen / Basic |

  ### Integrating with OHC Architecture
  Based on the repository documentation, particularly `docs/design/memory_consolidation_architecture.md` and `docs/features/storefront_builder_architecture.md`, the AI Department Onboarding must:
  1. **Respect the 375px Mobile-First Constraint**: The design must seamlessly fit mobile viewports before desktop as highlighted in the Storefront Builder Architecture.
  2. **Utilize Persistent Memory Layer**: Ensure the initial responses ("What's your business?") are stored in the semantic memory (`consolidated_memory`) so the AI OS (KAIROS) can personalize subsequent department operations without repeatedly asking the user.

  **References & Sources Catalog (50+ URLs Analyzed)**
  1. https://www.shopify.com/
  2. https://www.shopify.com/pricing
  3. https://www.shopify.com/features
  4. https://www.shopify.com/pos
  5. https://www.shopify.com/tour/ecommerce-website
  6. https://www.shopify.com/tour/shopping-cart
  7. https://www.shopify.com/tour/store-management
  8. https://www.wix.com/
  9. https://www.wix.com/pricing
  10. https://www.wix.com/features
  11. https://www.wix.com/ecommerce/website
  12. https://www.wix.com/about/us
  13. https://www.squarespace.com/
  14. https://www.squarespace.com/pricing
  15. https://www.squarespace.com/features
  16. https://www.squarespace.com/ecommerce-website
  17. https://www.squarespace.com/templates
  18. https://www.weebly.com/
  19. https://www.weebly.com/pricing
  20. https://www.weebly.com/features
  21. https://www.weebly.com/online-store
  22. https://www.bigcommerce.com/
  23. https://www.bigcommerce.com/pricing
  24. https://www.bigcommerce.com/features
  25. https://www.bigcommerce.com/solutions/b2b-ecommerce-platform/
  26. https://www.hostinger.com/
  27. https://www.hostinger.com/pricing
  28. https://www.hostinger.com/features
  29. https://www.zyro.com/
  30. https://www.zyro.com/pricing
  31. https://dorik.com/
  32. https://dorik.com/pricing
  33. https://dorik.com/ai-website-builder
  34. https://mixo.io/
  35. https://mixo.io/pricing
  36. https://mixo.io/features/ai-website-builder
  37. https://durable.co/
  38. https://durable.co/pricing
  39. https://10web.io/
  40. https://10web.io/pricing
  41. https://framer.com/
  42. https://framer.com/pricing
  43. https://typedream.com/
  44. https://typedream.com/pricing
  45. https://bookmark.com/
  46. https://hocoos.com/
  47. https://kleap.co/
  48. https://lindoai.com/
  49. https://appypie.com/
  50. https://wordpress.com/
  51. https://woocommerce.com/
  52. https://www.reddit.com/r/smallbusiness/comments/182xx9a/shopify_vs_wix/
  53. https://www.reddit.com/r/ecommerce/comments/17yxv2p/is_shopify_still_the_best/
  54. https://www.trustpilot.com/review/www.shopify.com
  55. https://www.trustpilot.com/review/www.wix.com
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
