issue_title: "[architecture] Autonomous Omni-Channel Catalog Sync Engine"
issue_description: |
  # Autonomous Omni-Channel Catalog Sync Engine

  ## Problem Statement
  Small business owners (like Priya the boutique owner) struggle to keep their inventory and product catalog synchronized across multiple sales channels. Managing in-store physical stock, online storefront inventory, Instagram Shopping, and local marketplaces simultaneously requires repetitive manual data entry, often leading to overselling or disjointed customer experiences. A unified, autonomous sync engine is necessary to give small businesses enterprise-level inventory integrity without technical complexity.

  ## Research Report
  **Market Analysis:**
  - Most e-commerce platforms (like Shopify or Wix) either require expensive third-party apps to synchronize across channels or only natively support a limited set of channels.
  - SMBs frequently over-sell items or lose out on potential revenue because updating product details (price, description, stock) on every platform is too slow.

  **Competitor Audit:**
  - **Shopify:** Provides a central catalog, but pushing to non-standard channels or handling real-time, low-latency syncs with third-party POS systems can be brittle or require paid apps.
  - **Square/Weebly:** Strong for local retail, but limited external omnichannel reach compared to dedicated platforms.
  - **Wix:** Basic sync capabilities, but often delayed and lacking AI intervention to handle mapping conflicts.

  **OHC Advantage:**
  OneHumanCorp’s "Operations Agent" (The Manager) can seamlessly and autonomously manage inventory sync across all linked channels. Using the AI to interpret and map product attributes (like "Color: Red" vs "Colour: Crimson"), OHC provides a zero-configuration, robust syncing solution.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Business Owner (Mobile)
      participant Catalog as OHC Master Catalog
      participant OpsAgent as Operations Agent
      participant Integrations as Omni-Channel Integrations
      participant Storefront as OHC Storefront
      participant Instagram as Instagram Shopping
      participant POS as Local POS

      Owner->>Catalog: Update Product (Price/Stock)
      Catalog->>OpsAgent: Emit CatalogChangeEvent
      OpsAgent->>OpsAgent: Map attributes per channel
      OpsAgent->>Integrations: Dispatch Sync Jobs
      Integrations->>Storefront: Update OHC Storefront
      Integrations->>Instagram: Update Meta Catalog API
      Integrations->>POS: Update Local POS via API
      Integrations-->>OpsAgent: Confirm Sync Success
      OpsAgent-->>Owner: Silent confirmation or feed alert on error
  ```

  ### Implementation Prompt
  **User Facing Outcome:**
  As a business owner, I want to update my product inventory or pricing exactly once in the OHC app, and have the AI autonomously ensure those changes are instantly reflected on my online storefront, my social media shopping channels, and my in-person POS system without any manual mapping or configuration.

  **Acceptance Criteria:**
  - Create a core `CatalogChangeEvent` schema and event bus integration.
  - Implement the `Operations Agent` logic to subscribe to catalog changes.
  - Build adapter interfaces for at least two target channels (e.g., OHC Internal Storefront and a mock external channel like Instagram Shopping).
  - Use AI attribute mapping to translate generic OHC catalog fields into channel-specific requirements.
  - Provide 100% unit test coverage for the sync logic and event handling.
  - Write a Playwright E2E test verifying a product update in the UI successfully propagates through the mock sync engine.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
