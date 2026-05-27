issue_title: "Architect and Implement the Zero-Touch AI Link-in-Bio & Dynamic Portfolio Generator"
issue_description: |
  # Architect and Implement the Zero-Touch AI Link-in-Bio & Dynamic Portfolio Generator

  ## Problem Statement
  Small business owners, especially creatives and service providers like Leo (music tutor) and Maya (custom baker), heavily rely on social media platforms (Instagram, TikTok) for customer acquisition. They need a single, highly optimized "link-in-bio" that serves as a mobile-first portfolio, booking engine, and storefront. Current solutions like Linktree are static and require manual updating, while full website builders (Wix, Squarespace) are too complex for a simple link-in-bio use case. Users need a dynamic, continuously updating portfolio that automatically pulls their latest work from social feeds, highlights their most popular services, and allows for 1-tap bookings and payments, all managed invisibly by AI without them ever having to edit a page.

  ## Research Report
  ### The Link-in-Bio Market Gap
  The "link-in-bio" is the modern storefront, but current tools fail the small business owner in critical ways:
  - **Linktree / Beacons:** Highly popular but essentially static lists of links. They require manual upkeep. Adding a new service or changing a price requires logging in and editing buttons. E-commerce integrations feel bolted on and take users away from the native experience.
  - **Shopify Linkpop:** Better integrated with commerce, but still requires manual curation. It is not designed to serve as a portfolio for service providers like Leo.
  - **Wix / Squarespace:** Building a single landing page is overkill and hard to maintain for a mobile-first user.

  ### OneHumanCorp Differentiation
  OHC will provide an **Autonomous Link-in-Bio Portfolio**. Instead of manually adding links, the AI Marketing and Operations agents will dynamically generate and update the page based on real-time business activity. When Maya posts a new cake on Instagram, the OHC Marketing Agent automatically adds it to her portfolio gallery. When Leo's Tuesday slots are filled, the AI Operations Agent automatically bumps his "Waitlist" link or highlights his digital course. The user never touches an editor; they simply run their business, and their link-in-bio perfectly reflects it.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ SOCIAL_ACCOUNT : "connects"
      MERCHANT ||--o{ PRODUCT_SERVICE : "offers"
      MERCHANT ||--o{ DYNAMIC_PORTFOLIO : "owns"
      SOCIAL_ACCOUNT ||--o{ MEDIA_ASSET : "imports"
      PRODUCT_SERVICE ||--o{ PORTFOLIO_BLOCK : "populates"
      MEDIA_ASSET ||--o{ PORTFOLIO_BLOCK : "populates"
      DYNAMIC_PORTFOLIO ||--o{ PORTFOLIO_BLOCK : "renders"

      %% AI Departments Interactions
      MARKETING_AGENT ||--o{ MEDIA_ASSET : "curates & tags"
      OPS_AGENT ||--o{ PRODUCT_SERVICE : "monitors availability"
      MARKETING_AGENT ||--o{ DYNAMIC_PORTFOLIO : "re-ranks blocks based on performance"
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant SocialMedia as Instagram / TikTok
      participant OHC_MarketingAgent as AI Marketing Dept
      participant OHC_PortfolioDB as Portfolio Ledger
      participant OHC_Edge as Edge Storefront (CDN)

      Maya->>SocialMedia: Posts photo of new "Vegan Chocolate Cake"
      SocialMedia->>OHC_MarketingAgent: Webhook / API Sync
      OHC_MarketingAgent->>OHC_MarketingAgent: Image AI: Identifies "Cake", "Vegan"
      OHC_MarketingAgent->>OHC_PortfolioDB: Ingests Media & Maps to Product Catalog
      OHC_PortfolioDB->>OHC_MarketingAgent: Identifies high-converting layout
      OHC_MarketingAgent->>OHC_PortfolioDB: Updates Portfolio Block Order
      OHC_PortfolioDB->>OHC_Edge: Invalidates cache & redeploys page
      Customer->>OHC_Edge: Clicks Link-in-bio
      OHC_Edge-->>Customer: Serves updated, dynamic portfolio with new cake prominent
  ```

  ### Mobile UX Flow (375px First)

  **Screen 1: The Merchant Setup (Zero-Config)**
  - **Dashboard Card:** "Set up your Link-in-Bio."
  - **Action:** User taps "Connect Instagram/TikTok."
  - **Magic Moment:** The AI instantly analyzes their feed, matches their OHC products/services to their social presence, and generates a live link. A translucent glass preview appears, confirming "Your dynamic portfolio is live."

  **Screen 2: The Customer Experience (The Link-in-Bio)**
  - **Header:** Merchant Profile Photo, Name, and AI-generated dynamic bio ("Booking Piano Lessons | 3 spots left this week!").
  - **Dynamic Content Blocks (Glassmorphism UI):**
    - **Featured Action:** Large card (e.g., "Book a 1-on-1 Lesson" for Leo, or "Order Custom Cake" for Maya).
    - **Live Gallery:** A horizontal scrolling carousel of recent, curated social posts that are shoppable. Tapping an image brings up a 1-tap checkout for the relevant product.
    - **Smart Links:** A few standard links (FAQ, Contact) auto-generated by the AI.
  - **Sticky Bottom Action:** A persistent "Book Now" or "Buy" button ensuring high conversion.

  ### AI Agent Integration Points
  - **Marketing Agent:** Continuously syncs connected social feeds, uses computer vision to tag images, and pairs them with existing inventory. A/B tests block order (e.g., does the booking link convert better above or below the gallery?).
  - **Operations Agent:** Monitors inventory and calendar availability. If a service is sold out, it automatically updates the link text to "Join Waitlist" or hides the block entirely, preventing customer frustration.
  - **Finance Agent:** Tracks the revenue generated directly from the link-in-bio and provides a simple weekly SMS summary ("Your bio link brought in 5 new bookings this week!").

  ### Key Design Decisions
  - **Edge Caching with Instant Invalidation:** The dynamic portfolio must be served from the edge for near-zero latency, even when bombarded by a viral TikTok. The AI agents will trigger cache invalidations only when the underlying dynamic content materially changes.
  - **No Drag-and-Drop Editor:** Merchants cannot manually drag blocks around. They can only set broad preferences ("Prioritize Bookings" vs "Prioritize Digital Downloads"). The Marketing Agent handles the layout optimization.
  - **Native Checkout:** The link-in-bio is not just a redirect; it uses OHC's universal 1-click checkout engine directly on the page, keeping the user in context.

  ## Implementation Prompt
  **For the Engineering Swarm:**
  Implement the core backend engine and data models for the Zero-Touch AI Link-in-Bio & Dynamic Portfolio.
  - **CUJ:** Maya connects her Instagram account to OHC. The Marketing Agent automatically pulls her latest 10 posts, identifies the ones featuring her core products, and generates a mobile-optimized link-in-bio page. When a customer clicks her bio link, they see her latest work and can buy directly from the page using Apple Pay, without Maya ever having to manually update the links.
  - **Acceptance Criteria:**
    - Create the `DYNAMIC_PORTFOLIO` and `PORTFOLIO_BLOCK` tenant-isolated data models.
    - Build the webhook receivers for social media sync (Instagram/TikTok basic graph API).
    - Implement the Marketing Agent logic that translates a social post into a `PORTFOLIO_BLOCK` and ranks it.
    - Ensure the output is designed to be consumed by an edge-cached static site generator or edge function.
    - Do not build a user-facing visual editor. The layout must be strictly data-driven.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []