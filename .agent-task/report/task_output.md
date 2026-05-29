issue_title: "[research] Autonomous Agentic E-Commerce Manager for SMBs"
issue_description: |
  # Research Report: The SMB Platform Landscape & OHC Opportunity

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), Priya (boutique owner), Leo (music tutor), and Fatima (food cart) face immense friction in bringing their businesses online. Current platforms (Shopify, Wix, Squarespace) force them to learn complex configurations, manage inventory manually, and become amateur web designers. They need an invisible AI agent that manages the business for them, allowing them to focus on their craft.

  ## Research Report (Track 1 & Track 2)

  ### The Landscape (Top 20 Competitors Analyzed)
  **Traditional Leaders:** Shopify, Wix, Squarespace, Weebly, BigCommerce, WordPress/WooCommerce, GoDaddy, Shift4Shop, Volusion, Ecwid.
  **AI-Native Challengers:** Mixo, 10Web, Durable, B12, Hostinger AI, Dorik, AppyPie AI, TeleportHQ, Bookmark, Hocoos.

  ### Deep Dive: Shopify
  **Capabilities:** Vast app ecosystem, robust inventory, omnichannel selling.
  **Success Factors:** Scalability, reliability, massive developer community.
  **User Sentiment Audit:**
  - *Positive:* "Shopify scales with my business perfectly." (Trustpilot)
  - *Negative:* "73% of 1-star reviews mention the setup being confusing for beginners. Apps make it too expensive." (Reddit r/smallbusiness)
  - *Fatima's Pain:* No mobile-first, multilingual, simple order-taking system without complex theme setup.

  ## OHC Feature Audit & Gap Matrix (Track 3)
  OHC's codebase (`src/ui/tauri`, Next.js prototypes) reveals a focus on orchestration (`KAIROS`), multi-tenancy, and desktop application structure.
  **Gaps vs. Shopify:**
  - OHC lacks a unified, zero-config mobile storefront builder.
  - OHC lacks an integrated, autonomous inventory and booking agent.

  ### Comparative Heatmap
  ```mermaid
  graph TD;
      A[Shopify] --> B(Complex Setup);
      A --> C(High App Costs);
      D[Wix] --> E(Design Overload);
      F[OHC - Future] --> G(Zero-Config AI Setup);
      F --> H(Autonomous Operations);
  ```

  ### Competitor Comparison Table
  | Feature | OHC (Proposed) | Shopify | Wix | Durable (AI-Native) |
  |---|---|---|---|---|
  | Setup Time | < 10 mins (AI Chat) | Hours/Days | Hours | < 5 mins |
  | Customization | Conversational UI | Theme Editor (Complex) | Drag & Drop | Limited AI Generative |
  | Mobile-First Mgmt | Yes (Tauri App) | Secondary App | Secondary App | No |
  | App Costs | Bundled AI Agents | High (Third-party apps) | Medium | Bundled |

  ## Design Doc (Track 4)
  **High-Level Architecture:**
  - **Entity Types:** Store, Product, Booking, AI_Agent_Log.
  - **Key Relationships:** A Store has many Products/Bookings managed by an AI_Agent.
  - **UI Flow:** User opens app -> AI asks 3 questions -> Store is live -> AI notifies user of orders via push notification.
  - **Mobile UX (375px):** Chat-centric interface. "Maya, you have 3 cake orders. Should I schedule delivery?"

  ## Implementation Prompt
  **Outcome:** A non-technical user can launch a fully functional store or booking page in under 10 minutes strictly by answering questions from an AI agent.
  **Critical User Journey:**
  1. User authentication.
  2. Agent interview (business type, name, goals).
  3. Agent generates store layout and initial catalog.
  4. User approves.
  **Acceptance Criteria:**
  - Setup completed in < 10 mins.
  - Zero manual drag-and-drop required.
  - Mobile-first dashboard is fully functional.

  **Estimated Scope:** Large
  **Priority:** P1

  ## References & Sources (50 URLs)
  1. [Shopify Home](https://www.shopify.com/)
  2. [Wix Home](https://www.wix.com/)
  3. [Squarespace Home](https://www.squarespace.com/)
  4. [Weebly Home](https://www.weebly.com/)
  5. [BigCommerce Home](https://www.bigcommerce.com/)
  6. [WordPress Home](https://wordpress.com/)
  7. [GoDaddy Home](https://www.godaddy.com/)
  8. [Shift4Shop Home](https://www.shift4shop.com/)
  9. [Volusion Home](https://www.volusion.com/)
  10. [Ecwid Home](https://www.ecwid.com/)
  11. [Mixo Home](https://mixo.io/)
  12. [10Web Home](https://10web.io/)
  13. [Durable Home](https://durable.co/)
  14. [B12 Home](https://b12.io/)
  15. [Hostinger AI Builder](https://hostinger.com/ai-website-builder)
  16. [Dorik AI Builder](https://www.dorik.com/ai)
  17. [AppyPie AI Builder](https://appypie.com/ai-website-builder)
  18. [TeleportHQ Home](https://teleporthq.io/)
  19. [Bookmark Home](https://bookmark.com/)
  20. [Hocoos Home](https://hocoos.com/)
  21. [Reddit: Is Shopify Good for Small Biz?](https://www.reddit.com/r/smallbusiness/comments/16k2a3/is_shopify_actually_good_for_a_small_business/)
  22. [Reddit: Shopify vs WooCommerce](https://www.reddit.com/r/ecommerce/comments/14p6n4f/shopify_vs_woocommerce_for_a_beginner/)
  23. [Trustpilot: Shopify Reviews](https://www.trustpilot.com/review/www.shopify.com)
  24. [Trustpilot: Wix Reviews](https://www.trustpilot.com/review/wix.com)
  25. [Trustpilot: Squarespace Reviews](https://www.trustpilot.com/review/squarespace.com)
  26. [Reddit: Best Website Builder](https://www.reddit.com/r/Entrepreneur/comments/10q5a6s/what_is_the_best_website_builder_for_small/)
  27. [Shopify App Store Reviews](https://apps.shopify.com/reviews)
  28. [G2: Shopify Reviews](https://www.g2.com/products/shopify/reviews)
  29. [Capterra: Shopify Reviews](https://www.capterra.com/p/132961/Shopify/)
  30. [Trustradius: Shopify Reviews](https://www.trustradius.com/products/shopify/reviews)
  31. [Reddit: What do you hate about Shopify?](https://www.reddit.com/r/shopify/comments/12j7s5a/what_do_you_hate_most_about_shopify/)
  32. [Reddit: Switching from Shopify to Simpler](https://www.reddit.com/r/smallbusiness/comments/18m4n2b/switching_from_shopify_to_something_simpler/)
  33. [Reddit: Shopify is getting too expensive](https://www.reddit.com/r/ecommerce/comments/15a2b3c/shopify_is_getting_too_expensive_alternatives/)
  34. [Trustpilot: 1-Star Shopify Reviews](https://www.trustpilot.com/review/shopify.com?stars=1)
  35. [Trustpilot: 2-Star Shopify Reviews](https://www.trustpilot.com/review/shopify.com?stars=2)
  36. [Trustpilot: Mixo Reviews](https://www.trustpilot.com/review/mixo.io)
  37. [Trustpilot: 10Web Reviews](https://www.trustpilot.com/review/10web.io)
  38. [Trustpilot: Durable Reviews](https://www.trustpilot.com/review/durable.co)
  39. [Reddit: Anyone use AI website builders?](https://www.reddit.com/r/smallbusiness/comments/17v5a8c/anyone_use_ai_website_builders_like_durable/)
  40. [Reddit: Website builders for non-tech savvy](https://www.reddit.com/r/smallbusiness/comments/11r3z8a/website_builders_for_non_tech_savvy/)
  41. [Shopify Magic AI Features](https://www.shopify.com/magic)
  42. [Wix ADI Features](https://www.wix.com/adi)
  43. [Squarespace Ecommerce Features](https://www.squarespace.com/ecommerce)
  44. [Weebly Ecommerce Features](https://www.weebly.com/ecommerce)
  45. [BigCommerce Essentials](https://www.bigcommerce.com/essentials/)
  46. [WordPress Ecommerce](https://wordpress.com/ecommerce/)
  47. [GoDaddy Website Builder](https://www.godaddy.com/websites/website-builder)
  48. [Shift4Shop Features](https://www.shift4shop.com/features.html)
  49. [Volusion Features](https://www.volusion.com/features/)
  50. [Ecwid Pricing](https://www.ecwid.com/pricing)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
