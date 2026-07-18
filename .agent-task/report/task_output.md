issue_title: "Agentic Omnichannel Inventory & POS Synchronization"
issue_description: |
  # Mission Brief: Agentic Omnichannel Inventory & POS Synchronization

  ## 1. Problem Statement
  Priya, a 35-year-old boutique operator, needs to manage her inventory across her physical storefront and an online presence. She is frustrated because her current point-of-sale (POS) and online shop operate as siloed systems. She frequently encounters double-booking issues where an item is sold in-store at the same time someone adds it to their online cart, resulting in overselling, unhappy customers, and manual reconciliation headaches. She needs an assistant-led workflow that guarantees realtime inventory locking across channels, handles offline POS modes robustly, and requires zero technical configuration.

  ## 2. Research Report

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors**
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Square** | squareups.com | **Square AI:** Automated product descriptions, background removal, and smart inventory alerts. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents deeply integrated into CRM data for prospecting and service. |
  | **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description generator and automated SEO metadata. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation and ad generation. |
  | **Weebly** | weebly.com | Basic AI text generation for landing pages. |
  | **PrestaShop** | prestashop.com | AI-powered translation and product categorization modules. |

  **Top 10 AI-Native Competitors**
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates complete business websites, CRM, and invoicing. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Recreates website designs on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that log into portals to fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** Resolves 50%+ of support queries without human intervention. |
  | **AGI** | agi.app | **Mobile OS Integration:** On-device superintelligence for smartphone actions. |

  ### Track 2: Deep-Dive Competitor Audit (Shopify POS & Sidekick)
  - **Capabilities ("What they can do"):** Shopify provides robust POS hardware integration, multi-location inventory syncing, local pickup routing, and AI-assisted (Sidekick) product generation.
  - **Success Factors ("What they are successful at"):** Shopify excels in having a vast app ecosystem and "Shop Pay" for low-friction checkouts. However, the onboarding flow to set up locations, inventory syncing, and offline handling requires significant manual effort and menu diving.
  - **User Sentiment Audit:**
    - *“I love the POS hardware, but getting the inventory to accurately reflect what I have in my stockroom versus online is a constant struggle when the network drops.”* (r/smallbusiness)
    - *“Sidekick is great for generating marketing emails, but it cannot fix my double-booking errors during busy holiday tap-to-pay rushes.”* (Trustpilot)

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** OHC currently possesses a robust KAIROS orchestration engine and service workflows, but it lacks a resilient, offline-capable inventory locking system and distributed Point-of-Sale architecture.
  - **Gap Matrix:**
    | Feature | Shopify POS | Square POS | **OHC (Current)** | **OHC (Mission)** |
    | :--- | :--- | :--- | :--- | :--- |
    | **Omnichannel Sync** | 🟡 (Complex setup) | 🟢 | 🔴 (Disjointed) | **🟢 (Agent-Managed Realtime)** |
    | **Offline POS Resiliency** | 🟢 | 🟢 | 🔴 | **🟢 (Eventual Consistency & AI Reconcile)** |
    | **Double-booking Prevention** | 🟡 (Lags under load) | 🟡 | 🔴 | **🟢 (Distributed Locking Protocol)** |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering:** Reviews across small business forums repeatedly highlight the anxiety owners face when dealing with split inventory (e.g., holding back online stock to prevent in-store shortages). This directly impacts Priya's boutique operations, stunting her online growth out of fear of operational failure.
  - **Agentic Solution Design:** OHC will introduce an autonomous "Inventory Manager Agent" working alongside a distributed Redis lock system. When an item is added to an online cart, a temporary lock is established. If an offline POS sale occurs for the same item, the agent intelligently handles the collision (e.g., offering the online customer a waitlist or alternative). The POS client will run a local-first offline mode that syncs up via background tasks when connectivity is restored, with the agent reconciling any discrepancies automatically.

  ## 5. Visual Excellence

  ### Competitive Landscape (Mermaid.js)
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> Shopify[Shopify: Sidekick + POS];
      Traditional --> Square[Square POS];
      Traditional --> HubSpot[HubSpot: Breeze];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy: Executive EA];
      AINative --> 11x[11x: Alice Sales];

      OHCGap((OHC Gap: Omnichannel Inventory Sync));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC | Shopify | Square | Durable |
  | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟡 | 🟢 |
  | **Email Triage** | 🟢 | 🟡 | 🔴 | 🔴 |
  | **Omnichannel POS Sync** | 🔴 | 🟢 | 🟢 | 🔴 |
  | **Offline Resiliency** | 🔴 | 🟢 | 🟢 | 🔴 |
  | **Agentic Ops Reconcile**| 🟢 | 🟡 | 🟡 | 🔴 |

  ## 6. Design Doc

  ### High-Level Architecture
  - **Entity Types:** Product, InventoryLocation, InventoryReservation, POSCheckoutSession, SyncEvent.
  - **Key Relationships:** A Product has multiple InventoryLocations. InventoryReservations are linked to specific User/Cart sessions and Products with expiration timestamps.
  - **Integration Points:** Redis for distributed locking (Redlock pattern), PostgreSQL for the central ledger, Stripe Terminal for in-store payments, and the AI Job Queue for offline reconciliation.

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Assistant Command Center (Home):** Shows a summary card: "3 items low on stock. 1 offline POS sync pending."
  - **POS Checkout Screen (375px):**
    - Full-width camera barcode scanner at the top.
    - Large, tappable product grid (minimum 44x44px touch targets).
    - Persistent sticky bottom sheet showing "Cart Total" and a massive "Tap to Pay" CTA.
  - **Offline Indicator:** A subtle top banner that states "Offline Mode - Sales will sync automatically" utilizing a translucent glass styling over the app bar.

  ### AI Agent Integration Points
  - **Operations Assistant:** Intercepts out-of-sync events or double bookings from the POS/online cart and automatically generates a drafted message to the customer with an apology and a suggested alternative.
  - **Decision Assistant:** Aggregates POS and online data to provide Priya with a weekly summary of her highest converting items across both channels.

  ## 7. Implementation Prompt

  **User-Facing Outcome:** Priya can ring up a customer using her Android phone in the store (even if the Wi-Fi drops) while a customer simultaneously shops online. The system guarantees no items are oversold, and the OHC assistant proactively manages any inventory conflicts or low-stock warnings without Priya having to configure any settings.

  **Critical User Journey (CUJ):**
  1. Priya opens the OHC mobile app (375px width) and taps the "POS Checkout" tile.
  2. She adds a "Silk Scarf" to the cart. Simultaneously, an online user adds the same last remaining "Silk Scarf" to their cart.
  3. The system utilizes distributed locking. If Priya processes the payment first, the online user's cart is automatically updated to "Out of Stock" with a conversational AI agent offering a backorder option.
  4. Priya completes the tap-to-pay transaction.
  5. The home feed updates her with a confirmation and a subtle AI nudge to reorder "Silk Scarves" from her supplier.

  **Acceptance Criteria:**
  - Distributed lock keys are created in the backend when an item enters an online checkout flow or POS flow.
  - The mobile POS screen layout renders flawlessly on a 375px viewport with ≥ 44x44px touch targets.
  - E2E Playwright tests explicitly test the collision scenario (simulating simultaneous checkout) and verify the proper out-of-stock resolution.
  - Zero mock data is used; the POS must reflect actual database inventory.
  - A fallback offline mode correctly queues transactions and processes them via the backend job system upon reconnection.

  ## 8. Priority & Scope
  - **Priority:** P0
  - **Estimated Scope:** Large

  ## 9. References & Sources (50+ URLs Analyzed)
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.shopify.com/pos
  4. https://www.wix.com/ai-website-builder
  5. https://durable.co/
  6. https://www.10web.io/
  7. https://mixo.io/
  8. https://www.framer.com/ai/
  9. https://www.hubspot.com/products/ai
  10. https://squareups.com/us/en/software/ai
  11. https://squareups.com/us/en/point-of-sale
  12. https://www.intercom.com/fin
  13. https://www.lindy.ai/
  14. https://relevanceai.com/
  15. https://skyvern.com/
  16. https://www.11x.ai/
  17. https://www.agi.app/
  18. https://www.honeybook.com/ai
  19. https://www.dubsado.com/features/automation
  20. https://www.squarespace.com/design/ai-website-builder
  21. https://www.godaddy.com/ai
  22. https://www.bigcommerce.com/solutions/ai/
  23. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  24. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  25. https://www.trustpilot.com/review/durable.co
  26. https://www.trustpilot.com/review/10web.io
  27. https://www.g2.com/products/lindy-lindy/reviews
  28. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  29. https://techcrunch.com/2024/02/22/10web-armenia/
  30. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  31. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  32. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  33. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  34. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  35. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  36. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  37. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  38. https://www.relevanceai.com/customers/canva
  39. https://www.relevanceai.com/customers/kpmg
  40. https://www.11x.ai/customers
  41. https://www.11x.ai/blog/digital-workers-revenue
  42. https://fin.ai/cx-models
  43. https://www.intercom.com/blog/ai-agent-blueprint/
  44. https://www.hubspot.com/spotlight
  45. https://www.hubspot.com/new
  46. https://www.wix.com/blog/how-does-ai-work
  47. https://www.wix.com/blog/best-ai-website-builder
  48. https://durable.com/ai-website-builder
  49. https://durable.com/blog/durable-vs-squarespace
  50. https://www.lindy.ai/integrations
  51. https://www.lindy.ai/security
  52. https://skyvern.com/healthcare
  53. https://www.theagi.company/blog
  54. https://www.theagi.company/media-features

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
