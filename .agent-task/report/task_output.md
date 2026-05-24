issue_title: "[Architecture] Autonomous Dynamic Pricing & Promotions Engine"
issue_description: |
  # Issue Brief: Autonomous Dynamic Pricing & Promotions Engine

  ## Problem Statement
  Small business owners (SMBs) like Maya (baker) and Leo (music tutor) struggle with optimizing their prices. They either underprice their services or miss out on revenue during peak demand times. Running sales or offering dynamic discounts is a manual, spreadsheet-driven process that causes anxiety. They lack the data and time to do "yield management" like large airlines or hotel chains, leaving significant revenue on the table. They need an invisible capability that optimizes their pricing and auto-generates personalized promotions without requiring a degree in economics.

  ## Research Report
  - **Competitor Landscape**:
    - Shopify requires complex 3rd-party apps for dynamic pricing or simple discount code management.
    - Wix offers basic manual coupon codes.
    - Neither offers autonomous yield management or AI-driven localized promotions.
  - **Market Opportunity**: SMBs typically guess their pricing. Implementing enterprise-grade yield management (adjusting price based on demand, time, or inventory) autonomously can increase revenue by 10-20% with zero extra work.
  - **AI Differentiation**: OHC's Finance and Marketing AI Agents coordinate to monitor inventory velocity and calendar booking density. The system automatically adjusts prices within user-approved guardrails and surfaces 1-tap localized promotions to drive sales during slow periods.

  ## Design Doc
  ### High-Level Architecture
  - **Trigger**: The background Finance AI agent continuously monitors `Ledger`, `Inventory`, and `Booking Calendar` metrics.
  - **Agent Action**:
    - Identifies low-demand periods (e.g., Leo's Tuesday morning slots are empty) or high-demand scarcity (e.g., Maya's weekend custom cakes are almost sold out).
    - Calculates an optimized price adjustment or promotional offer within the merchant's predefined bounds.
    - Generates a targeted marketing message (via Marketing AI agent) if a promotion is created.
  - **Data Model & Invariants**:
    - `PricingConfig`: Multi-tenant scoped rules defining minimum price floor and maximum price ceiling.
    - `PromotionLedger`: Immutable log of generated promotions and their conversion rates.
    - Strict tenant isolation ensures Maya's pricing data never influences Carlos's pricing model.

  ### Architecture Diagram (Mermaid.js)

  **1. Entity-Relationship Diagram (Data Model)**
  ```mermaid
  erDiagram
      MERCHANT ||--o{ PRICING_CONFIG : "configures"
      MERCHANT ||--o{ INVENTORY_ITEM : "sells"
      MERCHANT ||--o{ PROMOTION_LEDGER : "owns"

      INVENTORY_ITEM ||--o{ PRICING_CONFIG : "has bounds"
      INVENTORY_ITEM ||--o{ PROMOTION_LEDGER : "promoted in"

      PRICING_CONFIG {
          string tenant_id FK "Strict tenant isolation"
          float min_price_floor
          float max_price_ceiling
          boolean auto_apply_promotions
      }

      PROMOTION_LEDGER {
          string id PK
          string tenant_id FK
          string item_id FK
          float discounted_price
          datetime start_time
          datetime end_time
          float conversion_rate
      }
  ```

  **2. Sequence Diagram (User Flow)**
  ```mermaid
  sequenceDiagram
      participant DemandMesh as Demand & Inventory Mesh
      participant FinanceAgent as Finance AI Department
      participant MarketingAgent as Marketing AI Department
      participant User as Merchant (Mobile)

      DemandMesh->>FinanceAgent: Stream booking density & inventory velocity
      FinanceAgent->>FinanceAgent: Analyze yield gaps & calculate optimal price/promo
      FinanceAgent->>User: Push 1-tap promotion approval to Activity Feed
      User->>FinanceAgent: Approve promotion
      FinanceAgent->>MarketingAgent: Generate targeted social/email campaign
      MarketingAgent-->>DemandMesh: Drive traffic to new promotion
  ```

  ### Mobile UX Flow (375px First)
  1. **Activity Feed Nudge**: User receives a card: "Next Tuesday is looking slow. Offer a 15% discount for morning bookings?"
  2. **Review Screen**: The user sees the expected revenue impact, the proposed updated price, and the auto-generated marketing post.
  3. **Action**: User taps "Approve" (instantly updates pricing and schedules the post) or "Dismiss". No complex forms or percentage math required.

  ## Implementation Prompt
  Implement the "Autonomous Dynamic Pricing & Promotions Engine". Build the background worker that monitors inventory and calendar density. Design the `PricingConfig` schema with multi-tenant guardrails. Create the event bridge that triggers the Marketing AI Department upon user approval of a generated promotion via the mobile Activity Feed. Ensure all pricing math guarantees precision and no floating-point errors.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
