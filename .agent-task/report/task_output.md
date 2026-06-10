issue_title: "Unified AI Work Triage & Agentic Feed for SMB Onboarding"
issue_description: |
  # OHC Research Report: AI Agentic Workflows & The Invisible Assistant

  ## Executive Summary
  This report investigates the current landscape of AI-powered website builders, e-commerce platforms, and work assistants to identify unresolved pain points for small business owners. Our analysis reveals that while platforms like Shopify and Wix provide powerful tools, they suffer from high setup complexity and rely on reactive chat-based advice. OHC's key differentiator is "Invisible AI Automation"—proactive agents that act as functional departments (Sales, Marketing, Operations) that execute tasks autonomously, requiring only simple user approval.

  ---

  ## 1. Market Mapping & Competitor Discovery (Track 1)

  ### Top 10 General Competitors
  1. **Shopify**: Dominant e-commerce player with an extensive app ecosystem, but a highly complex setup process.
  2. **Wix**: Popular drag-and-drop visual builder, though e-commerce features can feel disjointed.
  3. **Squarespace**: Design-centric builder optimized for creatives.
  4. **GoDaddy**: Fast and simple initial setup but extremely limited customization.
  5. **Weebly / Square Online**: Simple POS-integrated builder.
  6. **Hostinger**: Low-cost hosting with basic builder tools.
  7. **Zyro**: Budget-friendly website builder with limited operational depth.
  8. **Webflow**: Incredible design power, but with a steep learning curve.
  9. **WordPress.com**: Ultimate flexibility but requires significant technical knowledge.
  10. **BigCommerce**: Powerful but targets mid-market and enterprise rather than micro-SMEs.

  ### Top 10 AI-Native & Emerging Competitors
  1. **Durable**: AI website generation in 30 seconds.
  2. **10Web**: AI WordPress builder.
  3. **Framer AI**: AI design generation, heavily focused on aesthetics.
  4. **CodeDesign.ai**: AI-powered drag-and-drop builder.
  5. **Mixo**: AI landing page generator for rapid idea validation.
  6. **Hocoos**: AI business website builder utilizing an 8-question setup.
  7. **Relume**: AI-powered sitemap and wireframe generation.
  8. **Pineapple Builder**: AI builder designed for busy founders.
  9. **Appy Pie AI**: AI app and website generator.
  10. **Jimdo AI**: Automated website creation tailored specifically to small businesses.

  ---

  ## 2. Deep-Dive Competitor Audit: Shopify & Sidekick (Track 2)

  ### Capabilities ("What they can do")
  Shopify offers a massive ecosystem of over 21,000 apps, robust checkout (Shop Pay), and multi-channel selling capabilities. Shopify Sidekick acts as an AI commerce assistant, primarily functioning as an interactive chatbot to navigate the admin panel, suggest content, and perform simple bulk edits.

  ### Success Factors ("What they are successful at")
  - **Ecosystem**: Unparalleled third-party app integration.
  - **Checkout**: Shop Pay provides an industry-leading, frictionless checkout experience.
  - **Reliability**: Seamless handling of massive traffic spikes.

  ### User Sentiment Audit (Synthesized from Reddit & Trustpilot)
  - **The "App Tax"**: Users frequently complain that the base Shopify plan is insufficient without subscribing to expensive third-party apps for essential features like bookings or reviews.
  - **Setup Paralysis**: Non-technical users struggle significantly with initial configuration (e.g., shipping zones, domain setup). "The setup process is overwhelming. Too many menus and settings before I can even see my store."
  - **Advice vs. Action**: Current AI tools like Sidekick are seen as glorified manuals. They tell the user *how* to do things rather than proactively executing tasks for them.

  ---

  ## 3. OHC Gap & Pain Point Identification (Track 3)

  ### OHC Feature Audit vs Shopify

  | Feature | OHC (Vision) | Shopify | Gap to Close |
  | :--- | :--- | :--- | :--- |
  | Mobile-first Setup | Yes (< 10 mins via Agent) | No (Desktop preferred) | OHC needs fully native, conversational mobile onboarding. |
  | AI-Native Execution | Yes (Agents *do* the work) | Partial (Chatbots *advise*) | OHC must automate tasks (e.g., modifying inventory, sending emails), not just advise. |
  | All-in-one Pricing | Yes | No (App fees add up) | OHC must bundle bookings and commerce natively. |

  ### Unresolved User Pain Points
  1. **The "Now What?" Syndrome**: Users launch a site but have zero traffic due to a lack of marketing/SEO knowledge.
  2. **Instagram DM Overload**: Owners spend hours manually replying to repetitive questions on social media.
  3. **Fragmented Operations**: Juggling separate tools for websites, bookings, payments, and marketing creates overwhelming friction.

  ---

  ## 4. Deeper Focused Research & Agentic Solutions (Track 4)

  ### Persona Pain Points & Agentic Solutions

  #### Persona: Maya (Home Baker)
  **Pain Point:** Setup Paralysis. She wants to sell custom cakes, not configure DNS settings or juggle multiple app subscriptions for booking and deposits.
  **Agentic Mission: "Zero-Click Onboarding Agent"**
  - **Outcome:** Maya chats with OHC for 5 minutes. The AI provisions her domain, configures custom deposits via Stripe, and creates her first product from an uploaded photo.
  - **Acceptance Criteria:** A user goes from login to a published product link using only natural language in a mobile-first (375px) environment.

  #### Persona: Carlos (Field Service / Handyman)
  **Pain Point:** Missed Leads. He loses business because he is on the job and cannot answer calls or manually quote prices.
  **Agentic Mission: "Agentic Negotiator & Booker"**
  - **Outcome:** An AI agent intercepts incoming calls/DMs, checks his calendar, quotes a price based on the described project, and collects a deposit autonomously.
  - **Acceptance Criteria:** The agent successfully books a meeting and secures payment without owner intervention.

  #### Persona: Fatima (Food Cart Operator)
  **Pain Point:** Language Barriers. Struggles with English-speaking customers on the phone during busy hours.
  **Agentic Mission: "Multilingual Order Interceptor"**
  - **Outcome:** The agent handles phone orders in English and translates them into Fatima's native language on her Kitchen Display System (KDS).
  - **Acceptance Criteria:** Real-time translation of voice-to-text orders with high accuracy.

  ---

  ## 5. Visual Excellence

  ### Competitive Landscape (Mermaid.js)
  ```mermaid
  quadrantChart
      title SMB Platform Landscape: Complexity vs AI Integration
      x-axis "Manual Configuration / Reactive" --> "Autonomous Execution"
      y-axis "Complex / Enterprise" --> "Simple / Mobile-First"
      quadrant-1 "Ideal Future (OHC)"
      quadrant-2 "AI Toy Builders"
      quadrant-3 "Traditional Monoliths"
      quadrant-4 "Complex Integrators"
      "Shopify": [0.3, 0.4]
      "Wix": [0.4, 0.6]
      "Durable": [0.8, 0.8]
      "OHC Target": [0.95, 0.95]
      "Squarespace": [0.3, 0.7]
  ```

  ### Setup Time Comparison
  ```mermaid
  journey
      title Setup Time Comparison: Traditional vs OHC
      section Traditional Setup (Shopify)
        Sign up & verify: 3: User
        Navigate complex settings: 1: User
        Install themes & apps: 2: User
        Add initial products manually: 1: User
      section OHC Agentic Flow
        Enter business idea: 5: User
        AI generates site, DB, and copy: 5: Agent
        Review and launch from phone: 5: User
  ```

  ---


  ### Design Doc
  - **High-Level Architecture**: Connects social APIs (Instagram Graph API) to a Gemini-powered intent classifier via a native integration layer in the `KAIROS` orchestration engine.
  - **Entity Types**: `Tenant` (Owner Workspace), `Customer`, `MessageThread`, `ProductCatalog`, `AgentDraft`.
  - **Key Relationships**: A `MessageThread` belongs to a `Customer` and a `Tenant`. An `AgentDraft` relies on a `ProductCatalog` query for context.
  - **Integration Points**: Meta Graph API, KAIROS Agentic Service, iOS/Android Push Notifications.
  - **UI Wireframes (375px)**: A simple list view "Agent Feed" with unread items. Tapping an item opens a "Draft Card" showing the customer context, the AI's proposed reply, and two prominent bottom-sheet buttons: [Approve & Send] and [Edit].

  ## 6. Implementation Prompt (For Engineering Swarm)
  **Feature Name:** The Ambassador - Native Social Inbox Auto-Responder
  **Target Persona:** Maya the Baker

  **Outcome:** An automated DM response system where the AI agent drafts replies based on inventory and business rules. Maya can review and approve them directly from her mobile device.

  **Critical User Journey (CUJ):**
  1. Maya logs into the OHC mobile web app (optimized for 375px).
  2. Maya connects her Instagram Business account via the Integrations tab.
  3. A customer DMs Maya: "Do you have vegan chocolate cake available for Saturday?"
  4. The Ambassador Agent queries Maya's inventory, confirms availability, and drafts: "Yes we do! We have 3 left for this Saturday. Would you like a booking link?"
  5. Maya receives a push notification: "Agent drafted a reply to @customer. Tap to review."
  6. Maya taps the notification, views the draft, and clicks "Approve". The message is sent.

  **Acceptance Criteria:**
  - UI must function flawlessly on a 375px viewport with no horizontal scroll.
  - Implement an automated E2E Playwright test verifying the end-to-end approval flow.
  - Do not require the user to configure complex rules; the LLM handles intent and context matching natively.

  ---


  **Estimated Scope**: Medium

  ## References & Sources (50+ Analyzed URLs)
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
  53. https://workbuddy.tencent.com
  54. https://wecom.qq.com
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
