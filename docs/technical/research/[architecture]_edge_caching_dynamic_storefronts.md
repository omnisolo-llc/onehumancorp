# [Architecture] Edge-Caching Dynamic Storefronts

## Problem Statement
Small business owners like Maya (the baker selling custom cakes on Instagram) and Fatima (the food cart operator) need their online storefronts to be lightning-fast and universally available. If Maya's cake portfolio page takes 5 seconds to load when a customer clicks her TikTok link-in-bio, she loses the sale. If Fatima's pre-order menu goes down during a lunch rush, she loses crucial revenue. Furthermore, these users have absolutely no technical knowledge—they do not understand what "edge caching," "CDNs," or "serverless" mean. They just expect their site to load instantly anywhere in the world and never crash, even if a viral social media post brings a massive spike in traffic.

## Research Report
**Competitor Systems Audit:**
- **Shopify:** Leverages their global CDN (Cloudflare-backed) to deliver fast storefronts, but custom liquid themes or heavy apps can drastically slow down Time to First Byte (TTFB).
- **Vercel / Next.js Commerce:** The industry standard for edge-cached, highly performant React storefronts using Incremental Static Regeneration (ISR). However, it requires a developer to build and configure the pipelines.
- **Wix:** Has improved performance recently using SSR and caching, but often still struggles with complex dynamic content compared to pure edge-rendered frameworks.

**Gaps Identified:**
OneHumanCorp (OHC) needs a zero-configuration architecture that automatically compiles a user's mobile-first storefront into edge-cached, dynamic pages. It must provide the performance of a Vercel-hosted Next.js app (instant TTFB, offline capabilities, edge routing) without exposing any of the configuration or deployment complexities to the small business owner. The system must seamlessly invalidate cache only when Maya changes a cake price or adds a new photo, ensuring the storefront is always up-to-date yet instantly delivered.

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    subgraph Edge Network (CDN/Edge Workers)
        EdgeCache[(Edge Cache)]
        EdgeRouter[Edge Router & SSR]
    end

    subgraph User Traffic
        CustomerPhone[Customer Mobile View] --> EdgeRouter
        EdgeRouter -- Cache Hit --> EdgeCache
    end

    subgraph OHC Cloud Data Plane
        EdgeRouter -- Cache Miss / Dynamic API --> API_Gateway[OHC API Gateway]
        API_Gateway --> ContentDB[(Omnichannel Ledger / CMS)]
    end

    subgraph Merchant App
        MayaApp[OHC Mobile App 375px] --> CMS_Update[Update Price/Image]
        CMS_Update --> API_Gateway
        API_Gateway -- Invalidation Webhook --> EdgeCache
    end

    subgraph AI Agent Departments
        API_Gateway --> OpsAgent[Ops: Asset Optimization]
        API_Gateway --> MarketingAgent[Marketing: SEO & Analytics Sync]
    end
```

### Mobile UX Flow (375px First)
1. **Editing:** Maya opens the OHC app, taps her active storefront, and selects a cake. She changes the price from $45 to $50 and taps "Save".
2. **Invisible Optimization:** The app shows a brief "Updating storefront..." toast using macOS-style Translucent Glass materials. In the background, the system updates the database and sends a targeted invalidation request to the edge cache.
3. **Instant Delivery:** A customer clicks the link in Maya's Instagram bio. The edge worker nearest to the customer serves the newly cached page with the $50 price in under 50ms.
4. **Offline Resilience:** If the central database is momentarily unreachable, the edge worker continues serving the cached storefront, queuing any new customer orders/deposits to sync later, ensuring Maya never misses a sale.

### AI Agent Integration Points
- **Operations Agent:** Intercepts image uploads (e.g., Maya's new cake photo), automatically compresses them into Next-Gen formats (WebP/AVIF), and pushes them to the edge CDN before the cache invalidation completes.
- **Marketing Agent:** Continuously analyzes the edge analytics (bounces, TTFB, popular products) and suggests simple, plain-language improvements to the user (e.g., "Your vegan cake page is getting a lot of views! Want me to move it to the top of your menu?").

### Key Design Decisions & Security
- **Incremental Static Regeneration (ISR) Concept:** Storefronts are pre-rendered and cached at the edge. Mutations to the central ledger trigger targeted cache invalidations rather than requiring full site rebuilds.
- **Zero-Configuration:** Users never see terms like "deploy," "build," or "cache." The state is simply "Live."
- **Multi-Tenant Edge Isolation:** Edge cache keys and API routes are strictly partitioned by the tenant's SPIFFE SVID. A cache poisoning attack on one tenant cannot affect another.

## Implementation Prompt
Implement the Edge-Caching Dynamic Storefront routing and generation engine.
- **User-Facing Outcome:** Customers visiting a merchant's OHC link-in-bio or custom domain experience instant page loads (<50ms TTFB) globally. Merchants can update their catalog in the mobile app and see changes reflect on their live site immediately without understanding the underlying deployment pipeline.
- **CUJ:** Maya updates a product price in the app. The backend updates the database and fires a targeted cache invalidation. A customer visiting the site immediately sees the new price served directly from the nearest edge node.
- **Acceptance Criteria:** Storefronts must be served from an edge CDN/worker. Updates via the API must selectively invalidate the edge cache rather than requiring a full rebuild. Ensure uploaded media is automatically optimized by the Ops Agent before being cached. The merchant UI must hide all caching and deployment complexity behind a simple "Save" action.

## Priority
P0

## Estimated Scope
Large
