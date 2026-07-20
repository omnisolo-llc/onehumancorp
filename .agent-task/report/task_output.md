issue_title: "Implement Autonomous Local SEO & Discoverability Agent"
issue_description: |
  # Research Report: Autonomous Local SEO & Discoverability Agent

  ## Title
  Autonomous Local SEO & Discoverability Agent

  ## Problem Statement
  Small business owners consistently report that getting found on Google is a major pain point ("SEO Mystery" ranks highly among SMB complaints). Terms like "meta tags," "schema markup," and "sitemaps" are intimidating and technical. Owners just want their bakery or handyman service to appear when locals search for them. Existing platforms (Shopify, Wix) offer SEO tools, but they require manual configuration, ongoing maintenance, and an understanding of SEO best practices, which defeats the purpose of an "easy" builder for non-technical users.

  ## Research Report
  - **Market Gap:** While traditional builders provide the *tools* for SEO (e.g., fields to enter title tags), they do not *do* the SEO. SMBs are left with blank fields and no idea what to write.
  - **Competitor Analysis:**
    - *Shopify/Wix:* Require users to manually write meta descriptions and alt text. They often push users to buy expensive third-party apps for advanced SEO.
    - *Durable/AI Builders:* Generate an initial, often generic, set of copy but lack ongoing, dynamic SEO management based on real-time business changes (like new products or updated service areas).
  - **OHC Opportunity:** SEO should be an invisible, autonomous process. As the owner updates their inventory, services, or business hours, the Marketing Agent should autonomously update the site's meta tags, generate rich snippets, and ping search engines without any manual intervention.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Action: Add Product/Service] --> B(Event Bus)
      B --> C[The Promoter Agent]
      C -->|Analyze Content & Location| D[LLM Generation Engine]
      D -->|Generate Meta Tags & Schema| E[SEO Metadata Store]
      E --> F[Edge Cache / Pre-rendering Engine]
      F --> G[Search Engine Crawlers]
  ```

  ### Mobile UX Flow (375px)
  1. **Zero Configuration Setup:** During onboarding, the system asks for the business name, category, and location. That's it. There is no dedicated "SEO Settings" tab by default.
  2. **Agent Feed Notification:** When the owner adds a new "Vegan Chocolate Cake", The Promoter agent surfaces a card in the feed: "I've optimized your new cake for local searches in Austin. I added keywords like 'vegan bakery near me'."
  3. **One-Tap Approval (Optional):** The owner can tap "Looks Good" or just ignore it, and the agent proceeds autonomously.

  ### AI Agent Integration Points
  - **The Promoter (Marketing/SEO Agent):** Listens for `EntityCreated` and `EntityUpdated` events (products, services, business details). It uses the LLM to generate highly relevant, localized title tags, meta descriptions, and JSON-LD schema markup.
  - **Edge Cache Integration:** The generated metadata is instantly pushed to the edge (or pre-rendered) so that the next Googlebot crawl sees the perfectly optimized content.

  ### Key Design Decisions
  - **Invisibility First:** Hide all technical SEO jargon. No more "Edit Meta Description" boxes on the main product edit screen.
  - **Localized Focus:** For most SMBs, local SEO (Google Maps, "near me" searches) is far more important than global SEO. The agent prioritizes local keywords based on the business address.

  ## Implementation Prompt
  Implement the "Autonomous Local SEO Agent". Create a background worker that listens for changes to products or the main business profile. When triggered, use the configured LLM to generate localized title tags, meta descriptions, and valid JSON-LD schema for the business. Store this metadata in the database and ensure the frontend (Tauri/Next.js) automatically injects it into the `<head>` of the public storefront pages. The owner should not have to configure anything for this to work.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
