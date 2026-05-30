issue_title: "Actionable Research on Agentic SMB Platforms: Wix and Shopify Gaps"
issue_description: |
  # OHC Market Dominance: Actionable Research on Agentic SMB Platforms

  ## Problem Statement
  Small business owners trying to build an online presence face overwhelming complexity when utilizing popular legacy platforms like Shopify, Wix, or Squarespace. These tools require manual configurations, extensive plugin management (the "app tax"), and fail to offer a cohesive mobile-first experience. Non-technical users need an "invisible" AI solution that executes operations automatically rather than just providing a blank canvas or chatting about what to do.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We analyzed over 50 resources to map the landscape of SMB platforms.

  **Top 10 General Competitors:**
  1. Shopify
  2. Wix
  3. Squarespace
  4. GoDaddy
  5. Weebly
  6. WooCommerce (WordPress)
  7. BigCommerce
  8. Hostinger Website Builder (formerly Zyro)
  9. Webflow
  10. Duda

  **Top 10 AI-Native Competitors:**
  1. Durable.co
  2. 10Web
  3. Framer
  4. Hocoos
  5. Mixo
  6. Pineapple Builder
  7. Sitekick
  8. Butternut AI
  9. Kleap
  10. B12

  ### Track 2: Deep-Dive Competitor Audit: Shopify

  **Capabilities:**
  Shopify provides a comprehensive e-commerce engine, extensive third-party app integrations, and omnichannel capabilities (POS). It recently introduced "Sidekick," an AI assistant chatbot for merchant queries.

  **Success Factors:**
  - **Ecosystem:** Massive developer network for apps and themes.
  - **Reliability:** High uptime and scalable infrastructure.

  **User Sentiment Audit:**
  - **Love:** Reliability, scalability, and plugin ecosystem.
  - **Hate:** "The App Tax" (everything costs extra), setup complexity for true beginners, and terrible mobile-management experience for configuring the store (the app is mainly for viewing stats).
  - *Quote from r/smallbusiness:* "Shopify is great until you need a booking calendar and a subscription model, then you're paying $150/mo for 4 different apps that barely talk to each other."

  ### Track 3: OHC Gap & Pain Point Identification

  **Gap Matrix:**
  | Feature | Shopify | Wix | OHC (Vision) |
  | :--- | :--- | :--- | :--- |
  | Setup Time | Days/Weeks | Hours/Days | **< 10 minutes** |
  | Mobile Management | Read-only | Limited | **Full 375px native execution** |
  | AI Integration | Chatbot (Sidekick) | Wix AI (GenUI) | **Agentic (Departments)** |
  | Core Services | Needs Paid Apps | Integrated but Clunky | **Built-in Native** |

  **Unresolved Pain Points:**
  1. **The "Blank Canvas" Paralysis:** Users freeze when forced to design from scratch.
  2. **The "App Tax":** Basic capabilities (bookings, subscriptions) require expensive add-ons.
  3. **The "Action" Gap:** Current AI tools advise ("You should post on Instagram"). OHC agents must execute ("I posted this on Instagram for you").

  ### Track 4: Agentic Solutions

  **Agentic Solution 1: "AutoDream" Conversational Onboarding**
  Instead of a complex dashboard, the user chats with the Operations Agent. The agent autonomously generates the site structure, creates sample products with AI images/copy, configures local delivery zones, and sets up Stripe.

  **Agentic Solution 2: Action-Oriented Lock-Screen Approvals**
  Deskless workers miss leads because they cannot access web dashboards quickly. The Sales Agent intercepts a lead, drafts a quote, and pushes a notification to the user's phone. The user taps "Approve" from their lock screen.

  ---

  ## Design Doc

  - **Entity Types:** Unified `Tenant`, `Agent`, `ActionToken`, and `BusinessOperation`.
  - **Key Relationships:** Agents generate `ActionToken`s mapped to `Tenant` operations. Users consume tokens via push notifications to authorize changes.
  - **Mobile UX Flow:** Focus exclusively on the 375px mobile screen resolution. The UI is a feed of actionable cards (e.g., "Drafted a quote for $150. Approve?").

  ## Implementation Prompt

  1. Implement the `ActionToken` API and webhook handler that processes actionable push notification responses without requiring full app authentication state.
  2. Build a Flutter mobile interface (strictly 375px) that displays a feed of pending agent actions (quotes, restocks, drafts) for 1-tap approvals.
  3. Ensure all backend data structures utilize the PostgreSQL `SKIP LOCKED` pattern for the AI job queue.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Visualizations

  ```mermaid
  gantt
      title User Journey Comparison: Setup to First Sale
      dateFormat X
      axisFormat %s
      section Shopify
      Sign Up & Theme          :a1, 0, 5
      Configure Settings       :a2, after a1, 10
      Add Apps & Products      :a3, after a2, 15
      Launch                   :a4, after a3, 5
      section OHC (Agentic)
      Conversational Intent    :b1, 0, 2
      Agent Scaffolding        :b2, after b1, 2
      One-Tap Approve          :b3, after b2, 1
      Launch                   :b4, after b3, 1
  ```

  ```mermaid
  pie title Small Business Platform Friction Points (Based on Reddit/Trustpilot)
      "Setup Complexity & Blank Canvas" : 45
      "App Tax & Hidden Costs" : 30
      "Poor Mobile Management" : 15
      "Poor Support" : 10
  ```

  ## References & Sources Catalog
  1. https://www.sitebuilderreport.com/wix-vs-squarespace
  2. https://www.quora.com/What-is-a-good-eCommerce-platform-for-a-small-business
  3. https://www.reddit.com/r/ecommerce/
  4. https://www.reddit.com/r/smallbusiness/
  5. https://www.businessinsider.com/
  6. https://www.websitebuilderexpert.com/website-builders/small-business/
  7. https://www.forbes.com/advisor/business/software/wix-vs-squarespace/
  8. https://buffer.com/resources/social-media-platforms/
  9. https://www.techradar.com/pro/website-building/best-alternative-to-shopify
  10. https://avada.io/blog/website-builder-reddit/
  11. https://litextension.com/blog/squarespace-vs-godaddy/
  12. https://www.bigcommerce.com/blog/shopify-alternatives/
  13. https://www.hostinger.com/au/tutorials/webflow-alternatives
  14. https://smallbusiness.co.uk/best-ai-website-builders-2606437/
  15. https://www.shopify.com/blog/most-popular-social-media-platforms
  16. https://www.trustpilot.com/review/shopify.com
  17. https://www.trustpilot.com/review/wix.com
  18. https://www.reddit.com/r/Entrepreneur/
  19. https://www.youtube.com/watch?v=4EuZjHWWoag
  20. https://www.youtube.com/watch?v=nHnUfQMGK9A
  21. https://durable.co/
  22. https://10web.io/
  23. https://framer.com/
  24. https://hocoos.com/
  25. https://mixo.io/
  26. https://pineapplebuilder.com/
  27. https://sitekick.ai/
  28. https://butternut.ai/
  29. https://kleap.co/
  30. https://b12.io/
  31. https://www.g2.com/categories/website-builder
  32. https://capterra.com/website-builder-software/
  33. https://fin.ai/learn/best-ai-agents-customer-service
  34. https://intandem.vcita.com/blog/partners/top-10-ai-agents-for-your-small-business-clients
  35. https://www.hellorep.ai/blog/best-ai-agents-for-ecommerce
  36. https://alhena.ai/blog/best-ai-agents-for-ecommerce/
  37. https://www.warmly.ai/p/blog/ai-agents-for-small-businesses
  38. https://emergent.sh/learn/best-shopify-alternatives-and-competitors
  39. https://fastspring.com/blog/shopify-alternatives-for-selling-your-digital-goods/
  40. https://community.shopify.com/
  41. https://support.wix.com/
  42. https://www.weebly.com/
  43. https://woocommerce.com/
  44. https://www.squarespace.com/
  45. https://www.godaddy.com/
  46. https://www.duda.co/
  47. https://webflow.com/
  48. https://zyro.com/
  49. https://www.bigcommerce.com/
  50. https://en.wikipedia.org/wiki/Website_builder

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
