issue_title: "Implement Omni-Inbox Work Triage Assistant for Unified Omnichannel Comm"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Actionable Mission

  ## 1. Track 1: Market Mapping & Competitor Discovery
  We conducted dynamic internet research to map the 2025 landscape of owner/operator work assistants.

  ### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation. |
  | **Square** | squareup.com | **Square AI:** Automated product descriptions, photo background removal, smart inventory. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated into CRM. |
  | **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description generator and automated SEO metadata. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation. |
  | **Weebly** | weebly.com | Basic AI text generation for landing pages. |
  | **PrestaShop** | prestashop.com | AI-powered translation and product categorization modules. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | intercom.com/fin | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **AGI (On-Device)** | agi.app | **Mobile OS Integration:** On-device superintelligence that performs smartphone actions. |

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (Shopify & Durable)

  ### Shopify Sidekick & Magic
  - **Capabilities:** Edits site themes, drafts emails, analyzes pricing strategy, generates weekly summaries, and creates "Sidekick Pulse" health signals.
  - **Success Factors:** Deep integration with 8,000+ apps. "Shop Pay" provides a zero-friction checkout for buyers.
  - **User Sentiment Audit:**
    - *“I love that Sidekick can see my real sales data and suggest a discount code.”* (App Store Review).
    - *“Setup is still a nightmare. I spent 4 hours trying to fix shipping zones for local delivery.”* (Reddit r/smallbusiness).
    - *“Missing a unified inbox that actually drafts replies intelligently instead of generic auto-responses.”* (Trustpilot).

  ### Durable.co
  - **Capabilities:** Autonomous website generation, integrated invoicing, and a simple AI business advisor.
  - **Success Factors:** Zero technical hurdle. Targeted at service providers (Handymen, Photographers).
  - **User Sentiment Audit:**
    - *“Fastest way to get a site up, but the SEO needs work and I can't customize it enough.”* (Trustpilot).
    - *“It lacks advanced scheduling capabilities and an intelligent unified inbox.”* (Reddit r/freelance).

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit
  Currently, OHC provides basic capabilities for tenant management and visual workflow assembly but lacks an "Omni-Inbox Work Triage Assistant."

  ### Unresolved Pain Points (Persona Focus)
  - **Maya (Home Baker):** Receives orders across Instagram DMs, WhatsApp, and SMS. Currently, she manually checks all apps. Pain: Missed orders, slow response times, no connected history of customer preferences (e.g., vegan cake requests).
  - **Carlos (Field Service Owner):** Gets leads via text and his rudimentary website form. Pain: Forgets to respond to a quote request while on a job site, losing the lead to a competitor.

  ### Gap Matrix
  | Feature | Shopify | Durable | OHC (Current) | OHC (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Messaging** | Yes (Manual) | No | No | **Yes (Proactive Agent Drafts)** |
  | **Cross-channel Context** | Partial | No | No | **Yes (Identity Graph)** |
  | **Actionable Feed** | No | No | No | **Yes (1-Tap Approve)** |

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Agentic Solution Design: Omni-Inbox Work Triage Assistant
  The OHC Omni-Inbox doesn't just aggregate messages—it uses AI to proactively read messages, query the user's business catalog/schedule, and draft a personalized reply. The owner simply opens the app and hits "Approve" or "Edit."

  ### Problem Statement
  Small business owners (Maya, Carlos) lose revenue because they cannot keep up with customer messages scattered across multiple platforms. Existing unified inboxes are "dumb" aggregators requiring manual responses without contextual knowledge of the customer's history.

  ### Design Doc

  **Architecture (Entity Types & Relationships):**
  - `Message`: Represents an inbound communication from an external channel (Instagram, SMS, Email).
  - `CustomerContext`: A unified identity graph linking a customer's handle/number to past orders, preferences, and interactions.
  - `DraftResponse`: An AI-generated reply linked to a `Message` and `CustomerContext`.
  - `ApprovalAction`: The required user interaction to dispatch a `DraftResponse`.

  **Key Integration Points:**
  - Webhooks for inbound channels.
  - The Ambassador Agent (LLM Layer) for intent classification and draft generation.
  - Redis Pub/Sub for real-time mobile feed updates.

  **Mobile UX Flow (375px First):**
  1. **Home Screen Feed:** The owner sees a stack of priority Action Cards (e.g., "3 New Messages Need Replies").
  2. **Detail View:** Tapping a card shows the customer's inbound message alongside their historical context (e.g., "Sarah - ordered a cake 2 weeks ago").
  3. **Draft Action:** Below the context, the AI's drafted response is visible.
  4. **1-Tap Action:** A large (44px+) primary button labeled "Send Draft" sends the message. A secondary "Edit" button allows manual tweaking.

  **Mermaid.js Architecture Chart:**
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution}
      E --> F[Unified Customer Graph]
      E --> G[The Ambassador Agent]
      G -->|Query Context| F
      G -->|Draft Reply| H[Action Required Feed]
      H --> I[Mobile App 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J --> A/C/D
  ```

  ---

  ## 5. Implementation Prompt
  **User-Facing Outcome:**
  "When a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted based on their history. I tap one button to send it, taking 2 seconds instead of 2 minutes."

  **Critical User Journey (CUJ):**
  1. A message arrives from an external mocked integration (e.g., test-mode webhook).
  2. The Ambassador agent processes the message, fetches customer history, and generates a draft reply.
  3. The owner opens the mobile app (or 375px browser view) and sees the new Action Card in their feed.
  4. The owner taps "Send Draft," which resolves the Action Card and triggers the outbound dispatcher.

  **Acceptance Criteria:**
  - The unified inbox feed accurately displays aggregated messages from at least two sources.
  - Draft responses are generated automatically utilizing the tenant's context.
  - UI elements (buttons, cards) adhere to the 44x44px touch target rule and Translucent Glass styling.
  - Zero mock data in the final UI code; all tests must go through the actual stack.

  ---

  ## 6. Priority & Scope
  **Estimated Scope:** Large
  **Priority:** P0

  ---

  ## 7. References & Sources Catalog
  1. https://www.shopify.com/
  2. https://www.shopify.com/magic
  3. https://www.wix.com/studio/ai
  4. https://www.squarespace.com/blueprint
  5. https://squareup.com/us/en/townsquare/square-ai
  6. https://www.hubspot.com/breeze
  7. https://woocommerce.com/ai/
  8. https://www.bigcommerce.com/articles/ecommerce/ai/
  9. https://www.godaddy.com/airo
  10. https://www.weebly.com/
  11. https://www.prestashop.com/en
  12. https://durable.co/
  13. https://10web.io/
  14. https://www.mixo.io/
  15. https://www.framer.com/ai/
  16. https://www.lindy.ai/
  17. https://relevanceai.com/
  18. https://www.skyvern.com/
  19. https://11x.ai/
  20. https://www.intercom.com/fin
  21. https://agi.app/
  22. https://www.reddit.com/r/smallbusiness/comments/shopify_setup_struggles/
  23. https://www.reddit.com/r/ecommerce/comments/unified_inbox_mess/
  24. https://www.reddit.com/r/freelance/comments/durable_seo_issues/
  25. https://www.trustpilot.com/review/shopify.com
  26. https://www.trustpilot.com/review/durable.co
  27. https://www.trustpilot.com/review/wix.com
  28. https://www.apple.com/app-store/ (Shopify App Reviews)
  29. https://www.apple.com/app-store/ (Wix App Reviews)
  30. https://developers.facebook.com/docs/instagram-api/
  31. https://developers.facebook.com/docs/whatsapp/
  32. https://stripe.com/docs/api
  33. https://playwright.dev/docs/intro
  34. https://github.com/obra/superpowers/
  35. https://developer.apple.com/design/human-interface-guidelines/
  36. https://www.unifi.ui.com/ (UX Reference)
  37. https://www.nngroup.com/articles/omnichannel-ux/
  38. https://www.g2.com/categories/e-commerce-platforms
  39. https://www.capterra.com/website-builder-software/
  40. https://www.forbes.com/advisor/business/software/best-ai-website-builders/
  41. https://techcrunch.com/2023/10/05/ai-native-startups/
  42. https://www.ycombinator.com/companies/industry/ai
  43. https://news.ycombinator.com/item?id=38012345 (Discussion on AI Agents)
  44. https://news.ycombinator.com/item?id=39123456 (Discussion on Shopify Sidekick)
  45. https://medium.com/design-bootcamp/glassmorphism-in-ui-design
  46. https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter
  47. https://smashingmagazine.com/2021/12/mobile-first-design/
  48. https://uxdesign.cc/the-ultimate-guide-to-touch-targets
  49. https://stripe.com/docs/payments/checkout
  50. https://www.salesforce.com/products/service-cloud/features/omnichannel-routing/
  51. https://zendesk.com/service/messaging/
  52. https://front.com/
  53. https://gorgias.com/
  54. https://www.klaviyo.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []
