issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Architecture Gap: Missing Edge-Cached Dynamic Storefront and Agentic SEO for SMBs

  **Problem Statement:**
  Currently, SMB owners on platforms lack the technical ability to handle massive, unpredictable traffic spikes (e.g., going viral on social media) without facing high latency or timeouts, which frustrates customers and causes lost revenue. They also lack the expertise to configure SSG or SSR for their storefronts, leading to poor search engine visibility.

  **Research Report:**
  Based on the competitive landscape (Shopify, Vercel, Wix), the gap lies in offering an invisible and autonomous edge caching and pre-rendering solution. SMBs cannot be expected to configure CDNs or cache invalidations. We need an architecture where storefront reads hit a global edge cache automatically, and where agentic operations (like the Operations Agent updating inventory) autonomously invalidate specific cache keys. Furthermore, SEO pre-rendering should happen automatically via the Marketing Agent.

  **Design Doc:**
  *Architecture:*
  - Read traffic to storefronts routes through a CDN/Edge caching layer (e.g., Cloudflare Workers).
  - Write traffic (inventory updates) hits the central PostgreSQL DB, which triggers an Agentic invalidation process.
  - The Marketing Agent detects content changes and triggers a Server-Side Rendering (SSR) pipeline to generate static HTML with SEO metadata, pushing it to the Edge Cache.

  *Architecture Diagram:*
  ```mermaid
  sequenceDiagram
    participant User
    participant EdgeCache
    participant OHCApi
    participant DB
    participant Agent

    User->>EdgeCache: GET /storefront
    EdgeCache-->>User: Cached HTML

    User->>OHCApi: POST /inventory (Update)
    OHCApi->>DB: Update Stock
    OHCApi->>Agent: Trigger Invalidation Event
    Agent->>EdgeCache: Purge Cache Key

    Agent->>Agent: Generate Static HTML (SSR)
    Agent->>EdgeCache: Push New HTML
  ```

  *Mobile UX Flow:*
  - The business owner updates inventory or content on their 375px mobile device within the OHC Assistant.
  - They see a quick success toast: "Store updated."
  - Invisibly, the Operations/Marketing Agents handle cache invalidation and SEO pre-rendering.

  *UI Wireframes (375px):*
  - **Screen 1 (Inventory Edit):** Clean, translucent glass UI. Input for quantity. Button to "Save".
  - **Screen 2 (Success Toast):** Small floating green toast at bottom "Store updated". No technical jargon about cache or rendering.

  **Implementation Prompt:**
  As an Implementer, your task is to build the backend infrastructure and agent coordination for this architecture. You must:
  1. Implement a generic Edge Cache interface that the OHC server can interact with.
  2. Modify the inventory and storefront write paths to publish cache invalidation events.
  3. Create an Agentic workflow (within the Marketing/Operations domains) that listens to these events, generates static HTML for the storefront, and pushes it to the Edge Cache.

  **Top 5 Codebase Issues:**
  1. Missing clear multi-tenant boundaries in some older database schemas.
  2. The legacy Next.js UI is still partially present and confusing for new developers.
  3. Lack of consistent error handling in some background worker queues.
  4. The test suite has some flaky tests related to timing issues.
  5. The CI pipeline occasionally hits rate limits pulling images.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
