issue_title: "[Architecture] Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and SEO penalties. Existing platforms like Vercel require developer knowledge, while simpler builders like Wix lack the autonomous, instant scalability of true edge architectures during unpredictable spikes. SMBs need a storefront that is instantly fast globally and automatically optimized for search engines without manual configuration.

  ## Research Report
  Our research into high-scale capabilities (Track 1 & Track 2) reveals a critical gap in the OHC platform. While we excel at agentic workflows, our core storefront delivery architecture lacks edge caching and static pre-rendering.
  - **Competitor Systems Audit**: Shopify uses Cloudflare for edge caching. Vercel/Next.js dominates with ISR/SSG but is too technical.
  - **OHC Gap**: OHC must shift from dynamic centralized rendering for every request to an edge-cached architecture. Crucially, this must be invisible to the user.
  - **The Solution**: Implement Universal Edge Caching for all storefront reads, coupled with Agentic Cache Invalidation (triggered by inventory changes) and Agentic SEO Pre-rendering (triggered by content updates).

  ## Design Doc
  ### High-Level Architecture
  - **Universal Edge Caching**: All storefront reads hit a global CDN (e.g., Cloudflare) automatically.
  - **Agentic Cache Invalidation**: When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally (`ohc:cache:{tenant_id}:storefront:{page_id}`).
  - **Agentic SEO Pre-rendering**: When the Marketing Agent updates the website, it autonomously triggers a background job to generate highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge.

  ### Data Model & Invariants
  - `CacheConfig`: Tenant-scoped rules for cache TTLs.
  - `PreRenderJob`: A queue table for SEO rendering tasks.
  - Strict multi-tenant isolation ensures cache keys are namespaced by `tenant_id`.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      STOREFRONT_APP ||--o{ EDGE_CDN : "Reads cached pages"
      EDGE_CDN ||--o{ BACKEND_API : "Fetches dynamic content (cache miss)"
      MARKETING_AGENT ||--o{ PRE_RENDER_QUEUE : "Triggers SSG Job"
      OPERATIONS_AGENT ||--o{ BACKEND_API : "Triggers invalidation"
      PRE_RENDER_QUEUE }|--|| EDGE_CDN : "Pushes static HTML"
      BACKEND_API }|--|| EDGE_CDN : "Invalidates keys"
  ```

  ### Mobile UX Flow (375px First)
  - The feature is primarily invisible infrastructure, but the owner dashboard (375px) will feature a new "Performance & SEO" card.
  - The card displays "Edge Cache Active" and "SEO Status: Excellent" using OHC Premium Tokens (Glassmorphism, green status dots).
  - No manual "Clear Cache" button should be needed; it's fully autonomous.

  ### AI Integration Points
  - **Operations Agent**: Triggers targeted cache invalidation on inventory updates.
  - **Marketing Agent**: Triggers full-page SEO pre-rendering on content changes and generates dynamic meta tags.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering engine.
  1. Define the caching headers and edge-routing rules in the API Gateway/Backend to support edge caching.
  2. Create a `PreRenderJob` queue using the PostgreSQL `SKIP LOCKED` pattern.
  3. Implement the worker that fetches dynamic content, injects AI-generated SEO metadata, renders static HTML, and uploads it to an edge-accessible store (e.g., GCS/S3).
  4. Integrate the Operations Agent to trigger targeted cache invalidations upon inventory changes.
  5. Add a "Performance" card to the mobile dashboard UI showing the active edge caching status.
  Do not prescribe specific CDN providers; design the architecture to be provider-agnostic. Include comprehensive tests for the cache invalidation logic and Playwright E2E tests for the new dashboard card.

  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
