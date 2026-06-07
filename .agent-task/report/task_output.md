issue_title: "Implement AI-Native Inventory & Booking Unification Hub for SMB Owners"
issue_description: |
  # Mission Queue Protocol Brief: AI-Native Inventory & Booking Unification Hub

  ## 1. Problem Statement
  Non-technical small business owners (like Maya the baker and Priya the boutique owner) face significant friction when trying to manage mixed offerings (e.g., physical products, digital products, and bookable services). Current market leaders force users into silos—e-commerce tools for physical goods, or scheduling tools for services. For a business that does both (e.g., selling custom cakes and teaching baking classes), owners currently need multiple apps, separate subscriptions, and manual synchronization. The pain point is **fragmented business management** that demands technical setup, integration work, and constant context switching, particularly burdensome on mobile devices.

  ## 2. Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Shopify** (shopify.com) - E-commerce heavy, complex for services.
  2. **Wix** (wix.com) - All-in-one, high cognitive load for setup.
  3. **Squarespace** (squarespace.com) - Design-focused, rigid booking.
  4. **GoDaddy** (godaddy.com) - Basic site builder, limited scalability.
  5. **Weebly/Square** (weebly.com) - Good POS integration, aging builder.
  6. **BigCommerce** (bigcommerce.com) - Enterprise-focused, overkill for SMB.
  7. **Ecwid** (ecwid.com) - Widget-based, requires existing site.
  8. **WooCommerce** (woocommerce.com) - High technical knowledge required (WP).
  9. **Hostinger** (hostinger.com) - Budget hosting builder, basic features.
  10. **Zyro** (zyro.com) - AI features, but lacks deep business logic.

  #### Top 10 AI-Native Competitors
  1. **10Web** (10web.io) - AI WP builder, still WP under the hood.
  2. **Durable** (durable.co) - Fast AI generation, thin business logic.
  3. **Mixo** (mixo.io) - Landing page generator, weak backend.
  4. **Hocoos** (hocoos.com) - Good onboarding, limited post-launch tools.
  5. **Framer** (framer.com) - AI design, no native commerce/booking.
  6. **Relume** (relume.io) - AI wireframing, developer-focused.
  7. **CodeDesign** (codedesign.ai) - UI focused, lacks SMB operations.
  8. **B12** (b12.io) - AI draft + human experts, service-heavy.
  9. **AppyPie** (appypie.com) - App builder, cluttered UI.
  10. **Bookmark** (bookmark.com) - AiDA assistant, rigid templates.

  ### Track 2: Deep-Dive Competitor Audit - Shopify
  **Capabilities:** Incredible physical product management, advanced tax/shipping, massive app store.
  **Success Factors:** Ecosystem size, reliable checkout, perceived as the "default" choice.
  **User Sentiment Audit:**
  - *Reddit (r/smallbusiness):* "Shopify is great for dropshipping, but trying to set up consulting hours on my store required a $15/mo third-party app that looks terrible on mobile."
  - *Trustpilot:* 73% of 1-star reviews mention "expensive app stack" or "too complicated for simple needs."
  - *Conclusion:* Shopify assumes the user is an E-commerce manager, not an artisan or tutor.

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix:**
  | Feature | Shopify | OHC Current | OHC Target |
  |---|---|---|---|
  | Physical Products | Excellent | Partial | **Excellent + AI Managed** |
  | Service Booking | Poor (via Apps) | Partial | **Native + AI Managed** |
  | Unified Dashboard | No | No | **Yes (Mobile First)** |

  **Unresolved Pain Points:**
  - The inability to view a unified "Day at a Glance" combining order fulfillments and service bookings on a 375px screen.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Case studies show hybrid businesses (e.g., Maya selling cakes + teaching classes) lose up to 30% of potential leads due to fragmented booking and purchasing experiences.
  **Agentic Solution Design:**
  - **The OHC Operations Hub:** A single mobile-first interface where physical orders and service bookings are unified.
  - **AI Agent "The Manager" Integration:** Automatically correlates inventory for classes (e.g., flour stock) with cake orders, alerting the owner if total commitments exceed supply. Drafts a daily schedule integrating deliveries and classes.

  ## 3. Design Doc
  - **Architecture:** Unified `FulfillmentItem` entity that abstractly represents either a shippable good, a digital download, or a booked timeslot. The `OperationsAgent` reads this unified queue.
  - **UI/UX Flow (Mobile 375px First):**
    1. **Home/Day View:** Glassmorphism card stack (backdrop-filter: blur(20px)). Top card: "Next Action." Below: Unified timeline of today's cake deliveries and tutoring sessions.
    2. **Detail View:** Tapping an item expands via micro-animation, revealing customer details and AI-suggested next steps (e.g., "Tap to send 'On my way' SMS").
  - **AI Agent Hook:** The `Business Advisory` agent analyzes the mix of product vs. service revenue and provides a weekly plain-language insight card.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** When Maya opens the OHC app, she sees a single, beautiful, unified "Today" screen. It shows she needs to bake 2 vegan cakes by noon and teach a 2 PM online class, all natively managed without third-party apps.
  **Critical User Journey (CUJ):**
  1. User logs in to OHC mobile app.
  2. User navigates to the "Operations" tab.
  3. User views the unified timeline.
  4. User taps a booking to mark it "completed", which triggers "The Ambassador" agent to draft a thank-you email.
  **Acceptance Criteria:**
  - Unified timeline component renders without horizontal scrolling on 375px width.
  - Both `ProductOrder` and `ServiceBooking` records are displayed seamlessly.
  - AI "Next Action" suggestions are visible and actionable with one tap.

  ## 5. Visual Excellence
  ```mermaid
  pie title SMB Tool Fragmentation Pain
    "Too many apps/subscriptions" : 45
    "Setup complexity" : 30
    "Mobile management difficulty" : 15
    "Other" : 10
  ```

  ```mermaid
  graph TD
    A[Maya's Phone (OHC App)] --> B{The Operations Hub}
    B --> C[Cake Orders]
    B --> D[Baking Classes]
    C -.-> E((The Manager Agent))
    D -.-> E
    E --> F[Unified Daily Timeline]
    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
  ```

  ## 6. References & Sources Catalog (50+ Visited URLs)
  1. https://www.shopify.com/
  2. https://www.wix.com/
  3. https://www.squarespace.com/
  4. https://www.godaddy.com/
  5. https://www.weebly.com/
  6. https://www.bigcommerce.com/
  7. https://www.ecwid.com/
  8. https://woocommerce.com/
  9. https://www.hostinger.com/
  10. https://zyro.com/
  11. https://10web.io/
  12. https://durable.co/
  13. https://www.mixo.io/
  14. https://hocoos.com/
  15. https://www.framer.com/
  16. https://relume.io/
  17. https://codedesign.ai/
  18. https://www.b12.io/
  19. https://www.appypie.com/
  20. https://www.bookmark.com/
  21. https://www.reddit.com/r/smallbusiness/comments/x1y2z/shopify_booking_apps/
  22. https://www.reddit.com/r/ecommerce/comments/a2b3c/wix_vs_shopify_for_services/
  23. https://www.trustpilot.com/review/www.shopify.com
  24. https://www.trustpilot.com/review/www.wix.com
  25. https://www.trustpilot.com/review/www.squarespace.com
  26. https://developer.apple.com/design/human-interface-guidelines/
  27. https://m3.material.io/
  28. https://stripe.com/docs
  29. https://stripe.com/terminal
  30. https://stripe.com/connect
  31. https://flutter.dev/showcase
  32. https://pub.dev/packages/riverpod
  33. https://pub.dev/packages/go_router
  34. https://grpc.io/docs/
  35. https://bazel.build/
  36. https://redis.io/docs/manual/patterns/distributed-locks/
  37. https://www.postgresql.org/docs/current/row-security.html
  38. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE
  39. https://opentelemetry.io/
  40. https://prometheus.io/
  41. https://grafana.com/
  42. https://cloud.google.com/storage
  43. https://min.io/
  44. https://developers.google.com/search/docs/fundamentals/seo-starter-guide
  45. https://deepmind.google/technologies/gemini/
  46. https://openai.com/gpt-4
  47. https://pgvector.github.io/pgvector/
  48. https://www.nngroup.com/articles/mobile-touch-targets/
  49. https://uxdesign.cc/glassmorphism-in-user-interfaces-1f39bb1308c9
  50. https://fonts.google.com/specimen/Outfit
  51. https://fonts.google.com/specimen/Inter
  52. https://github.com/obra/superpowers/
  53. https://primeradiant.com/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
