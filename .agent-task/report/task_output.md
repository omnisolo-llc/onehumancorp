issue_title: "[architecture]_autonomous_inventory_prediction_engine"
issue_description: |
  # Architect Autonomous Predictive Inventory Engine

  ## Problem Statement
  Small business owners (like Priya the boutique owner and Maya the baker) suffer from "Procurement Paralysis". They constantly struggle with out-of-sync inventory across physical and online channels. They are forced to manually monitor stock levels, predict demand based on gut feeling, and manually generate purchase orders or restock alerts. Current platforms like Shopify treat inventory as a static database—alerting owners only *after* an item is out of stock. OHC needs an autonomous system that predicts stock-outs before they happen and proactively creates restock tasks.

  ## Research Report
  * **User Pain Point:** Manual inventory management and forecasting takes hours per week. Out-of-stock items lead directly to lost revenue. (Identified in OHC Market Gap Analysis as a top 10 pain point).
  * **Competitor Analysis:**
    * **Shopify:** Only provides basic "low stock" alerts. Predictive analytics require expensive third-party apps (e.g., Inventory Planner) that are too complex for non-technical users.
    * **Wix:** Basic manual inventory tracking. No predictive features.
  * **OHC Advantage:** The Operations Agent AI can analyze historical sales velocity, upcoming calendar events, and current stock levels to predict exact stock-out dates. It shifts inventory from a "database" to an "autonomous team member".

  ## Design Doc

  ### Key Design Decisions
  *   **Proactive rather than Reactive:** Shifts from simple "out of stock" alerts to predictive restock forecasting based on 30-day velocity.
  *   **Invisible Orchestration:** Restock actions require just 1-tap approval from the Action Feed rather than filling out complex purchase orders manually.
  *   **Multi-tenant Boundary Enforcement:** Inventory projections must never cross tenant bounds. Data isolation strictly enforces that one merchant's sales spikes do not affect another's predictive models.

  ### Entity-Relationship Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INVENTORY_ITEM : owns
      INVENTORY_ITEM ||--o{ INVENTORY_FORECAST : generates
      TENANT {
          string tenant_id PK
          string business_name
      }
      INVENTORY_ITEM {
          string item_id PK
          string tenant_id FK
          int current_stock
          float daily_sales_velocity
      }
      INVENTORY_FORECAST {
          string forecast_id PK
          string item_id FK
          date predicted_stockout_date
          int recommended_restock_qty
          string status
      }
  ```

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant Ledger as Universal Capacity & Inventory Ledger
      participant OpsAgent as Operations Agent
      participant ActionHub as Hub (Action Feed)
      participant UI as Mobile App (375px)

      Ledger-->>OpsAgent: Nightly Event: Trigger Stock Analysis
      OpsAgent->>Ledger: Query 30-day sales velocity per SKU
      OpsAgent->>OpsAgent: Calculate predicted stock-out date
      alt predicted_days_remaining < lead_time
          OpsAgent->>ActionHub: Create 1-Tap Restock Action
          ActionHub-->>UI: Push Notification & Feed Card
      end
  ```

  ### Mobile UX Flow
  1. The business owner opens the OHC app.
  2. The home screen "Activity Feed" displays a plain-language card: "Your Vegan Chocolate Cake mix will run out in 4 days. Approve restock order?"
  3. The card displays a large, thumb-friendly `[ Approve & Restock ]` button (optimized for 375px viewports).
  4. Tapping "Approve" triggers the backend agent to finalize the order or send the pre-drafted supplier email.

  ### AI Agent Integration
  * The **Operations Agent** acts as the core engine. It runs a scheduled task to pull sales velocity from the Universal Ledger.
  * It uses the LLM (or a simpler heuristic model) to calculate run-rate and factor in seasonality or upcoming booked events (e.g., a large catering order next week).

  ### Technical Integrity & Zero-Trust Security
  *   **Performance Targets:** End-to-end sync of a completed restock event must reflect on the mobile client in <200ms. Background forecasting tasks must process asynchronously without blocking the main event mesh.
  *   **Offline Support:** Restock approvals cached locally and synced via NATS JetStream upon reconnection.
  *   **Zero-Trust Identity:** All inter-agent communication (e.g., Operations Agent requesting purchase via Finance Agent) must use mTLS validated by SPIFFE/SPIRE with strict tenant ID matching.

  ## Implementation Prompt
  Implement the Autonomous Predictive Inventory Engine.
  * Define the data models for `InventoryForecast` and `RestockAction`. Ensure strict multi-tenant isolation.
  * Create a background worker that periodically analyzes sales velocity against current inventory levels.
  * If the predicted stock-out date falls within a configured threshold, the worker must generate a plain-language action card in the user's Action Feed.
  * Do not prescribe specific database schemas or API signatures; design for mobile-first simplicity.

  ## Priority
  P1 (High) - Directly solves a core operational pain point and establishes OHC's AI dominance over static competitors.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
