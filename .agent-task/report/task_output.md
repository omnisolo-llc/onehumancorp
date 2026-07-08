issue_title: "Implement Universal Edge-Cached Dynamic Storefront for Mobile Discovery"
issue_description: |
  ## Target Persona
  Maya (Home Baker), Priya (Boutique Operator), Leo (Music Tutor)

  ## Problem Statement
  Owners need a blazingly fast, mobile-optimized public storefront to capture organic traffic, social media link-in-bio clicks, and direct referrals. Currently, dynamic storefronts suffer from slow load times on low-end devices and poor SEO, directly leading to high bounce rates for potential customers on 3G/4G networks. A laggy product discovery experience directly impacts Maya's custom cake orders and Priya's boutique sales.

  ## Research Report
  - **Market Context**: Platforms like Shopify and Wix utilize global CDNs and edge caching to deliver sub-second time-to-interactive metrics. Consumers expect instant loading on mobile.
  - **Competitor Gaps**: Traditional SaaS platforms often render storefronts server-side per request, causing latency spikes under load or in remote regions.
  - **The OHC Opportunity**: By introducing an edge-cached dynamic storefront architecture, OHC can guarantee sub-second loads for mobile shoppers, natively integrated with the OHC backend (Inventory, Pricing, and Availability) and AI agents (for instant support chat on the storefront).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    A[Mobile Shopper] -->|HTTPS| B[CDN Edge Cache]
    B -->|Cache Miss| C[Storefront Gateway]
    C -->|gRPC| D[Inventory/Catalog Service]
    C -->|gRPC| E[Tenant Context Service]
    B -->|Cache Hit| A
  ```
  ### Multi-Tenant Isolation
  - All public storefront routes are scoped by a unique tenant identifier (e.g., `/{tenant_slug}/products`).
  - Cache keys must include the `tenant_id` to prevent cross-tenant data leakage.
  - Invalidation triggers when inventory or product details change via OHC Admin.

  ### Mobile UX Flow
  1. **Shopper View (375px)**: Instant loading of the storefront homepage. Large touch-friendly product cards.
  2. **Product Detail**: Fast image loading (WebP). Clear Call to Action ("Order Now" or "Book").
  3. **AI Integration**: A minimal floating action button for the "Ambassador Agent" to answer questions like "Is this gluten-free?" instantly without navigating away from the page.

  ## Implementation Prompt
  - Design and implement the caching layer for the public storefront using Redis or a CDN integration.
  - Ensure all catalog API endpoints (`/api/v1/storefront/...`) support appropriate cache-control headers.
  - Create the mobile-first (375px) public storefront layout using OHC Premium Token library (Translucent Glass, UniFi layout).
  - Integrate the AI Ambassador agent as a floating chat component on the storefront.
  - Acceptance Criteria: Sub-second initial load on mobile emulation, cache hits for repeated product views, and successful cache invalidation on inventory updates. Do NOT prescribe exact CDN providers; focus on the caching architecture and mobile UX.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
