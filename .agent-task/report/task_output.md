issue_title: "[architecture]_multimodal_ai_business_ingestion_engine"
issue_description: |
  # Architecture: Multimodal AI Business Ingestion & Zero-Touch Migration Engine

  ## Problem Statement
  The single largest point of friction for established micro-businesses (like Maya the baker or Fatima the food cart owner) when adopting a new platform is data entry. Launching a business "in under 10 minutes" is impossible if a user has to manually type out 40 menu items, configure pricing variants, and upload photos from their camera roll on a mobile keyboard. Small business owners hate typing. If they already have an Instagram feed or a chalk board menu, forcing them to recreate their catalog from scratch is a failure of the platform.

  ## Research Report
  *   **Competitor Analysis:**
      *   **Shopify:** Relies heavily on CSV imports or expensive third-party apps (e.g., Cart2Cart). CSVs are fundamentally incompatible with a mobile-first user experience.
      *   **Wix/Squarespace:** Offer basic URL text-scraping for initial setup, but lack the ability to structure complex e-commerce variants from unstructured visual data.
      *   **GoDaddy:** Uses AI to generate generic stock-photo sites, but cannot ingest a user's actual existing business assets natively.
  *   **The Market Gap:** There is no platform that allows a user to point their phone camera at a physical paper menu, or paste an Instagram handle, and have an AI instantly construct a fully relational, multi-variant, inventory-tracked database and live storefront.
  *   **Persona Impact:** For Fatima, who has limited English and relies on a physical printed menu, taking a single photo eliminates hours of stressful UI navigation. For Maya, syncing her existing Instagram portfolio instantly creates her shoppable catalog.

  ## Design Doc

  ### Key Design Decisions and Why
  1.  **Streaming "Glassy" Feedback:** Instead of a long loading spinner, the KAIROS Orchestrator pushes parsed items to the mobile UI in real-time via WebSockets. The user sees their products magically appearing line-by-line, which builds extreme product delight and trust.
  2.  **Optimistic Generation with Anomaly Flagging:** If the Vision model cannot read a price due to glare, it will optimistically infer a market-rate price (e.g., $4.00 for a cupcake) but flag it visually in the UI for mandatory review, rather than failing the import job.
  3.  **Ephemeral Data Sandboxing:** All uploaded images and scraped HTML are processed in secure, ephemeral Bazel-backed K8s namespaces. The data is destroyed immediately after the structured ledger commit to ensure Zero-Trust multi-tenant isolation.

  ### AI Agent Integration Points
  *   **The Onboarding Agent:** Orchestrates the flow. It accepts the multimodal payload (image buffer or URL) and delegates to the Data Ingestion Agent.
  *   **Data Ingestion Agent (Multimodal LLM):** Processes the image/HTML, extracting titles, prices, descriptions, and categories.
  *   **Marketing Agent:** Runs a secondary pass on the extracted data to enhance descriptions (e.g., turning "Choc Cake - $20" into "Decadent Double Chocolate Custom Cake").
  *   **Operations Agent:** Maps the extracted items to the underlying Postgres `TenantProductLedger` and initializes stock levels.

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant M as Mobile App (375px)
      participant K as KAIROS Orchestrator
      participant DIA as Data Ingestion Agent
      participant MA as Marketing Agent
      participant DB as Postgres Tenant Ledger

      M->>K: POST /ingest (Image/URL)
      K->>DIA: Trigger LangGraph Extraction Cycle
      loop Real-time Extraction
          DIA-->>K: Yield Extracted Item (Partial)
          K-->>M: WebSocket Push (Item Skeleton UI)
      end
      DIA->>MA: Pass raw items for SEO/Desc Enhancement
      MA->>DB: Commit Structured Catalog to Tenant DB
      DB-->>K: Acknowledge Commit
      K-->>M: Return Final OK & Launch Storefront
  ```

  ### Mobile UX Flow (375px Viewport)
  1.  **Input Screen:** Clean, translucent glass UI. "Already selling? Let the AI build your store." Two large buttons: "Take Photo of Menu" or "Connect Instagram".
  2.  **Processing Screen:** A blurred background of their image. Translucent UniFi-style cards begin sliding up from the bottom of the screen one by one as the AI extracts them (e.g., "Found: Vegan Chocolate Chip Cookie - $3.50").
  3.  **Review Screen:** A Tinder-style swipeable card stack. The user can quickly swipe right to approve an item or tap to edit a flagged anomaly (e.g., missing price).
  4.  **Activation:** The final screen says "Your business is live." with a confetti animation and their custom short-link.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your objective is to implement the "Multimodal AI Business Ingestion Engine" core logic and API.
  **CUJ (Critical User Journey):** A user uploads a picture of a handwritten menu or an Instagram URL. The system must asynchronously parse this unstructured data, structure it into OHC Product entities, and stream the results back to the client.
  **Acceptance Criteria:**
  1. Create a secure, multi-tenant HTTP endpoint `/api/v1/onboarding/ingest` that accepts either an image payload or a URL.
  2. Implement an agentic workflow using the KAIROS Orchestrator that passes the payload to a multimodal LLM provider (e.g., MiniMax/GPT-4o).
  3. The prompt to the LLM must enforce a strict JSON output schema representing products (title, price, description, category).
  4. Implement WebSocket or Server-Sent Events (SSE) to stream parsed items back to the client in real-time.
  5. Do not hardcode specific database schemas or tables; assume you are interacting with an abstract `CatalogRepository` interface that you will define.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
