issue_title: "Agentic B2B Supply Chain & Autonomous Procurement Engine"
issue_description: |
  ## Title
  Agentic B2B Supply Chain & Autonomous Procurement Engine

  ## Problem Statement
  Small business operators like Jun (Location Manager) and Maya (Home Baker) spend hours manually auditing raw material inventory (coffee beans, flour, packaging) and cross-referencing it against upcoming demand (sales velocity, future bookings). When supplies run low, they must manually draft purchase orders or navigate complex vendor portals. Traditional systems like Shopify or Wix are purely B2C sales-focused and completely ignore the B2B supply chain side. Traditional ERPs (like NetSuite) are far too complex and expensive. This results in stockouts, missed sales, and operational anxiety.

  ## Research Report
  - **Competitor Analysis:** Shopify offers basic inventory tracking for finished goods but relies on complex third-party apps (like Stocky) for purchase orders, which still require manual forecasting. Wix has basic low-stock alerts but no procurement automation. Square tracks stock but doesn't autonomously draft vendor orders.
  - **OHC Opportunity:** By leveraging the 'Manager' (Operations) Agent and the 'Accountant' (Finance) Agent, OHC can create an invisible ERP. The system will automatically calculate the Bill of Materials (BOM) for sold items, predict raw material depletion, and proactively draft Purchase Orders (POs) to existing vendors, presenting them to the owner as a simple 1-tap approval card on their mobile feed.
  - **Data & Security Target:** SPIFFE/SPIRE identity mesh will ensure that internal agents accessing vendor APIs or sending emails on behalf of the tenant operate strictly within multi-tenant boundaries (row-level security in PostgreSQL).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Checkout Event Stream] -->|Webhook| B(Event Mesh)
      B --> C{BOM Resolution Engine}
      C -->|Deduct Raw Materials| D[Unified Inventory Ledger PostgreSQL]
      D --> E[Operations Agent: The Manager]
      E -->|Analyze Velocity & Lead Time| F{Replenishment Threshold Check}
      F -->|Threshold Met| G[Draft Purchase Order]
      G --> H[Action Required Queue]
      H --> I[Owner Mobile Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J -->|Email/API| K[B2B Vendor]
      I -->|1-Tap Approve| L[Finance Agent: Ledger Accrual]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed:** Top card displays a prominent warning: "Flour & Sugar running low based on next week's cake orders."
  - **Interaction:** Tapping the card opens a "Draft Purchase Order" view. It shows the suggested quantities based on predicted demand, the vendor (e.g., "Sysco" or "Local Mill"), and the total estimated cost.
  - **Action:** A primary "Approve & Send PO" button (≥ 44x44px touch target) and a secondary "Edit Quantities" button.
  - **Visual Design:** Apple/Ubiquiti-style clean cards. Translucent glass material for the modal overlay. Clear status tokens (e.g., a yellow warning dot for low stock, green for PO sent).

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Uses a time-series prediction prompt (via Gemini Pro) analyzing past sales data, seasonality, and upcoming calendar bookings to forecast when raw materials will hit zero. Drafts the PO based on vendor constraints (e.g., minimum order quantities).
  - **Finance Agent:** Estimates cash flow impact and checks if the tenant has sufficient balance/cash flow before suggesting the PO, warning the owner if the purchase might cause a cash crunch.

  ### Key Design Decisions
  - **Implicit Bill of Materials (BOM):** The system must allow users to loosely link finished products to raw materials (e.g., 1 Custom Cake = 1 unit Flour, 1 unit Sugar) via natural language during onboarding, rather than complex ERP configuration screens.
  - **Zero Trust Multi-Tenancy:** Each tenant's vendor API keys and financial projections are strictly isolated via RLS and SPIFFE/SPIRE-issued short-lived tokens when the agent executes the procurement action.

  ## Implementation Prompt
  **User-Facing Outcome:** As an operator (Jun or Maya), I open my OHC app and see an AI-drafted purchase order for my essential supplies exactly when I need them, preventing stockouts without me having to count inventory manually. I can approve the order to my supplier with one tap.
  **CUJ & Acceptance Criteria:**
  1. Given a configured tenant with a finished product ("Vegan Cake") and associated raw materials ("Vegan Flour", "Sugar").
  2. When multiple sales events reduce the virtual stock of raw materials below the AI-calculated threshold.
  3. The Operations Agent successfully triggers and generates a draft Purchase Order record in the `ActionRequiredQueue`.
  4. The owner logs into the UI (mobile 375px layout), sees the PO card, and taps "Approve & Send PO".
  5. The system transitions the PO to 'Sent' and mocks the API/Email dispatch to the vendor.
  6. Provide Playwright E2E tests covering the complete flow from sales event to PO approval, ensuring all UI elements have correct accessibility and target sizes, and verifying database persistence (no mocked UI data).

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
