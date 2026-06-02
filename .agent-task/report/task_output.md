issue_title: "[Platform Architecture] Missing Edge-Caching Dynamic Storefront Capability"
issue_description: |
  # The Problem
  Maya, the baker, needs a fast storefront that doesn't go down under load. Currently, OneHumanCorp (OHC) lacks an explicit edge-caching layer specifically optimized for high-traffic dynamic storefronts. While we have CDN-fronted file storage (GCS/MinIO), we do not have a caching architecture for the *dynamic* generated content (the storefront HTML, product catalogs, variant selections).

  # Research Findings
  Platforms like Shopify, Wix, and Vercel heavily rely on edge caching to serve high-traffic storefronts with near-zero latency, while invalidating dynamically when inventory or product details change. We are missing this crucial link between our backend multi-tenant data model and our CDN-fronted assets. We must bridge this gap to provide an "invisible" high-performance layer that requires zero configuration from the user.

  # Design Solution
  We will introduce a Dynamic Edge-Caching Storefront Architecture that:
  1. Utilizes a unified cache invalidation protocol driven by PostgreSQL event triggers or job queue consumers.
  2. Involves the Operations department to handle transparent cache invalidation when inventory, pricing, or product variants change.
  3. Uses Redis as a backing store with appropriate multi-tenant isolation schemas (`ohc:cache:{tenant_id}:{resource_type}:{hash}`).

  ## Mobile & UX Flow
  - Maya continues to manage her storefront from her 375px phone screen.
  - The UI will have an implicit "publish/update" mechanism that reflects instantly for her preview but propagates through the edge cache for her customers.
  - A simple "Store Performance" metric card will show Maya her site load speed, proving the value of the platform.

  # Implementation Prompt
  Implement the edge-caching dynamic storefront architecture.
  1. Establish the caching schema in Redis respecting the multi-tenant `tenant_id` boundaries.
  2. Implement the invalidation logic, specifically hooking into the Operations AI department's inventory/product update flows.
  3. Update the frontend UI to display the "Store Performance" metric card (ensuring it adheres to the macOS Translucent Glass and UniFi layout standards).
  4. Ensure complete coverage with unit tests and a Playwright E2E test verifying the fast loading and correct invalidation of a storefront after a product update.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
