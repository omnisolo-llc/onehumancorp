issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  ## Issue Brief

  **Title**: Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture

  **Problem Statement**:
  Small business owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to high latency and lost revenue. Furthermore, they lack the technical expertise to configure CDNs or Server-Side Rendering for SEO, resulting in poor search engine visibility.

  **Research Report**:
  - **Competitors**: Shopify relies on third-party apps for advanced SEO. Vercel/Next.js are inaccessible to non-technical users. Wix/Squarespace require manual configuration.
  - **OHC Differentiator**: Invisible and autonomous edge caching and SEO pre-rendering.
  - **Value**: Guaranteed uptime during viral spikes, automated organic traffic growth, and reduced database scaling costs.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
  ```mermaid
  graph TD
      A[Operations/Marketing Agent] -->|Updates Inventory/Content| B(PostgreSQL DB)
      B --> C[Edge Cache Invalidator]
      C -->|Purge cache by tag| D(Redis/Cloudflare Edge Cache)
      E[Customer Browser] -->|Requests Storefront| D
      D -->|Cache Miss| F[Storefront Delivery API]
      F -->|Fetch Content & SEO| B
      F -->|Render HTML & Inject SEO| F
      F -->|Update Cache| D
  ```
  - **Architecture**:
    - Universal Edge Caching: All storefront reads hit a global edge cache (e.g., Cloudflare) automatically.
    - Agentic Cache Invalidation: The Operations Agent purges the edge cache key globally upon inventory updates.
    - Agentic SEO Pre-rendering: The Marketing Agent triggers a pre-rendering process upon website updates, generating static HTML with injected JSON-LD schema and meta tags, and pushes it to the edge.
  - **Mobile UX Flow**: The caching and pre-rendering happen invisibly in the background. The user sees a lightning-fast storefront on their 375px mobile device.
  - **AI Agent Integration**:
    - The Marketing Agent (`The Promoter`) generates structured JSON-LD schemas based on the tenant's product catalog.
    - The Operations Agent manages cache invalidation.

  **Implementation Prompt**:
  - Implement the core caching layer in `src/server/builder/edge.rs`.
  - Extend the `MarketingAgent` to automatically generate and inject SEO metadata (`seo_title`, `seo_description`, `seo_schema_json`) into product pages upon creation or update.
  - Ensure the edge cache is invalidated whenever a product or its SEO metadata is updated.
  - Add E2E tests verifying that updating a product triggers cache invalidation and serves updated SEO metadata on the edge storefront.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
