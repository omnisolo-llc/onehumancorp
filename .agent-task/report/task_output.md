issue_title: "[Architecture] Edge-Caching Dynamic Storefront Engine"
issue_description: |
  # [Architecture] Edge-Caching Dynamic Storefront Engine

  ## Problem Statement
  Small business owners like Maya (a baker who sells custom cakes via Instagram) experience highly volatile traffic. If she posts a reel that goes viral, thousands of people might click the link in her bio simultaneously. If her OneHumanCorp storefront takes more than a couple of seconds to load, she loses those sales. Current static site generators are fast but can't handle dynamic inventory checks (e.g., "only 2 vegan cakes left!"). Conversely, traditional dynamic servers buckle under the load or suffer from high latency when accessed globally. Maya, Carlos, and Priya need a storefront architecture that loads instantly (sub-50ms) globally, but remains dynamic enough to show real-time inventory, personalized quotes, and active booking slots.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify Oxygen / Vercel Edge Network:** Utilize edge computing to run lightweight compute closer to the user. They deliver dynamic content with the speed of static assets by caching at the edge and revalidating in the background (Stale-While-Revalidate).
  - **Wix / Squarespace:** Generally rely on heavy monolithic architectures or traditional CDNs. They are getting faster but often struggle to combine deeply dynamic user-specific state with instant first-paint times globally.
  - **Cloudflare Workers:** The gold standard for edge compute. Can intercept requests globally, stitch together cached HTML with live data fetched via lightweight key-value stores at the edge.

  **Gaps Identified:**
  OHC currently lacks a global edge-caching layer that dynamically injects real-time tenant state (inventory, pricing, active agent responses) into ultra-fast static shells. We need a system where a storefront's core HTML is cached at global edge nodes, while real-time data is injected at the edge or fetched instantly via an edge KV store, guaranteeing sub-50ms Time-To-First-Byte (TTFB) even under massive viral load.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network (e.g., Cloudflare/Fastly)
          EdgeNode[Edge Compute Worker] --> EdgeCache[(Edge KV Cache)];
          EdgeCache --> EdgeCDN[CDN Static Assets];
      end

      CustomerMobile[Customer Browser / Mobile] --> EdgeNode;

      subgraph Core Backend
          Gateway[OHC API Gateway] --> MainDB[(Cloud Postgres Ledger)];
          Gateway --> Agents[AI Agent Swarm];
      end

      EdgeNode -- "Cache Miss / Revalidate" --> Gateway;
      EdgeNode -- "Live Stock Check" --> EdgeCache;

      Gateway -- "Sync Invariants (Stock/Prices)" --> EdgeCache;

      subgraph Mobile Device (Merchant)
          MerchantApp[OHC Mobile App 375px] --> StorefrontUI[Storefront Config UI];
      end
      MerchantApp -- "Publish Changes" --> Gateway;
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** Maya opens the OHC app. She taps on "Storefront". The UI is snappy, using macOS-style Translucent Glass materials.
  2. **Performance Indicator:** A prominent dashboard card shows a green checkmark indicating "Global Edge Active - 99.9% Cache Hit Rate".
  3. **Updating Content:** Maya changes the price of a cake. She taps "Save".
  4. **Invisible Invalidation:** The app doesn't show complex loading bars or "Rebuilding Site" spinners. Instead, an AI Operations Agent confirms instantly: "Price updated globally." In the background, the edge cache is selectively invalidated and the edge KV is updated with the new price in milliseconds.
  5. **Advanced Settings (Hidden):** Terms like "Edge Cache", "CDN", and "TTFB" are hidden behind an "Advanced Settings" switch for the rare power user. Maya just knows her site is instantly fast.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory levels. When stock reaches 0, it directly updates the Edge KV cache to mark the item as "Sold Out" instantly, preventing overselling during a viral traffic spike without waiting for a full database roundtrip.
  - **Marketing Agent:** Analyzes traffic patterns. If it detects a massive spike (e.g., from an Instagram reel), it automatically scales the cache TTLs to ensure maximum edge hit rates and alerts the business owner of the viral activity with a summary notification.

  ### Key Design Decisions & Security
  - **Stale-While-Revalidate (SWR):** The edge network will serve cached content instantly while asynchronously fetching fresh data from the core backend.
  - **Zero-Trust Edge Isolation:** Edge compute workers will execute in strict isolation per tenant request. Data synced to the Edge KV store is cryptographically bound to the tenant ID.
  - **No Developer Jargon:** The merchant never configuring "CDNs" or "Edge Workers." The system provides "Instant Global Speed" by default.

  ## Implementation Prompt
  Implement the Edge-Caching Dynamic Storefront Architecture.
  - **User-Facing Outcome:** Customer storefronts load in under 50ms globally, handling massive traffic spikes effortlessly while accurately displaying real-time dynamic data like inventory and pricing. Merchants experience instant publish times without "rebuilding" screens.
  - **CUJ (Critical User Journey):**
    1. A merchant updates a product price in the 375px mobile app.
    2. The core backend updates the Edge KV cache instantly.
    3. A customer across the globe clicks the storefront link.
    4. The Edge Compute Worker serves the cached static shell and injects the updated live price from the Edge KV store, delivering the page in sub-50ms.
  - **Acceptance Criteria:**
    - Time-to-First-Byte (TTFB) must consistently be under 50ms globally for storefront reads.
    - Dynamic state (inventory, prices) must be accurate without full page re-renders.
    - Architecture must utilize Edge KV stores for real-time state and SWR caching patterns.
    - No infrastructure jargon should be exposed in the merchant mobile UI.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
