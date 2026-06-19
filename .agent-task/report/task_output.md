issue_title: "Implement Edge-Cached Dynamic Storefront with Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  - **Market Context**: Fast load times and strong SEO are critical for SMBs. Competitors like Shopify offer strong edge network capabilities, while Vercel/Next.js are the gold standard for developers. Wix/Squarespace provide easier SEO tools but lack instant scalability.
  - **The OHC Differentiator**: OHC's approach must be invisible and autonomous. All storefront reads must hit a global edge cache (e.g., Cloudflare) automatically.
  - **Agentic Cache Invalidation**: The Operations Agent must instantly purge specific edge cache keys globally when inventory updates occur (e.g., an item sells out).
  - **Agentic SEO Pre-rendering**: The Marketing Agent must autonomously trigger a pre-rendering process upon website updates, generating optimized static HTML with meta tags and pushing it to the edge.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      A[Customer Browser] --> B[Edge Cache CDN];
      B -- Cache Miss --> C[OHC Storefront Service];
      C --> D[PostgreSQL];
      E[Operations Agent] -->|Inventory Update| C;
      E -->|Invalidate Key| B;
      F[Marketing Agent] -->|Site Update| G[Pre-rendering Engine];
      G -->|Push Static HTML| B;
    ```
  - **UI Wireframes/Screen Flow**:
    - The storefront UI remains mostly unchanged but benefits from instantaneous load times.
    - Owner dashboard (375px mobile-first) shows SEO health and cache hit rates in plain language ("Your site is loading instantly for 95% of visitors!").
  - **AI Agent Integration Points**:
    - **Operations Agent**: Triggers cache invalidation upon inventory changes.
    - **Marketing Agent**: Triggers SEO pre-rendering upon site content changes.
  - **Key Design Decisions**:
    - Use a global CDN for edge caching.
    - Implement a robust cache invalidation strategy to prevent overselling.
    - Automate SEO pre-rendering to eliminate manual configuration for users.

  ## Implementation Prompt
  - **User-facing outcome**: The storefront loads instantly globally. The owner sees improved SEO rankings and traffic without any manual configuration.
  - **CUJ**: Maya updates a cake's description on her phone. The Marketing Agent automatically pre-renders the new page and pushes it to the edge cache. A customer in another country clicks her Instagram link and the page loads instantly.
  - **Acceptance Criteria**:
    - Storefront reads are served from the edge cache.
    - Inventory changes instantly invalidate the corresponding cache keys.
    - Site content changes trigger automated SEO pre-rendering.
    - The system must function flawlessly under simulated high-traffic loads.
    - All features must be fully functional on a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
