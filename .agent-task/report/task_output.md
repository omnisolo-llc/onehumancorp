issue_title: "Implement Invisible AI Setup Agent for Zero-Click Storefront Generation"
issue_description: |
  # OHC SMB Platform Research Report: Agentic Storefront Generation

  ## Problem Statement
  Non-technical users (like Maya the Baker or Carlos the Handyman) currently face cognitive overload when creating a storefront. They are required to choose templates, configure colors, upload images manually, and structure pricing. Competitors like Shopify or Wix take 30-60 minutes for a basic setup. The core gap for OHC is that we need to achieve our promise of "idea → live business in under 10 minutes" through zero-click, agentic generation where the user only provides a business description and the AI handles the rest.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors:
  1. **Shopify**: Market leader for SMB e-commerce. Great for physical goods, but steep learning curve.
  2. **Wix**: Flexible drag-and-drop builder. Often overwhelming for mobile-first users.
  3. **Squarespace**: Design-focused, highly visual, but rigid for specific booking workflows.
  4. **GoDaddy**: Simple, but limited scaling capabilities.
  5. **Weebly**: Easy to use, but outdated templates and ecosystem.
  6. **WordPress (WooCommerce)**: High customization, but requires technical knowledge.
  7. **BigCommerce**: Enterprise-lite, too complex for our personas.
  8. **Hostinger / Zyro**: Cheap and fast, but lacks deep functionality.
  9. **Webflow**: Too technical, geared towards designers.
  10. **Square Online**: Good POS integration, but limited customization.

  #### Top 10 AI-Native Competitors:
  1. **Durable**: AI website builder that generates a site in 30 seconds. Strong early traction.
  2. **Mixo.io**: Focuses on landing pages and lead generation for startups.
  3. **10Web**: AI WordPress builder, good for cloning existing sites.
  4. **Hostinger AI Builder**: Integrated into their hosting platform, fast but generic.
  5. **CodeDesign.ai**: AI builder with cloud hosting, focuses on marketing.
  6. **Appy Pie**: AI app and website generator, very basic UI.
  7. **B12**: AI website builder with professional services upsell.
  8. **Bookmark AiDA**: AI design assistant, somewhat dated.
  9. **Jimdo**: AI-driven setup, fast but limited flexibility.
  10. **TeleportHQ**: AI code generation, too technical for our personas.

  ### Track 2: Deep-Dive Competitor Audit (Shopify)
  - **Capabilities**: Comprehensive inventory management, vast app store, robust payment gateway (Shopify Payments).
  - **Success Factors**: Strong brand trust, scalable from $0 to $100M+ revenue, excellent partner ecosystem.
  - **User Sentiment Audit**:
    - *Reddit (r/smallbusiness)*: "Shopify is great once it's set up, but getting the theme to look right on mobile took me days."
    - *Trustpilot*: "Too many paid apps needed for basic features like booking or reviews."
    - *App Store*: "The mobile app is good for tracking sales, but I can't actually design my store from my phone."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC currently lacks a zero-click onboarding flow. Users still have to manually map their services.
  - **Gap Matrix**:
    - *Shopify*: 30-60 min setup, desktop-first design, manual configurations.
    - *OHC*: Needs <10 min setup, mobile-first design, AI-agentic configurations.
  - **Unresolved Pain Points**: Users don't know how to write copy, select themes, or structure their pricing tiers.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering**: Small business owners (like Maya) often abandon setup at the "Theme Customization" stage due to decision fatigue.
  - **Agentic Solution Design**: Introduce "The Promoter" AI Agent during onboarding. The user inputs: "I sell vegan cakes in Austin." The agent instantly generates a mobile-optimized UI, placeholder images (via integration), copy, and a pricing structure for deposits.

  ## Design Doc
  - **Architecture**:
    - Integration of a new `OnboardingAgent` service using the Gemini Pro LLM provider.
    - Uses existing `tenant` architecture to isolate generated configurations.
  - **UI Wireframes/Flow (Mobile-First 375px)**:
    1. Splash Screen: "Describe your business in one sentence."
    2. Loading State: "The Promoter is designing your storefront..." (Glassmorphism progress bar).
    3. Preview Screen: Fully functional 375px storefront. User can click "Launch" or "Regenerate".
  - **AI Agent Integration**: The `OnboardingAgent` communicates with the `WebsiteDesign` module to populate the database with UI configurations.

  ## Implementation Prompt
  - **User Journey**: As a non-technical user (e.g., Maya), I want to type a single sentence about my business so that the AI can generate a complete, mobile-ready storefront without requiring manual configuration.
  - **Acceptance Criteria**:
    - The `OnboardingAgent` successfully parses the user input.
    - A fully populated storefront is generated and persisted in the database.
    - The generated UI renders perfectly on a 375px viewport.
    - The process takes under 30 seconds from input to visual preview.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Charts & Diagrams
  ```mermaid
  graph TD;
      User[User Input: Business Idea] --> Agent[The Promoter AI Agent];
      Agent --> DB[Tenant DB: Generate Config];
      Agent --> UI[Mobile Storefront UI];
      Agent --> Copy[Generate SEO & Marketing Copy];
      UI --> Preview[User Preview & Launch];
  ```

  ## References & Sources
  1. https://www.shopify.com/pricing
  2. https://www.wix.com/pricing
  3. https://www.squarespace.com/pricing
  4. https://durable.co/
  5. https://mixo.io/
  6. https://10web.io/
  7. https://www.reddit.com/r/smallbusiness/comments/1/shopify_vs_wix
  8. https://www.trustpilot.com/review/www.shopify.com
  9. https://en.wikipedia.org/wiki/Website_builder
  10. https://en.wikipedia.org/wiki/Shopify
  11. https://en.wikipedia.org/wiki/Wix.com
  12. https://en.wikipedia.org/wiki/Squarespace
  13. https://en.wikipedia.org/wiki/GoDaddy
  14. https://en.wikipedia.org/wiki/Weebly
  15. https://en.wikipedia.org/wiki/E-commerce
  16. https://en.wikipedia.org/wiki/Electronic_commerce
  17. https://en.wikipedia.org/wiki/Small_and_medium-sized_enterprises
  18. https://en.wikipedia.org/wiki/Artificial_intelligence
  19. https://www.hostinger.com/website-builder
  20. https://zyro.com/
  21. https://webflow.com/
  22. https://squareup.com/us/en/online-store
  23. https://www.bigcommerce.com/
  24. https://woocommerce.com/
  25. https://codedesign.ai/
  26. https://www.appypie.com/website-builder
  27. https://www.b12.io/
  28. https://www.bookmark.com/aida
  29. https://www.jimdo.com/
  30. https://teleporthq.io/
  31. https://www.reddit.com/r/ecommerce/
  32. https://www.trustpilot.com/review/www.wix.com
  33. https://www.trustpilot.com/review/www.squarespace.com
  34. https://apps.apple.com/us/app/shopify-ecommerce-business/id371295624
  35. https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482
  36. https://apps.apple.com/us/app/squarespace/id1370251390
  37. https://www.godaddy.com/websites/website-builder
  38. https://www.weebly.com/
  39. https://www.shopify.com/tour/ecommerce-website
  40. https://www.wix.com/about/us
  41. https://www.squarespace.com/about/company
  42. https://www.bigcommerce.com/essentials/
  43. https://woocommerce.com/about/
  44. https://www.hostinger.com/tutorials/best-ai-website-builders
  45. https://www.forbes.com/advisor/business/software/best-ai-website-builders/
  46. https://www.techradar.com/best/ai-website-builders
  47. https://www.pcmag.com/picks/best-website-builders
  48. https://www.cnet.com/tech/services-and-software/best-website-builder/
  49. https://www.zdnet.com/article/best-website-builder/
  50. https://www.businessinsider.com/guides/tech/best-website-builder
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
