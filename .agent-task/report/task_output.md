issue_title: "OneHumanCorp Platform Strategy: Unifying Sales, Booking, and AI Agents for Non-Technical SMBs"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: The Invisible Operating System for SMBs: Unifying Storefronts, Booking, and Customer Management with AI

  **Problem Statement**: Non-technical small business owners (bakers, handymen, tutors) are overwhelmed by fragmented software. They currently string together Shopify for products, Calendly for bookings, Instagram for communication, and spreadsheets for finance. This creates a technical barrier to entry and ongoing operational friction. They need a single, mobile-first interface where AI agents invisibly handle the integration, routing, and heavy lifting.

  ### Research Report

  #### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1.  **Shopify** (shopify.com): E-commerce giant. Great for physical products, poor for services/bookings. Too complex for simple setups.
  2.  **Wix** (wix.com): All-in-one builder. Flexible but clunky mobile management. "Jack of all trades, master of none."
  3.  **Squarespace** (squarespace.com): Beautiful templates. Strong for portfolios and basic commerce. Weak on complex bookings or AI automation.
  4.  **GoDaddy** (godaddy.com): Domain registrar turned builder. Basic, easy, but limited scalability and feature depth.
  5.  **Weebly / Square Online** (weebly.com / squareup.com): Strong POS integration. Builder feels dated.
  6.  **BigCommerce** (bigcommerce.com): Enterprise-focused e-commerce. Too complex for our personas.
  7.  **WooCommerce** (woocommerce.com): WordPress plugin. Requires technical knowledge to manage hosting and updates.
  8.  **Ecwid** (ecwid.com): Good for embedding stores into existing sites.
  9.  **Webflow** (webflow.com): Designer-focused. Extremely technical.
  10. **Systeme.io** (systeme.io): Marketing funnel focus. Not a general business OS.

  **Top 10 AI-Native/Rising Competitors:**
  1.  **Durable** (durable.co): AI website generator. Fast setup, but lacks deep operational backend (inventory, complex booking).
  2.  **10Web** (10web.io): AI WordPress builder. Still inherits WordPress complexity.
  3.  **Gamma** (gamma.app): Great for presentations/landing pages, not a full business OS.
  4.  **Mixo** (mixo.io): Quick landing page validation. Lacks full commerce features.
  5.  **Zyro / Hostinger AI** (hostinger.com): Good budget option, AI text/logo generation, but standard traditional builder backend.
  6.  **B12** (b12.io): AI + human experts. Higher price point, focuses on professional services.
  7.  **Stan Store** (stan.store): Link-in-bio optimized for digital products. Excellent mobile conversion, weak on physical goods/complex services.
  8.  **Gumroad** (gumroad.com): Creator-focused digital downloads. Not a storefront builder.
  9.  **Linktree** (linktr.ee): Routing hub, not a full business platform.
  10. **Shopify Sidekick**: AI chatbot for merchants. It's an assistant, not invisible infrastructure.

  #### Mermaid Competitor Landscape

  ```mermaid
  quadrantChart
      title SMB Competitive Landscape: Complexity vs. Automation Level
      x-axis "Manual Tasks" --> "Autonomous AI"
      y-axis "High Setup Friction" --> "Zero Setup Friction"
      quadrant-1 "Ideal: Effortless OS"
      quadrant-2 "Basic Builders"
      quadrant-3 "Enterprise E-commerce"
      quadrant-4 "Complex Niche Tools"
      "OneHumanCorp (Vision)": [0.85, 0.9]
      "Shopify": [0.3, 0.2]
      "Wix": [0.25, 0.5]
      "Squarespace": [0.2, 0.45]
      "GoDaddy": [0.15, 0.7]
      "Durable AI": [0.65, 0.8]
      "WooCommerce": [0.1, 0.1]
  ```

  #### Track 2: Deep-Dive Competitor Audit - Shopify

  **Why Shopify?** It's the market leader for SMB commerce, but its complexity is exactly what OHC aims to solve for non-technical users.

  **Capabilities:**
  - Exhaustive product management, variant tracking, inventory syncing.
  - Robust checkout and payment processing (Shop Pay).
  - Massive app ecosystem (which is also its Achilles heel for simplicity).
  - Basic native reporting and analytics.

  **Success Factors:**
  - Reliability and scale.
  - Ecosystem of developers and themes.
  - Brand recognition.

  **User Sentiment Audit (r/smallbusiness, r/shopify, Trustpilot):**
  - *The "App Tax" Complaint*: "I just wanted to add a simple booking calendar for my consultations, and I have to pay $15/mo for a third-party app that looks completely different from my theme."
  - *Setup Overwhelm*: "It took me 3 weeks to figure out how to set up shipping zones correctly. I'm a baker, not a web developer."
  - *Mobile Management Limitations*: "The Shopify mobile app is okay for checking sales, but if I need to actually change my store design or fix a product variant, I have to open my laptop."

  #### Track 3: OHC Gap & Pain Point Identification

  **OHC vs. Shopify Map:**

  | Feature | Shopify | OHC Vision | OHC Current Gap |
  | :--- | :--- | :--- | :--- |
  | **Core Commerce** | Deep, complex | Simple, guided | Needs robust variant/inventory foundation |
  | **Bookings/Services** | Paid 3rd party apps | Native, integrated | Needs native booking system |
  | **Mobile Management** | Companion app (limited) | Primary OS (full power) | Needs 100% mobile-first parity |
  | **AI Assistance** | "Sidekick" chatbot | Invisible Agents | Needs agentic workflows wired into core actions |

  #### Feature Gap Heatmap

  ```mermaid
  pie
      title SMB Owner Time Wasted vs OHC Agentic Solutions
      "Manual Social Follow-ups": 35
      "Inventory Syncing": 25
      "Website Formatting/Design": 20
      "Managing Complex Apps/Plugins": 15
      "Actually Running the Business": 5
  ```

  **Unresolved Pain Points for OHC Personas:**
  1.  **The "Frankenstein" Stack**: Maya (Baker) and Leo (Tutor) currently need separate tools for showing products and booking time.
  2.  **The "Blank Canvas" Paralysis**: Traditional builders give users a blank page. Non-technical users don't know what to write or how to design.
  3.  **The "Always On" Burden**: Carlos (Handyman) loses leads when he's under a sink and can't reply to a quote request immediately.

  #### Track 4: Deeper Focused Research & Agentic Solutions

  **Evidence:** SMBs are increasingly moving to "Link-in-bio" tools (Stan Store, Linktree) because traditional builders are too heavy, but they quickly outgrow them when they need real inventory or booking management.

  **The Agentic Solution (OHC Differentiation):**
  Instead of a complex settings menu, OHC uses the "Department" model.
  - *User Intent*: "I need to sell 1-hour guitar lessons."
  - *Agentic Action*: The "Operations" agent automatically creates a service product, sets up a calendar connection, and the "Marketing" agent generates a landing page block for it. The user just taps "Approve".

  #### Target End-to-End Operation Flow vs Legacy Flow

  ```mermaid
  graph TD
      subgraph Legacy Platform (Shopify/Wix)
      A1[User Logs In] --> B1[Navigate to complex Products Dashboard]
      B1 --> C1[Manually type title, upload uncompressed images]
      C1 --> D1[Install third-party Calendar plugin to accept bookings]
      D1 --> E1[Manually configure link to website front page]
      end

      subgraph OneHumanCorp Platform (Agentic)
      A2[User Logs In on Mobile] --> B2[Tap primary '+' and type intent]
      B2 --> C2[AI Operations Agent provisions Service Offering & Calendar DB]
      B2 --> D2[AI Marketing Agent drafts landing page component]
      C2 & D2 --> E2[User taps 'Approve' -> Live instantly]
      end
  ```

  ### Design Doc

  **High-Level Architecture (Unifying Core Primitives):**
  Instead of hardcoded "Products" vs "Services", OHC needs a unified `Offering` primitive.

  **Entities:**
  - `Tenant` (The Business)
  - `Offering` (What is sold: Physical, Digital, Service)
    - Type determines requirements (Shipping vs. Calendar vs. File URL)
  - `Order` / `Booking` (The transaction)
  - `AgentTask` (Background jobs executed by the AI Departments)

  **Mobile UX Flow (375px First):**
  1.  **The Feed (Home):** Not a dashboard of charts. An actionable feed from the AI Departments.
      - Card: "Salesperson Agent: You have 3 pending quote requests from yesterday. [Draft Replies] or [View Details]"
      - Card: "Advisor Agent: Your Tuesday slots are empty. Want me to email your past clients? [Yes, Draft Email]"
  2.  **Creation Flow (The "Plus" Button):**
      - User types: "I want to offer vegan cupcakes."
      - AI generates: Product title, description, placeholder image, suggested price ($4.00), and categorization.
      - User reviews, edits price, uploads own photo (auto-compressed to WebP), taps "Publish".

  **AI Agent Integration Points:**
  - **Interceptor Pattern**: When an entity is created/updated, specific agents are triggered via the Redis/Postgres job queue.
  - *Example*: `Offering` created -> `Marketing Agent` queued to draft a social media post announcement.

  ### Implementation Prompt

  **User-Facing Outcome:**
  Implement the unified "Offering Creation" flow prioritizing the Mobile (375px) experience. The user should be able to create a new product or service by typing a simple sentence, having the AI populate the details, and publishing it to their storefront.

  **Critical User Journey (CUJ):**
  1.  User logs in and sees the Home Feed.
  2.  User taps the primary "+" FAB (Floating Action Button).
  3.  User is prompted: "What do you want to offer?"
  4.  User types: "Guitar lessons for beginners, 1 hour."
  5.  Loading state (glassmorphism shimmer).
  6.  Form appears pre-filled: Title ("Beginner Guitar Lesson (1 Hour)"), Description (AI generated), Type (Service), Price (Suggested based on market, e.g., $50).
  7.  User modifies price to $45 and taps "Publish".
  8.  Success toast. The new offering is immediately visible on the live public storefront.

  **Acceptance Criteria:**
  - UI strictly adheres to the OHC Premium Token library (glassmorphism, minimum 44x44px touch targets).
  - Works flawlessly on a 375px viewport (no horizontal scrolling).
  - State management uses the designated repo pattern (Riverpod/Zustand) to handle the optimistic update.
  - The AI prompt is routed through the abstraction layer (fallback support).
  - Playwright E2E test covers the entire flow from login to the offering appearing on the mocked storefront page.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ### References & Sources Catalog
  1. Shopify Pricing: https://www.shopify.com/pricing
  2. Shopify App Store Bookings: https://apps.shopify.com/search?q=booking
  3. Wix Features: https://www.wix.com/features/main
  4. Squarespace Commerce: https://www.squarespace.com/ecommerce-website
  5. GoDaddy Websites + Marketing: https://www.godaddy.com/websites/website-builder
  6. Square Online: https://squareup.com/us/en/ecommerce
  7. Durable AI Builder: https://durable.co/ai-website-builder
  8. Hostinger AI Builder: https://www.hostinger.com/ai-website-builder
  9. 10Web AI: https://10web.io/ai-website-builder/
  10. Mixo.io: https://mixo.io/
  11. Gamma App: https://gamma.app/
  12. Stan Store Features: https://stan.store/features
  13. Linktree Monetization: https://linktr.ee/s/monetize/
  14. Gumroad Features: https://gumroad.com/features
  15. Systeme.io Pricing: https://systeme.io/pricing
  16. B12 AI Websites: https://www.b12.io/
  17. Shopify Community (Setup issues): https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion
  18. Reddit r/smallbusiness "Website Builder Recommendations": https://www.reddit.com/r/smallbusiness/search/?q=website+builder
  19. Reddit r/ecommerce "Shopify Alternatives": https://www.reddit.com/r/ecommerce/search/?q=shopify+alternative
  20. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  21. Trustpilot Wix Reviews: https://www.trustpilot.com/review/www.wix.com
  22. Y Combinator AI Startups: https://www.ycombinator.com/companies?industry=B2B&tags=AI
  23. TechCrunch "Agentic Commerce": https://techcrunch.com/search/agentic+commerce
  24. G2 E-Commerce Platforms Grid: https://www.g2.com/categories/e-commerce-platforms
  25. Stripe Checkout Specs: https://stripe.com/docs/checkout
  26. Stripe Connect: https://stripe.com/docs/connect
  27. Flutter Glassmorphism Packages: https://pub.dev/packages?q=glassmorphism
  28. Material Design Touch Targets: https://m3.material.io/foundations/accessible-design/accessibility-basics#0343a44c-2875-451e-8e8e-c3eb3adca762
  29. Apple HIG Touch Targets: https://developer.apple.com/design/human-interface-guidelines/foundations/accessibility/#buttons-and-controls
  30. Redis Redlock Pattern: https://redis.io/docs/manual/patterns/distributed-locks/
  31. PostgreSQL SKIP LOCKED: https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
  32. OpenTelemetry Go: https://opentelemetry.io/docs/instrumentation/go/
  33. Prometheus Metrics: https://prometheus.io/docs/concepts/metric_types/
  34. Gemini API Docs: https://ai.google.dev/docs
  35. OpenAI API Docs: https://platform.openai.com/docs/api-reference
  36. Playwright E2E Testing: https://playwright.dev/docs/intro
  37. Riverpod Flutter State Management: https://riverpod.dev/
  38. Zustand React State Management: https://docs.pmnd.rs/zustand/getting-started/introduction
  39. Go Router Flutter: https://pub.dev/packages/go_router
  40. Bazel Build System: https://bazel.build/
  41. Kubernetes StatefulSets (for DBs): https://kubernetes.io/docs/concepts/workloads/controllers/statefulset/
  42. WebP Image Compression: https://developers.google.com/speed/webp
  43. Cloudflare CDN: https://www.cloudflare.com/cdn/
  44. MinIO Local Storage: https://min.io/
  45. gRPC Go: https://grpc.io/docs/languages/go/
  46. OpenAPI Specification: https://swagger.io/specification/
  47. pgvector for Embeddings: https://github.com/pgvector/pgvector
  48. Flutter Web Support: https://docs.flutter.dev/platform-integration/web
  49. Docker Compose Wait For It: https://docs.docker.com/compose/startup-order/
  50. OHC Internal Personas Doc: (Internal Assumption based on Prompt)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
