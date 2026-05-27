issue_title: '[architecture] AI Predictive Inventory Restocking Engine'
issue_description: |
  # Problem Statement
  Small business owners, particularly those selling physical products like Priya (boutique owner) and Fatima (food cart operator), currently manage their inventory manually (often in spreadsheets), resulting in out-of-sync catalogs, stockouts, and over-ordering. They lack the time, data analysis skills, and predictive tools to optimize their stock levels, making inventory management a major friction point.

  # Research Report
  An audit of our codebase, docs (especially `docs/research/small_business_platform_research_report.md`), and competitor platforms (Shopify, Wix) reveals a significant market gap. Competitors offer manual inventory management or rely on complex, third-party integrations for predictive analytics.
  OHC currently has a major gap: **AI Predictive Restocking**. By utilizing an AI-driven, invisible operations engine to forecast sales velocity and auto-draft supplier orders, OHC can realize its "Unfair Advantage" of invisible autonomy, directly resolving a primary pain point that accounts for 40% of small business friction.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  graph TD;
      MobileClient[Mobile App / OHC Storefront] --> API[Rust Server / API];
      API --> Postgres[(Postgres - Ledger/Inventory)];
      API --> SyncEngine[OHC-SIP Sync Engine];
      SyncEngine --> AutoDream[AutoDream Vector DB];
      Postgres --> EventBus[Event Pub/Sub];
      EventBus --> OPO[Operations Dept Agent];
      OPO --> Predictor[AI Velocity Predictor];
      Predictor --> AutoDraft[AutoDraft Supplier Order];
      AutoDraft --> API;

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
      class MobileClient,API,Postgres,SyncEngine,AutoDream,EventBus,OPO,Predictor,AutoDraft premium;
  ```

  ## Mobile UX Flow (375px)
  1. **Home Screen Notification Card**: A translucent, glassmorphic card surfaces: "Low Stock Alert: Vegan Chocolate Cake (Est. stockout in 2 days)".
  2. **Action Card Expansion**: Tapping the card opens a detail view with a predicted sales graph.
  3. **1-Tap Action**: A primary, high-contrast button "Approve Restock Order" automatically sends the pre-drafted PO to the supplier. No complex spreadsheet forms.

  ## AI Agent Integration Points
  - **Operations Department**: Monitors the distributed state machine for inventory changes.
  - **Predictive Velocity Model**: Analyzes historical sales data via the AutoDream pipeline and embedded vector truth.
  - **Auto-Draft Agent**: Synthesizes the supplier order context and creates draft POs.

  ## Key Design Decisions
  - **Zero-Trust SPIFFE Identity**: Operations agents accessing inventory data are verified via SPIRE to ensure isolated tenant access.
  - **Mobile-First Glassmorphism**: High-fidelity, simple 1-tap UX ensures users like Fatima can manage stock without understanding the underlying math.
  - **Hybrid RAG Strategy**: Local SQLite state synchronizes seamlessly with the Cloud Swarm Orchestration to update velocity models without interrupting the user.

  # Implementation Prompt
  Implement the AI Predictive Inventory Restocking Engine. Create the necessary data models (e.g., InventoryItem, StockForecast, SupplierOrder) ensuring strict multi-tenant isolation. Integrate the event flow with the KAIROS Orchestration Hub so the Operations Department agent can subscribe to inventory changes, predict stockouts, and automatically generate SupplierOrders. The feature must expose a GraphQL/REST endpoint that the mobile UI can poll or receive push notifications from to display the 1-Tap Restock card. Do not focus on UI code; implement the backend services, agent routing, and data layer.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
