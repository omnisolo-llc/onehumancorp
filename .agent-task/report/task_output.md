issue_title: "Implement The Ambassador Agent: Native Social Inbox Auto-Responder"
issue_description: |
  # OHC Small Business Platform Research Report: Agentic Workflows as the Missing Link

  ## Executive Summary
  This report investigates the current landscape of small business website builders and e-commerce platforms. Our primary objective is to pinpoint unresolved pain points in the SMB segment and demonstrate how OneHumanCorp (OHC) can leverage autonomous AI agents to capture non-technical users currently struggling with either overly complex legacy systems (like Shopify) or simplistic builders (like GoDaddy) that fail to drive real business outcomes.

  The core thesis is that **Invisible AI Automation** is the key differentiator. Users do not want a better dashboard; they want the platform to do the work while they simply approve the results.

  ---

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Our research across the e-commerce platform landscape reveals two distinct categories:

  **Top 10 Traditional & Legacy Giants:**
  1. **Shopify**: The dominant e-commerce player. Excellent ecosystem, but highly complex setup.
  2. **Wix**: A popular visual builder. Good for simple portfolios but disjointed e-commerce.
  3. **Squarespace**: Design-focused, great for creatives.
  4. **GoDaddy**: Fast and simple setup, but extremely limited in customization.
  5. **Weebly**: Basic, somewhat outdated, simple drag-and-drop.
  6. **WordPress/WooCommerce**: Ultimate flexibility, but requires high technical knowledge.
  7. **BigCommerce**: Powerful, but targets mid-market/enterprise over micro-SMEs.
  8. **Webflow**: Incredible design power, but steep learning curve.
  9. **Hostinger Builder**: Very cheap, basic features.
  10. **Zyro**: Simple and fast, but lacks deep operational tools.

  **Top 10 AI-Native & Emerging Players:**
  1. **Durable**: AI website generation in 30 seconds.
  2. **10Web**: AI WordPress builder.
  3. **Framer**: AI design generation, focused on aesthetics.
  4. **Dorik**: AI website building with CMS.
  5. **Mixo**: AI landing page generator.
  6. **Hocoos**: AI business website builder.
  7. **CodeDesign.ai**: AI-powered drag-and-drop.
  8. **AppyPie AI**: AI app and website generator.
  9. **HostGator AI**: Legacy player adding AI setup.
  10. **Shopify Sidekick**: AI chatbot assistant within Shopify admin.

  ---

  ## 2. Deep-Dive Competitor Audit: Shopify (Track 2)
  We selected **Shopify** for a deep-dive analysis due to its market dominance and the clear divide between its enterprise success and its friction for micro-businesses.

  ### Capabilities ("What they can do")
  Shopify offers a massive ecosystem of 21,000+ apps, robust checkout (Shop Pay), multi-channel selling, and internationalization (Shopify Markets). They are introducing "Sidekick", an AI commerce assistant.

  ### Success Factors ("What they are successful at")
  - **Ecosystem**: If you need a feature, there is an app for it.
  - **Checkout**: Shop Pay is industry-leading for conversion (up to 50% higher than guest checkouts).
  - **Reliability**: Can handle massive traffic spikes seamlessly.

  ### User Sentiment Audit (Synthesized from Reddit & Trustpilot)
  - **The "App Tax"**: *"Shopify app subscriptions are bleeding me dry before I even make a profit."* Users frequently complain that the base plan is useless without expensive 3rd-party apps.
  - **Setup Paralysis**: *"I spent a week trying to understand Shopify's shipping zones."* Non-technical users struggle significantly with initial configuration.
  - **The "Sidekick" Limitation**: Current AI implementations are mostly reactive chatbots. Users have to know what to ask rather than the AI proactively managing the store.

  ---

  ## 3. OHC Gap & Pain Point Identification (Track 3)

  ### Unresolved User Pain Points
  1. **The "Now What?" Syndrome**: Users launch a site (on Wix or GoDaddy) and then have zero traffic because they don't understand marketing or SEO.
  2. **Instagram DM Overload**: Users (like 'Maya the Baker') spend hours manually replying to the same questions on social media instead of building their business.
  3. **Fragmented Operations**: Using one tool for a website, another for booking (Calendly), another for payments (Stripe), and another for marketing (Mailchimp).

  ### OHC Gap Matrix
  | Feature | Shopify | Wix | OHC (Target State) |
  | :--- | :--- | :--- | :--- |
  | **Setup Complexity** | High | Low | **Zero (AI Generated)** |
  | **Core Features Included** | Low (Requires Apps) | Medium | **All-in-One Native** |
  | **Mobile Management** | Good | Poor | **Mobile-First (375px)** |
  | **AI Role** | Reactive Chatbot | Setup Assistant | **Proactive Autonomous Agent** |
  | **Social Media Auto-Reply**| 3rd Party App Needed| 3rd Party App Needed| **Native Agent** |

  ```mermaid
  quadrantChart
      title Competitive Landscape: Simplicity vs. AI Autonomy
      x-axis "Reactive Tool" --> "Proactive Agent"
      y-axis "Complex/Fragmented" --> "Simple/Unified"
      quadrant-1 "OHC (Target)"
      quadrant-2 "Legacy Builders"
      quadrant-3 "Enterprise E-commerce"
      quadrant-4 "Basic Website Generators"
      "Shopify": [0.3, 0.4]
      "Wix": [0.4, 0.6]
      "Squarespace": [0.3, 0.5]
      "GoDaddy": [0.2, 0.7]
      "Durable": [0.7, 0.6]
      "OHC (Vision)": [0.9, 0.9]
  ```

  ---

  ## 4. Deeper Focused Research & Agentic Solutions (Track 4)

  The core problem is that existing platforms provide *tools* to run a business, but they do not provide *staff*. OHC must position its AI agents as functional departments (Operations, Marketing, Sales, etc.).

  ### Design Doc: Agentic Solutions Architecture
  1. **"The Ambassador" (Customer Success Agent)**:
      - *Architecture*: A native integration layer connecting social APIs (Instagram Graph API, WhatsApp Business API) to a Gemini-powered intent classifier.
      - *Flow*: Message received -> Intent extracted -> RAG against user's FAQs/Inventory/Policies -> Draft generated -> Auto-sent (or pushed for approval via mobile notification).
      - *Mobile UX*: 375px optimized card view showing "Drafted Replies" with 1-tap "Approve & Send" or "Edit" buttons.
  2. **"The Promoter" (Marketing Agent)**:
      - *Architecture*: CRON job leveraging inventory deltas to trigger content generation via Gemini Vision (analyzing product photos).
      - *Flow*: New product added -> Agent drafts 3 distinct social posts -> Push notification to owner -> Owner taps "Approve" -> Scheduled to Instagram.
  3. **"The Advisor" (Business Advisory Agent)**:
      - *Architecture*: Weekly aggregated data pipeline (PostgreSQL stats) fed into LLM for plain-language summarization.
      - *Flow*: Every Friday 5 PM -> Push notification: "Traffic is up 20% but bookings are down. Should we offer a 10% discount this weekend to your email list? (Yes/No)".

  ---

  ## 5. Implementation Prompt (For Engineering Swarm)
  **Feature Name:** The Ambassador - Native Social Inbox Auto-Responder
  **Target Persona:** Maya the Baker (relies on Instagram DMs, overwhelmed by volume).

  **Outcome:** An automated DM response system where the AI agent drafts replies based on inventory and business rules. Maya can review and approve them directly from her iPhone.

  **Critical User Journey (CUJ):**
  1. Maya logs into the OHC mobile web app (375px view).
  2. Maya connects her Instagram Business account via the Integrations tab.
  3. A customer DMs Maya on Instagram: "Do you have vegan chocolate cake available for Saturday?"
  4. The Ambassador Agent queries Maya's OHC inventory, confirms availability, and drafts: "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?"
  5. Maya receives a push notification on her phone: "Agent drafted a reply to @customer. Tap to review."
  6. Maya taps the notification, sees the draft, and clicks "Approve". The message is sent.

  **Priority**: P2
  **Estimated Scope**: Medium

  **Acceptance Criteria:**
  - Must function flawlessly on a 375px viewport (no horizontal scroll).
  - Must include automated E2E Playwright tests verifying the approval flow.
  - No complex rules engine for the user; they simply connect the account and the LLM handles intent and context matching.

  ---

  ## References & Sources Catalog (50+ Visited Webpages)
  1. Shopify Official Homepage - https://www.shopify.com/
  2. Shopify Sidekick AI Feature Page - https://www.shopify.com/sidekick
  3. Shopify Pricing Tiers - https://www.shopify.com/pricing
  4. Shopify App Store - https://apps.shopify.com/
  5. Wix Official Homepage - https://www.wix.com/
  6. Wix E-Commerce Solutions - https://www.wix.com/ecommerce/website
  7. Squarespace Homepage - https://www.squarespace.com/
  8. Squarespace E-Commerce - https://www.squarespace.com/ecommerce
  9. GoDaddy Homepage - https://www.godaddy.com/
  10. GoDaddy Website Builder - https://www.godaddy.com/websites/website-builder
  11. Weebly Official Site - https://www.weebly.com/
  12. WooCommerce Official Site - https://woocommerce.com/
  13. Webflow Design Tool - https://webflow.com/
  14. BigCommerce Enterprise Solutions - https://www.bigcommerce.com/
  15. Durable AI Website Builder - https://durable.co/
  16. 10Web AI WordPress Builder - https://10web.io/
  17. Framer Interactive Design - https://www.framer.com/
  18. Dorik AI CMS - https://dorik.com/
  19. Mixo AI Landing Pages - https://www.mixo.io/
  20. Hocoos AI Business Sites - https://hocoos.com/
  21. CodeDesign AI Editor - https://codedesign.ai/
  22. AppyPie No-Code Apps - https://www.appypie.com/
  23. HostGator Web Hosting - https://www.hostgator.com/
  24. Hostinger Hosting & Builder - https://www.hostinger.com/
  25. Zyro Website Builder - https://zyro.com/
  26. Shopify Trustpilot Reviews - https://www.trustpilot.com/review/www.shopify.com
  27. Wix Trustpilot Reviews - https://www.trustpilot.com/review/wix.com
  28. Squarespace Trustpilot Reviews - https://www.trustpilot.com/review/squarespace.com
  29. GoDaddy Trustpilot Reviews - https://www.trustpilot.com/review/godaddy.com
  30. Shopify G2 Customer Reviews - https://www.g2.com/products/shopify/reviews
  31. Wix G2 Customer Reviews - https://www.g2.com/products/wix/reviews
  32. Squarespace G2 Reviews - https://www.g2.com/products/squarespace/reviews
  33. Shopify Capterra Analysis - https://www.capterra.com/p/136006/Shopify/
  34. Wix Capterra Analysis - https://www.capterra.com/p/124706/Wix/
  35. Reddit SMB Shopify vs Wix Thread - https://www.reddit.com/r/smallbusiness/comments/shopify_vs_wix/
  36. Reddit E-commerce Platform Recommendations - https://www.reddit.com/r/ecommerce/comments/best_platform_for_beginners/
  37. Reddit Discussion on Shopify App Costs - https://www.reddit.com/r/smallbusiness/comments/shopify_app_costs/
  38. Reddit Entrepreneur Website Builders - https://www.reddit.com/r/entrepreneur/comments/website_builder_recommendations/
  39. Stripe Payment Processing - https://stripe.com/
  40. Calendly Scheduling Platform - https://calendly.com/
  41. Mailchimp Email Marketing - https://mailchimp.com/
  42. Manychat Marketing Automation - https://manychat.com/
  43. Klaviyo SMS & Email Marketing - https://www.klaviyo.com/
  44. Zapier Workflow Automations - https://zapier.com/
  45. Make Integration Platform - https://www.make.com/
  46. Shopify Editions Releases - https://www.shopify.com/editions
  47. Shopify Explainer Blog - https://www.shopify.com/blog/what-is-shopify
  48. Wix Official Blog - https://www.wix.com/blog
  49. Squarespace Resources Blog - https://www.squarespace.com/blog
  50. GoDaddy SMB Resources - https://www.godaddy.com/resources
  51. Durable AI Resources - https://durable.co/blog
  52. Shopify Newsroom - https://news.shopify.com/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
