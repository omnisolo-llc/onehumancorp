issue_title: "Implement Autonomous Omnichannel Catalog Syndication Mesh"
issue_description: |
  # Autonomous Omnichannel Catalog Syndication Mesh

  ## Problem Statement
  Non-technical business owners like Priya (boutique) and Maya (baker) struggle to manage product listings across multiple sales channels (Instagram, TikTok, Google Shopping). Currently, they must manually upload photos, descriptions, and prices to each platform and manually sync inventory. This process is tedious, error-prone, and requires understanding platform-specific image requirements, product categories (taxonomies), and complex API sync rules.

  ## Research Findings
  *   **Competitors (Shopify, Wix, Squarespace):** Rely on complex app installations, manual category mapping, and often suffer from delayed inventory sync, leading to overselling.
  *   **OHC Opportunity:** OHC can leapfrog competitors by making omnichannel syndication entirely invisible. The KAIROS Orchestrator's AI Agents can analyze product photos/descriptions, automatically categorize them, resize/crop images for each platform, and push listings via background queues. An event-driven mesh guarantees near-instant inventory decrement across all channels.

  ## Next Steps
  Implement the Autonomous Omnichannel Catalog Syndication Mesh based on the detailed design document in `docs/research/[architecture]_autonomous_omnichannel_catalog_syndication_mesh.md`. This includes:
  1.  Unified `Product` data model.
  2.  AI-driven automatic taxonomy mapping (zero-config UI).
  3.  Edge asset transformation (resizing/cropping).
  4.  High-performance event queue (NATS JetStream) for inventory sync.
  5.  Mobile-first (375px), optimistic UI.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []