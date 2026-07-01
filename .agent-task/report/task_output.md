issue_title: "[Research] Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Small business owners face dual challenges when trying to grow online:
  1. **Performance Bottlenecks:** When a post goes viral, the resulting traffic spike can overwhelm traditional centralized databases, causing high latency, timeouts, and lost sales.
  2. **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content effectively, diminishing organic reach.

  Existing solutions like Shopify offer robust edge caching but are complex, while platforms like Wix and GoDaddy fail to provide the autonomous scalability required for high-performance e-commerce without manual setup. OHC lacks a unified, invisible edge-caching and SEO pre-rendering mechanism that "just works" for non-technical users.

  ## Research Report
  - **Competitive Landscape:** Shopify utilizes Cloudflare for edge caching, while the Next.js ecosystem provides features like ISR (Incremental Static Regeneration). However, these require developer intervention or complex configuration.
  - **The Gap:** SMBs need an architecture that automatically caches reads at the edge and dynamically invalidates cache keys when state changes (e.g., inventory updates), combined with autonomous SEO pre-rendering that generates optimized static HTML for web crawlers without any user action.

  ## Design Doc
  - **Architecture Diagram:**
  ```mermaid
  graph TD
      A[Customer/Crawler] --> B(Edge Cache Layer)
      B -- Cache Miss --> C[Storefront Service]
      C --> D[(PostgreSQL)]
      E[Operations Agent] -->|Monitors| D
      E -->|Invalidates| B
      F[Marketing Agent] -->|Pre-renders| G(Static HTML)
      G --> B
  ```
  - **Mobile UX Flow:**
    - The owner logs into the OHC app (optimized for 375px view).
    - Under the 'Storefront' tab, the system displays a simple, clear status: "Storefront is active and optimized."
    - There are no technical settings for CDN, Cache, or SSR/ISR.
    - When the Marketing Agent completes a pre-rendering run, a toast notification or a small feed item appears: "Agent has optimized your storefront for search engines (SEO)."
    - Tapping this item shows a simple summary card (e.g., "3 new products are now visible on Google faster.") with a 'Dismiss' button.
  - **AI Agent Integration Points:**
    - **Operations Agent:** Monitors the central PostgreSQL ledger. Upon state change (e.g., an item is sold), it triggers an immediate cache invalidation for the specific entity key across the edge network.
    - **Marketing Agent:** Detects updates to storefront content or inventory and autonomously orchestrates a pre-rendering job.
  - **Key Design Decisions:**
    - Universal Edge Caching: All storefront read operations are automatically routed to a global edge cache to ensure high performance globally.
    - Agentic Pre-rendering: Automatically generates highly optimized static HTML injected with meta tags and structured data for web crawlers, boosting SEO without any user configuration.

  ## Implementation Prompt
  - Design and implement the edge caching middleware layer for storefront reads.
  - Implement the Operations Agent workflow to trigger cache invalidation based on database state changes.
  - Build the Agentic SEO Pre-rendering pipeline that generates and distributes static HTML.
  - Ensure the system is completely invisible to the user, with no configuration required in the UI.
  - *Do NOT prescribe specific caching providers (e.g., Redis vs. Cloudflare) or database schema changes here; allow the implementer to design the detailed technical architecture.*

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
