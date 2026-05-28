issue_title: "[Architecture] Embedded Zero-Latency Analytics & Attribution Engine"
issue_description: |
  # Problem Statement
  Small business owners like Priya (Boutique owner) and Maya (Baker) need to understand what's driving their sales, but they do not have the time, technical skills, or patience to set up, configure, and interpret complex tools like Google Analytics or Meta Pixel. When Priya runs an Instagram campaign, she needs to instantly know how many sales resulted from it without leaving the OneHumanCorp (OHC) app. Currently, OHC relies on external tracking integrations which are fragile, difficult to configure on mobile, and present data in overly complex dashboards that fail the "grandmother test." OHC needs a native, zero-configuration, privacy-first analytics engine that surfaces actionable plain-language insights instantly on their mobile devices.

  # Research Report
  *   **Current Analytics Gaps:** Standard web analytics are disconnected from the transaction ledger. They require manual tag management, cookie consent banners (which lower conversion), and separate dashboard logins.
  *   **Competitor Analysis:**
      *   *Shopify:* Has robust built-in analytics, but the mobile dashboard can be overwhelming with dense tables.
      *   *Squarespace:* Offers basic built-in analytics but lacks deep attribution tied directly to POS and offline sales.
      *   *Google Analytics 4:* Overwhelming for non-technical users, requires custom implementation for accurate e-commerce tracking.
  *   **Discovery:** OHC must architect an embedded analytics event bus that securely unifies storefront interactions (views, clicks) with backend transactional events (checkouts, tap-to-pay) into a single tenant-isolated time-series data store. This engine must operate without cookies where possible (server-side tracking) and instantly summarize performance.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Customer (Web/Mobile)
      participant Edge as OHC Edge / Storefront
      participant EventBus as Real-Time Event Bus
      participant TSDB as Multi-Tenant Time-Series DB
      participant AI as Marketing/Ops Agent
      participant App as Merchant OHC App (Mobile)

      User->>Edge: View Product / Add to Cart / Checkout
      Edge->>EventBus: Publish anonymized interaction event
      EventBus->>TSDB: Ingest & aggregate event (Tenant Isolated)

      loop Background Analysis
          AI->>TSDB: Query performance metrics
          AI-->>AI: Generate plain-language insights
      end

      App->>AI: Request daily briefing
      AI->>App: Deliver simple metric cards & advice
  ```

  ### Mobile UX Flow (375px First)
  *   **Home Dashboard (375px):**
      *   Top of the screen features a Translucent Glass card showing "Today's Pulse".
      *   Instead of complex line graphs, it displays bold, simple metrics: "$450 Sales (↑ 12% vs yesterday) — 45 Visitors".
  *   **Insight Drill-down:**
      *   Tapping the card opens a clean, vertically scrollable list.
      *   "Where your customers came from: 60% Instagram, 30% Search, 10% Direct."
      *   No jargon like "Bounce Rate" or "Sessions". Uses plain English: "People left without buying" or "Store visits".
  *   **AI Agent Intervention:**
      *   A prominent message from the Marketing Agent: "Your new Vegan Cake post is driving 3x more traffic today. Should I boost it for $10?" with a simple "Yes/No" button.

  ### Key Design Decisions
  *   **Zero-Configuration:** Analytics tracking is hardcoded into the OHC storefront and checkout primitives. No tags or snippets required.
  *   **Privacy-First Server-Side Tracking:** Rely heavily on server-side event generation (e.g., when a cart is created in the backend) rather than purely client-side Javascript, improving accuracy and reducing reliance on third-party cookies.
  *   **Plain-Language Abstraction:** The raw data (time-series) is abstracted by AI agents before being presented to the user. The database stores events; the UI only shows synthesized insights.
  *   **Multi-Tenancy:** The Time-Series DB must strictly isolate events by `tenant_id` to prevent cross-contamination of merchant data.

  ### AI Agent Integration Points
  *   **Marketing Agent:** Analyzes the real-time traffic and conversion data to suggest actionable marketing spend or social media actions directly in the merchant's unified inbox.
  *   **Operations Agent:** Correlates traffic spikes with inventory levels to warn the merchant if a trending product is about to sell out.

  # Implementation Prompt
  Implement the Embedded Zero-Latency Analytics & Attribution Engine for OneHumanCorp. Build a real-time event ingestion pipeline that captures storefront interactions and checkout events securely, storing them in a tenant-isolated time-series database. Expose a gRPC/API layer that allows the AI Marketing Agent to query these aggregates with sub-second latency. Design the mobile-first (375px) UI components using the OHC design system (Glassmorphism cards) to display these metrics in plain, jargon-free English. The system must require zero configuration from the merchant and seamlessly blend online storefront traffic with in-person POS sales data.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
