issue_title: "[Research] Universal Edge-Cached Dynamic Storefront & Agentic SEO Architecture Design"
issue_description: |
  # Problem Statement
  Small business (SMB) platforms currently suffer from a massive architectural gap: they either provide fast, static websites with no native operational capabilities (like early Link-in-Bio tools) OR they provide complex, monolith-driven dynamic storefronts (like Shopify/WooCommerce) that are slow for end-users during traffic spikes and require manual SEO configuration that non-technical owners (like Maya the Baker or Carlos the Handyman) don't understand.

  When an SMB goes viral on TikTok, their slow monolith storefront often crashes or abandons carts due to latency. Furthermore, when Carlos adds a new service area, he doesn't know how to update schema tags to capture local Google search traffic.

  # Research Report
  Based on the `agentic_autonomous_website_builders_smb_platform_gap_analysis.md` and related documents, OHC's target is "Quadrant 1" (Simple/Mobile-First + Autonomous Execution). Currently, OHC requires a "Universal Edge-Cached Dynamic Storefront" combined with "Agentic SEO Pre-rendering".

  Competitors like Shopify rely on third-party caching apps or complex headless setups (Oxygen) that SMBs cannot manage. Wix and Squarespace manage caching but lack true autonomous SEO agents that pre-render localized content based on trends.

  The solution is an architecture where all public storefronts are edge-cached by default, and dynamic capabilities (like checking real-time inventory or localized pricing) are hydrated securely at the edge via lightweight functions, completely isolating the core Postgres/Rust monolith from public traffic spikes. Additionally, "The Promoter" agent must be able to autonomously generate optimized static HTML variants of the storefront and push them to the edge cache.

  # Design Doc
  ## Architecture

  ```mermaid
  sequenceDiagram
      participant Customer as Mobile Customer
      participant Edge as CDN / Edge Cache (e.g. Cloudflare/Nginx)
      participant RustCore as OHC Rust Core API
      participant Agent as The Promoter (SEO Agent)
      participant DB as Postgres (Row Level Security)

      Agent->>RustCore: Detects new "Vegan Cake" product added by Maya
      RustCore->>Agent: Returns product details & local search trends
      Agent->>Agent: Generates SEO-optimized HTML & Schema.org markup
      Agent->>Edge: Pushes pre-rendered static HTML to cache layer
      Customer->>Edge: Visits maya.ohc.store/vegan-cake
      Edge-->>Customer: Returns cached HTML (sub-50ms)
      Customer->>Edge: Adds to Cart (Dynamic Action)
      Edge->>RustCore: Edge function routes API call for cart mutation
      RustCore-->>Edge: Returns updated cart state
      Edge-->>Customer: Hydrates cart UI
  ```

  ## Mobile UX Flow (375px)
  1. The business owner (Maya) never configures caching or SEO.
  2. Maya opens the OHC App (375px view).
  3. In the **Unified Agent Feed**, an Action Card appears: "I noticed you added Vegan Cakes. I've pre-rendered a new SEO page targeting 'Vegan Bakeries in Austin' and pushed it live to capture local traffic. View Performance?"
  4. Maya taps "View Performance" to see a simple, non-technical chart of incoming localized traffic.

  ## AI Agent Integration Points
  - **The Promoter (Marketing Agent)**: Monitors the `tenant_inventory` and `tenant_settings` tables. When significant changes occur (new products, changed hours), it triggers a background job to re-generate the static HTML shell.
  - **The Operations Agent**: Informs The Promoter if an item goes completely out of stock so the cached HTML can be updated to show "Sold Out" without requiring a dynamic API call on page load.

  ## Key Design Decisions
  - **Strict Separation of Public Read vs Private Write**: Public storefronts must NEVER hit the Rust core monolith directly for initial HTML rendering. The edge cache is the source of truth for public display.
  - **Edge Hydration for Commerce**: The static HTML contains lightweight JS that makes authenticated API calls back to the Rust core ONLY for dynamic actions (Cart, Checkout, Tap-to-Pay).
  - **Zero Configuration**: There is no "Clear Cache" button or "SEO Settings" page in the mobile app. The AI agent manages cache invalidation and meta tags autonomously based on business events.

  # Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the foundation for the Universal Edge-Cached Dynamic Storefront.

  1. **Agent Coordination**: Extend the background worker queue (`OHCJobQueue`) to support a new job type: `RenderStorefrontToEdge`.
  2. **The Promoter Integration**: Update "The Promoter" agent logic (or create the stub) so that when a new product is created in the catalog, it enqueues a `RenderStorefrontToEdge` job.
  3. **Agent Feed Output**: Ensure that when the `RenderStorefrontToEdge` job completes successfully, it pushes a notification card to the `tenant_feed_items` table so the owner sees the action in their mobile Unified Agent Feed.

  **Acceptance Criteria:**
  - A test verifies that adding a product enqueues the rendering job.
  - A test verifies that the completion of the job results in a new Agent Feed item.
  - All new architecture must respect row-level security (RLS) for the tenant.
  - No changes should require manual owner configuration in the UI.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
