issue_title: "Architect and Implement Neighborhood Collective Discovery & Shared Loyalty Mesh"
issue_description: |
  ## Mission Queue Protocol: Neighborhood Collective Mesh

  ### Title: Architect and Implement Neighborhood Collective Discovery & Shared Loyalty Mesh

  ### Problem Statement
  Small business owners like **Maya (baker)**, **Carlos (handyman)**, and **Fatima (food cart operator)** operate as isolated islands. They lack the technical infrastructure to leverage each other's success through neighborhood synergy. Large retail chains dominate because of massive, unified loyalty networks. OHC needs an autonomous "Collective Mesh" that enables discovery, shared loyalty points, and cross-merchant rewards with zero manual accounting or technical setup.

  ### Research Report
  - **Shopify Collective**: Focused on e-commerce dropshipping, not local neighborhood synergy. High configuration burden.
  - **OHC Advantage**: Network effect for the "Little Guy." By utilizing the Universal Buyer Identity (OHC Pay), we allow separate tenants to form a "Collective" where AI agents proactively suggest shared loyalty programs.
  - **Discovery**: Use H3 geohashing for efficient spatial indexing of local businesses.
  - **Clearinghouse**: Points earned at Merchant A but spent at Merchant B require a virtual clearinghouse to balance the books for tax/payout purposes.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant OwnerA as Maya (Bakery)
      participant Promoter as AI Promoter Agent
      participant Mesh as Hybrid Event Mesh
      participant OwnerB as Carlos (Handyman)
      participant Accountant as AI Accountant Agent

      Promoter->>Promoter: Scan neighbors (H3 Geohashing)
      Promoter->>Mesh: Event: "Partnership Opportunity Detected"
      Mesh->>OwnerA: UI Card: "Form Collective with Carlos?"
      OwnerA->>Mesh: 1-Tap Approve: "Invite Carlos"
      Mesh->>OwnerB: Notification: "Maya invited you to 'Main St Collective'"
      OwnerB->>Mesh: 1-Tap Approve: "Join"
      Note over Mesh: Collective Activated
      Accountant->>Accountant: Monitor shared loyalty redemptions
  ```

  #### Data Model & Invariants
  - **COLLECTIVE**: Defines the neighborhood boundary (Geohash + Radius).
  - **COLLECTIVE_MEMBER**: Tenant membership status (ACTIVE | PENDING).
  - **SHARED_LOYALTY_LEDGER**: Cross-tenant value transfers.
  - **Invariants**:
    - Loyalty balances are scoped to the `CollectiveID`.
    - Zero-Jargon UI: "Neighborhood Group" instead of "Reciprocal API."
    - Strict Isolation: Maya cannot see Carlos's private sales data.

  #### Mobile UX Flow (375px First)
  1. **The Neighborhood Pulse (Dashboard Card)**: Translucent glass card: "There are 4 OHC businesses in your area. Form a collective?"
  2. **The "Partner Match" Sheet**: Tapping card shows neighbor cards with "Vibe" matches.
  3. **Shared Loyalty Setup**: AI suggests: "Give 5 'Main St' points per $10 spent. Valid at all 3 shops."
  4. **Buyer Experience**: Receipt shows: "You earned 15 Main St Points! Spend them at Carlos's Repairs nearby."

  #### AI Agent Integration
  - **The Promoter**: Periodically scans for OHC tenants within a 5-mile radius.
  - **The Accountant**: Handles the "Loyalty Clearinghouse" logic, ensuring value transfers are recorded for tax purposes.

  ### Implementation Prompt
  Implement the backend services and 375px mobile UI for the **Neighborhood Collective Discovery & Shared Loyalty Mesh**.
  - **Backend**: Implement `Collective` and `SharedLoyaltyLedger` entities with strict multi-tenant RLS.
  - **Discovery**: Create a geohashing-based discovery service (using H3 or PostGIS) that AI agents can query to find nearby OHC tenants.
  - **Clearinghouse**: Develop the logic for cross-tenant loyalty redemption and value transfer recording.
  - **Mobile UI**: Build the 375px "Neighborhood Widget" and "Neighbor Pulse" dashboard cards using OHC translucent glass tokens.
  - **CUJ (Critical User Journey)**:
    1. Tenant A invites Tenant B to a Collective.
    2. Tenant B accepts on mobile.
    3. A Buyer earns points at Tenant A.
    4. The Buyer redeems those points for a discount at Tenant B's checkout.
    5. The Shared Ledger reflects the value transfer.
  - **Acceptance Criteria**:
    - Passes the 30-second "Grandmother Test."
    - Cross-tenant queries are secured via SPIFFE/SPIRE.
    - Full E2E Playwright test coverage for the loyalty redemption loop.

  ### Priority: P1
  ### Estimated Scope: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
