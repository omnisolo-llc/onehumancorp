issue_title: "Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical small business owners (SMBs) like Maya the Baker or Leo the Musician heavily rely on social media to drive traffic. When a post goes viral, the resultant traffic spike can overwhelm unoptimized, centralized databases. This causes high latency, timeouts, and ultimately, lost revenue. Additionally, search engines struggle to index slow, client-side rendered dynamic content, resulting in poor organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO.

  ## Research Report
  Our competitive analysis indicates that while enterprise platforms like Shopify offer strong edge network capabilities (via Cloudflare), they often still require third-party apps for advanced SEO. Developer-centric ecosystems (like Vercel/Next.js) provide excellent ISR and Edge computing features but remain inaccessible to non-technical users. Platforms like Wix and Squarespace offer simpler SEO tools but require manual configuration and do not provide autonomous, instant scalability during massive, unpredictable spikes.

  To differentiate OHC, our approach must be completely invisible and autonomous. It involves three core capabilities:
  1. **Universal Edge Caching**: All storefront reads automatically hit a global edge cache without any user configuration.
  2. **Agentic Cache Invalidation**: When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
  3. **Agentic SEO Pre-rendering**: The Marketing Agent autonomously triggers a pre-rendering process upon website updates, generating highly optimized, static HTML injected with relevant meta tags and structured data, pushed directly to the edge.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    graph TD;
        User[End User/Search Engine Crawler] --> EdgeCache[Edge Cache CDN];
        EdgeCache -- Cache Miss --> StorefrontProxy[Edge Worker / Storefront Proxy];
        StorefrontProxy --> OHCAPI[OHC API Backend];
        OHCAPI --> CentralDB[(PostgreSQL / Redis)];

        OpsAgent[Operations Agent] -.-> |Invalidates Cache Key| EdgeCache;
        OpsAgent -.-> |Updates Inventory| CentralDB;

        MarketingAgent[Marketing Agent] -.-> |Triggers Pre-render| PreRenderService[Pre-rendering Engine];
        PreRenderService -.-> |Pushes Static HTML| EdgeCache;
        PreRenderService -.-> |Fetches Content| OHCAPI;
    ```
  - **UI/UX Flow (Mobile-First 375px)**:
    - The configuration for Edge Caching and SEO Pre-rendering is entirely invisible to the user. It happens in the background.
    - In the OHC Mobile App, when the user (e.g., Priya) updates a product or launches a new marketing campaign, she receives an Agent Action Card via the Unified Agent Feed.
    - *Action Card*: "Your product 'Red Dress' is live! The Marketing Agent is optimizing it for search engines globally. [View Preview]"
    - Touch targets are large (>44px). No complex CDN or SEO settings are exposed unless explicitly requested in "Advanced Mode".
  - **AI Agent Integration Points**:
    - **Operations Agent**: Monitors inventory changes and orchestrates fine-grained cache invalidation requests via the CDN's API.
    - **Marketing Agent**: Analyzes product updates and automatically generates optimized meta tags, titles, and descriptions, then triggers the Pre-rendering Engine.
  - **Key Design Decisions**:
    - We opt for an Agent-driven approach to SEO and Caching to remove the technical burden from the SMB owner.
    - Cache invalidation must be event-driven and granular to prevent stale inventory from causing overselling, a critical issue for small businesses.

  ## Implementation Prompt
  **Feature Name**: Agent-Driven Storefront Caching & SEO Pre-rendering
  **Target Persona**: Maya the Baker

  **Outcome**: When Maya updates her cake catalog, her storefront is instantly globally cached and fully pre-rendered for search engines, ensuring fast load times and high SEO rankings without her needing to touch a single configuration setting.

  **Critical User Journey (CUJ)**:
  1. Maya opens the OHC mobile app.
  2. She adds a new "Summer Berry Cake" product with a photo and description.
  3. She taps "Publish".
  4. In the background, the Marketing Agent generates SEO metadata. The Pre-rendering Engine creates a static version of her storefront and pushes it to the Edge Cache.
  5. Maya receives an Agent Feed notification: "Summer Berry Cake is live and optimized for Google search!"
  6. A customer clicking her Instagram link experiences sub-second load times due to Edge Caching.

  **Acceptance Criteria**:
  - The feature must be completely autonomous; no manual SEO or cache configuration by the user.
  - Granular cache invalidation must occur immediately when inventory or product details change.
  - The storefront must serve static HTML to web crawlers to ensure optimal indexing.
  - Must include E2E tests verifying that updates trigger cache invalidation and pre-rendering processes.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
