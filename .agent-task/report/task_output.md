issue_title: "Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  ## Problem Statement
  Non-technical business owners like Maya (The Home Baker) and Leo (The Music Tutor) need their storefronts to load instantly worldwide and rank high on Google (SEO). However, dynamically generated single-page applications (SPAs) or Flutter Web apps traditionally suffer from slow initial load times and poor SEO indexing. Competitors like Shopify and Wix invest heavily in edge caching and Server-Side Rendering (SSR) to solve this, but their solutions require technical configuration for dynamic catalog changes. OHC must provide instant, SEO-optimized storefronts automatically, with AI agents handling the optimization invisibly.

  ## Research Report
  - **Market Discovery:** Shopify uses edge caching (Shopify Edge) and heavily optimizes for Core Web Vitals. Wix utilizes SSR and distributed edge networks. OHC's architecture, heavily reliant on dynamic client-side rendering for management, needs a distinct approach for public-facing storefronts to guarantee sub-second LCP (Largest Contentful Paint) and pristine SEO.
  - **Competitor Gap:** Traditional platforms force users to manually optimize images, write meta descriptions, submit sitemaps, and manage URL redirects.
  - **OHC Innovation:** The "Marketing & Advertising" (Promoter) Agent should automatically pre-render public storefronts to static HTML/WebP, write SEO meta tags, generate dynamic XML sitemaps, and push these static assets to a global CDN (e.g., Cloudflare/CloudFront) every time inventory or content changes.

  ## Design Doc
  **Architecture Diagram:**
  ```mermaid
  graph TD
    A[Operations Agent updates Inventory] --> B[Promoter Agent triggered]
    B --> C[Fetch latest Tenant Data via gRPC]
    C --> D[Generate Static HTML & WebP Assets]
    D --> E[Inject SEO Meta Tags & JSON-LD]
    D --> F[Push to Edge CDN Cache / GCS]
    F --> G[Global Customers Browsing Instantly]
  ```

  **Mobile-First UX Flow:**
  - The feature operates entirely invisibly.
  - The business owner's mobile dashboard (375px) includes a simple "Storefront SEO & Speed" UniFi-style card in the Marketing section.
  - The card displays a status (e.g., "Excellent - Loading in 0.8s") and a feed of recent agent actions: "Promoter Agent updated meta tags for 3 new custom cakes."

  **Zero Trust & Security:**
  - The pre-rendering worker uses SPIFFE/SPIRE identity to securely access the tenant's read-only catalog data. Multi-tenant boundaries are strictly enforced via PostgreSQL Row Level Security (RLS) during the generation phase.

  ## Implementation Prompt
  - Build the background KAIROS DAG worker for the Promoter Agent that listens for tenant state changes (inventory, profile, services).
  - Develop the pre-rendering engine to construct static, SEO-optimized HTML pages for the public storefronts.
  - Integrate LLM (Gemini Pro) to autonomously generate high-converting SEO meta titles, descriptions, and JSON-LD structured data based on the business context.
  - Implement the publishing pipeline to push these static artifacts to the Edge CDN.
  - Implement a mobile-first (375px) "SEO & Speed" metric card in the frontend to surface the Promoter Agent's invisible work.
  - Acceptance Criteria: A newly added product must automatically trigger the agent, resulting in a live, edge-cached, SEO-optimized public product page within 60 seconds.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
