issue_title: "Implement The Operations Manager Agent - Autonomous Order and Inventory Resolution"
issue_description: |
  # Research Report: The Operations Manager Agent

  ## Problem Statement
  Small business owners (like Maya the Baker or Priya the Boutique Owner) often face logistical nightmares when managing inventory and orders across multiple channels (online and in-store). If an item sells out in-store, the online storefront needs immediate updating to prevent double-booking. When issues arise (e.g., an order is placed but inventory is low), the owner is forced to manually reconcile the discrepancy, which is error-prone and time-consuming. They need an "Operations Manager" to proactively identify and resolve these issues.

  ## Research Report
  - **Market Context:** Traditional platforms like Shopify rely on users to manually update inventory or purchase expensive third-party apps for multi-channel synchronization. Even with these tools, the system is reactive—it warns the user but doesn't propose a solution or automatically draft a restock order or customer notification.
  - **The OHC Opportunity:** By introducing an "Operations Manager" AI agent, OHC can shift from a reactive tool to a proactive assistant. This agent will autonomously monitor inventory levels, detect potential shortfalls against pending orders, and present the owner with actionable resolutions directly on their mobile device.
  - **Competitor Gaps:**
    - *Shopify:* Requires manual intervention or complex app setups to handle out-of-stock scenarios gracefully.
    - *Wix/Squarespace:* Basic inventory tracking; no proactive problem resolution.

  ## Design Doc
  ### Architecture
  - **Event Triggers:** The agent listens for `OrderPlaced`, `InventoryUpdated`, and `InStoreSaleProcessed` events via an internal message bus (e.g., Redis Pub/Sub).
  - **Logic Engine:** When an event is triggered, the Operations Manager queries the central PostgreSQL ledger to compare current inventory against pending orders.
  - **Resolution Generation:** If a discrepancy is found (e.g., inventory < required for pending orders), the agent uses LLM context to draft a resolution strategy (e.g., "Draft restock email to supplier" or "Draft apology/refund message to customer").
  - **Notification System:** The proposed resolution is pushed to the user's Agent Feed as an "Action Card".

  ### Mobile UX Flow (375px)
  1. An inventory conflict occurs.
  2. The owner receives a push notification: "Action Required: Inventory conflict for Red Dress."
  3. The owner opens the app to see an Action Card detailing the issue and proposing a solution (e.g., "Draft message to customer offering an alternative or refund").
  4. The owner taps "Approve" (which sends the message) or "Edit" (to modify the response).
  5. The UI must be fully responsive, with clear typography and touch targets >= 44x44px.

  ### AI Integration Points
  - **Intent/Context:** The LLM is used to formulate the plain-language explanation of the issue and draft the customer communication or supplier restock email based on the user's saved preferences/templates.

  ## Implementation Prompt
  **Feature Name:** The Operations Manager Agent - Inventory Conflict Resolution
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** Priya is automatically notified when an inventory conflict occurs and is provided with a drafted resolution (e.g., customer communication) that she can approve with a single tap.

  **Next Actions:**
  1. Implement an event listener for inventory/order state changes.
  2. Develop the logic to detect inventory shortfalls against pending orders.
  3. Integrate the LLM to generate resolution drafts.
  4. Build the mobile-first Action Card UI for the Agent Feed to present the issue and proposed resolution.

  ## Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
