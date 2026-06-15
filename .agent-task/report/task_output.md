issue_title: "OHC Collective: Autonomous Neighborhood Synergy & Shared Loyalty Mesh"
issue_description: |
  # OHC Collective: Autonomous Neighborhood Synergy & Shared Loyalty Mesh (The OHC "Main Street" Engine)

  ## 1. Problem Statement
  Small business owners like **Maya (baker)**, **Carlos (handyman)**, and **Fatima (food cart operator)** operate as isolated islands. While they may physically be located in the same neighborhood or serve the same local customer base, they lack the technical infrastructure to leverage each other's success. Large retail chains dominate because of massive, unified loyalty networks and cross-promotional power.

  Current "Collective" solutions (like Shopify Collective) focus on e-commerce drop-shipping rather than local, physical neighborhood synergy. OHC needs an autonomous "Collective Mesh" that enables discovery, shared loyalty points, and cross-merchant rewards without any manual accounting or technical setup.

  ## 2. Research Report
  - **The "Main Street" Advantage**: OHC can create a "Network Effect for the Little Guy." By utilizing the Universal Buyer Identity (OHC Pay) and a cross-tenant loyalty ledger, OHC turns every merchant into a discovery node for every other local partner.
  - **Competitor Analysis**:
    - **Shopify Collective**: Requires complex desktop setup; limited to drop-shipping.
    - **Wix/Squarespace**: Completely isolated; no cross-tenant discovery exists natively.
  - **OHC Differentiation**: OHC's Collective is **Assistant-First**. The AI Promoter Agent identifies neighbors, suggests partnerships based on business "Vibe," and handles the "Loyalty Clearinghouse" logic invisibly.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Neighborhood_Mesh [Neighborhood Collective Mesh]
          C[Collective Hub]
          L[Shared Loyalty Ledger]
          S[Synergy Matchmaking Engine]
      end

      T1[Maya's Bakery] <-->|Sync| C
      T2[Carlos's Repairs] <-->|Sync| C

      S -->|Analyze Metadata| C
      C -->|Suggest Partnership| T1 & T2

      Buyer[OHC Buyer Identity] -->|Pay at T1| L
      L -->|Credit Points| Buyer
      Buyer -->|Redeem at T2| L

      FinanceAgent[AI Accountant] -->|Clearinghouse| L
  ```

  ### Data Model & Invariants
  - **Collectives**: Geofenced groups of tenants.
  - **Shared Loyalty Ledger**: A cross-tenant table recording point accumulation and redemption events, scoped by `collective_id`.
  - **Invariants**:
    - **Multi-Tenant Isolation**: RLS enforces that tenants see only shared collective events, never each other's private CRM or transaction data.
    - **Identity**: Cross-tenant point redemption must be verified via the Universal Buyer Identity and signed by the originating tenant's agent identity.

  ### AI Agent Integration
  - **The Promoter (Marketing AI)**: Periodically scans for OHC tenants within a Geohash. Suggests partnerships (e.g., "Maya, Carlos is nearby. Share loyalty points to boost your weekend sales?").
  - **The Accountant (Finance AI)**: Automatically reconciles cross-merchant point redemptions, ensuring accurate financial reporting for the "Clearinghouse" value transfer.

  ### Mobile UX Flow (375px First)
  1. **Collective Pulse (Dashboard Card)**: A translucent glass card: *"Carlos's Repairs and Fatima's Food Cart are nearby. Join the 'Downtown Artisans' Collective?"*
  2. **1-Tap Partnership**: Maya taps "Join." The AI configures the shared loyalty rule: "5 Points per $10 spent, valid at all partners."
  3. **Buyer Experience**: A customer pays Maya. The 375px receipt modal displays: *"✨ You earned 15 Neighborhood Points! Spend them at Carlos's Repairs nearby."*
  4. **The Discovery Widget**: Maya's storefront includes a translucent footer showing partner businesses with 1-tap booking links.

  ## 4. Implementation Prompt
  **To the Implementer Swarm:**
  Implement the backend services and mobile UI for the "Neighborhood Collective Discovery & Shared Loyalty Mesh."
  - **Backend**:
    1. Implement the `ohc_collective_loyalty_balance` ledger that supports cross-tenant point accumulation and redemption.
    2. Build the `Synergy Matchmaking Engine` as a service that uses geohashing and business categories to suggest partnerships.
    3. Implement the `Finance AI` clearinghouse logic to record value transfers when points earned at Tenant A are spent at Tenant B.
  - **Frontend (Mobile 375px)**:
    1. Build the "Collective Pulse" dashboard card using macOS-style Translucent Glass materials.
    2. Implement the "Neighborhood Discovery Widget" for the public storefront, ensuring 100% mobile parity.
  - **Verification**: Ensure all cross-tenant ledger entries are auditable and strictly isolated by `collective_id`.

  ## 5. Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
