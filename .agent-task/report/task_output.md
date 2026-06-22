issue_title: "Build Zero-Config AI-Driven Edge Storefronts for Instant Consumer Discovery"
issue_description: |
  **Problem Statement**
  Business owners like Maya (the baker) and Priya (the boutique owner) rely on social media (Instagram, TikTok) to drive discovery, but they lack a lightning-fast, visually appealing, SEO-optimized web presence to capture and convert this demand seamlessly. Currently, OHC requires users to navigate complex setup processes or rely on unoptimized, slow-loading endpoints for customer interaction. Mobile users dropping off due to sluggish page loads directly impacts the owners' revenue and growth potential. An owner should never have to think about "edge caching" or "web vital metrics"; they just need their products and booking links to load instantly everywhere.

  **Research Report**
  An analysis of leading platforms (Shopify, Wix, Squarespace) reveals that competitive advantage lies in sub-second Time-to-Interactive (TTI) metrics, enabled by edge-caching dynamic content closer to the consumer.
  - **Shopify**: Utilizes global edge networks (Cloudflare/Fastly) to cache entire storefront HTML and assets, invalidating cache intelligently when inventory or prices change.
  - **Wix/Squarespace**: Pre-renders content and serves it via CDNs, offering heavily optimized mobile templates.
  - **Gap in OHC**: OHC currently serves customer-facing catalog/booking pages directly from the central database, resulting in high latency, particularly for media-heavy catalogs (e.g., Maya's cake photos). Furthermore, the initial setup process for these pages isn't fully autonomous, causing friction for non-technical users.

  **Design Doc**

  *Architecture diagram (Mermaid.js)*
  ```mermaid
  sequenceDiagram
      participant Consumer as Mobile Consumer (375px)
      participant Edge as CDN / Edge Cache
      participant OHC as OHC App Server
      participant Agent as Promoter Agent
      participant DB as PostgreSQL DB

      Consumer->>Edge: Request Storefront Page
      alt Cache Hit
          Edge-->>Consumer: Return Pre-Rendered HTML (Instant)
      else Cache Miss
          Edge->>OHC: Fetch Page
          OHC->>DB: Query Tenant Catalog/Offers
          OHC-->>Edge: Return Rendered HTML
          Edge-->>Consumer: Return Rendered HTML
          Edge->>Edge: Cache HTML
      end

      Note over Agent, DB: Background Generation
      Agent->>DB: Owner Adds New Product
      Agent->>OHC: Trigger Cache Invalidation for Tenant
      OHC->>Edge: Purge Storefront Cache
      OHC->>Edge: Pre-warm Cache with Updated HTML
  ```

  *UI wireframes or screen flow description (375px first)*
  1. **Owner Setup (The "Magic" Link):** Inside the OHC mobile app (375px), Maya taps a single button: "Publish Storefront."
  2. **Agent Interaction:** The Promoter Agent asks, "Should I feature your custom cakes or ready-to-buy items first?" Maya selects "Custom Cakes."
  3. **Consumer View:** Maya shares her OHC link on Instagram. When a consumer taps it, they see a full-screen, translucent glass-styled catalog. Large, compressed WebP images load instantly. A sticky bottom bar provides a clear Call to Action: "Request Custom Order" or "Book Now."

  *Mobile UX flow*
  - **Owner Path:** Home Tab -> "Grow Business" Card -> "Review Storefront" -> AI generates layout -> Approve -> Link copied to clipboard.
  - **Consumer Path:** Tap Instagram Link -> Instant load (<1s) -> Scroll through media-rich variants -> Tap "Buy/Book" -> Apple/Google Pay native sheet appears -> Confirmation.

  *AI agent integration points*
  - **The Promoter Agent:** Analyzes the tenant's current inventory, most popular items, and recent photos to automatically generate and arrange the storefront layout. It writes SEO meta tags and product descriptions based on brief owner notes.
  - **The Customer Assistant:** Seamlessly hands off inquiries from the storefront into the unified owner inbox, categorizing them as leads.

  *Key design decisions and why*
  - **Zero-Config Deployment:** Owners do not configure domains, DNS, or caching rules. OHC automatically generates a high-performance `[tenant-name].ohc.app` link.
  - **Edge-First Architecture:** All customer-facing read requests must be served from an edge CDN cache to guarantee sub-second load times globally, protecting the core OHC database from traffic spikes.
  - **Aggressive Media Optimization:** All uploaded media is automatically compressed to WebP and resized for mobile viewports by a background worker before being served to the edge.

  **Implementation Prompt**
  Implement the "Edge-Cached Storefront Engine" for OHC. Create the backend service responsible for generating public-facing, SEO-optimized HTML catalogs for a tenant's offerings. This service must integrate with a caching layer (e.g., Redis or an external CDN abstraction) to ensure sub-100ms response times for cached reads. The implementer must also create a background event listener that invalidates and regenerates this cache whenever a tenant updates their inventory or pricing. Develop the mobile-first (375px) public storefront UI adhering to the premium Translucent Glass design system. Provide Playwright E2E tests that verify the storefront loads correctly for a consumer and that cache invalidation works successfully when the owner updates a product.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
