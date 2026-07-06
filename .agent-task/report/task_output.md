issue_title: "Research: Autonomous Supply Chain & Local Sourcing AI Mesh"
issue_description: |
  ## Problem Statement
  Makers and operators (e.g., Maya the Home Baker, Fatima the Food Cart Operator) constantly face the hidden friction of inventory depletion. Existing platforms tell them when they are out of stock of finished goods, but fail to bridge the gap to their raw materials. When Maya runs low on premium vanilla extract, or Fatima is short on local halal meats, they must manually track levels, search for suppliers, compare prices, and arrange deliveries. They need an AI-driven work assistant that proactively monitors depletion rates, identifies local or online suppliers, negotiates or prepares purchase orders, and presents a simple "Approve Purchase" notification on their phone before an out-of-stock crisis occurs.

  ## Research Report
  - **Market Context**: Traditional SMB inventory systems (like Katana or specific Shopify apps) track Bill of Materials (BOM) and raw goods, but they are passive—they require the owner to set up complex reorder points and manually execute supplier communications.
  - **The OHC Opportunity**: Integrating an "Autonomous Supply Chain Mesh" into the Operations AI Agent. By combining edge-calculated inventory depletion (from the POS) with proactive RAG (Retrieval-Augmented Generation) against a directory of local wholesalers and online vendors, OHC can execute procurement on behalf of the owner.
  - **Competitor Gaps**:
    - *Shopify*: Has basic purchase orders, but lacks AI-driven autonomous supplier outreach and negotiation.
    - *Square*: Good retail inventory, poor raw material sourcing automation.
    - *Dedicated ERPs (Katana, Odoo)*: Too complex, technical, and expensive for micro-businesses; they require dedicated operational staff to run.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
    A[Mobile App (Flutter)] -->|Inventory Depletion Event| B[OHC Backend (Go/PostgreSQL)]
    B -->|Trigger Eval| C[AI Operations Agent]
    C -->|Calculate Burn Rate| D[Inventory & BOM Ledger]
    C -->|Threshold Breached| E[Supplier Directory / Vendor APIs]
    C -->|Draft PO & Pricing| F[AI Finance Agent]
    F -->|Budget Approval| G[Owner Triage Feed]
    G -->|Owner Taps 'Approve'| A
    G -->|Dispatch Order| E
  ```

  ### Mobile UX Flow (375px)
  1. **Triage Feed Alert**: A high-priority card appears in the owner's feed: "Low Stock Predicted: Premium Vanilla Extract. Need by Friday."
  2. **AI Recommendation**: The card expands to show: "I found a local supplier (BakeSupply Co.) with stock for $45, or Amazon Business for $40 (delivery Thursday). I have drafted the purchase order."
  3. **One-Tap Execution**: The owner taps a prominent, touch-friendly `Approve Amazon ($40)` or `Approve BakeSupply ($45)` button.
  4. **Confirmation**: The Operations Agent confirms: "Ordered. I will track the delivery." No complex PO forms required.

  ### AI Agent Integration Points
  - **Operations Agent**: Tracks Bill of Materials (e.g., 1 Custom Cake = 2 cups flour + 1oz vanilla), calculates burn rate based on recent orders, and triggers the reorder workflow.
  - **Finance Agent**: Ensures the proposed purchase order fits within the current weekly cash flow and automatically logs the expense once approved.
  - **Communications Agent (Optional)**: If the supplier lacks an API, this agent can draft and send a localized email or SMS to the supplier requesting the items.

  ### Key Design Decisions
  - **Abstracted BOMs**: Hide the complexity of "Bill of Materials". The AI simply learns that selling a cake reduces flour.
  - **Zero-Friction Approval**: Procurement is reduced to a feed item with a binary yes/no, preventing paralysis.
  - **Multi-Tenant Security**: Supplier contacts and pricing agreements are strictly isolated per tenant using the `ohc:lock:{tenant_id}` pattern.

  ## Implementation Prompt
  **Target Persona**: Maya (Home Baker) & Fatima (Food Cart Operator)
  **Outcome**: Enable the OHC Operations Agent to proactively detect impending raw material shortages, source options, and present a one-tap purchase approval card to the owner.

  **Critical User Journey (CUJ)**:
  1. Owner logs into the OHC mobile app.
  2. The system simulates an order rush, depleting virtual "raw materials" based on a simplified recipe model.
  3. The Operations Agent detects a projected shortfall before the weekend.
  4. The Agent surfaces a card in the Triage Feed suggesting a restock order with two supplier options (one local, one shipped).
  5. Owner taps "Approve". The system records the pending inbound inventory and logs the planned expense.

  **Acceptance Criteria**:
  - Implement a simplified "Recipe/BOM" ledger linking finished goods to raw materials in PostgreSQL.
  - Develop the AI Operations Agent prompt and trigger logic to evaluate burn rate against stock levels.
  - Create the Triage Feed UI card in Flutter for supplier selection and approval.
  - Ensure the approval action correctly mutates both the inventory ledger (inbound stock) and finance ledger (expense).
  - Add robust E2E tests for the auto-procurement CUJ without requiring real supplier APIs (use a generic local adapter).

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
