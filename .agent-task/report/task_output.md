issue_title: "Implement Autonomous Pre-Order & Waitlist Engine for High-Demand Drops"
issue_description: |
  # Research Report: Autonomous Pre-Order & Waitlist Engine for High-Demand Drops

  ## 1. Problem Statement
  Small business owners such as Fatima (Food Cart Operator) and Priya (Boutique Operator) often experience demand spikes that outstrip their current capacity. They are forced to manually manage waitlists via DMs or scattered notebooks, or accept pre-orders without a unified system to handle fulfillment timing. This manual tracking leads to lost sales, frustrated customers due to missed updates, and operational chaos when the supply finally arrives or capacity opens up.

  ## 2. Research Report
  - **Market Context**: Traditional platforms like Shopify or Wix require users to either install third-party waitlist apps (e.g., Back in Stock) or manually change product statuses to "Out of Stock." Pre-orders on these platforms often require complex inventory overrides and do not autonomously notify customers or shift fulfillment schedules based on real-world constraints.
  - **The OHC Opportunity**: By integrating a native Pre-Order and Waitlist Engine powered by the AI Operations and Marketing agents, OHC can seamlessly convert missed demand into future revenue. The AI agents can autonomously manage the waitlist queue, process delayed deposits, and notify customers exactly when their item or spot is available, all without the owner lifting a finger.
  - **Competitor Gaps**:
    - *Shopify*: Relies heavily on paid third-party apps for waitlists, leading to an inconsistent UX and added "App Tax."
    - *Wix*: Basic inventory management; lacks autonomous customer engagement when stock replenishes.
    - *Square*: Good POS but fragmented online pre-order capabilities without AI-driven customer communication.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Product_Waitlist`: Links a Customer to a Product variant with a state (pending, notified, converted, expired).
  - `PreOrder_Schedule`: Defines the target fulfillment window or capacity constraints for pre-ordered items.
  - `Inventory_Allocation`: Distinguishes between immediate stock and future allocated stock to prevent overselling on the primary ledger.

  ### AI Integration Points
  - **Operations Agent (The Manager)**: Monitors inventory levels and capacity. When an item hits zero, it autonomously toggles the product to "Waitlist/Pre-Order" mode. When stock arrives, it computes fulfillment routing and triggers the Marketing Agent.
  - **Marketing Agent (The Promoter)**: Autonomously drafts and sends personalized SMS/Email notifications to waitlisted customers containing a direct 1-tap checkout link when their spot opens up, prioritizing VIP customers based on past purchase history.

  ### Mobile UX Flow (375px first)
  1. **Customer View**: When a product is sold out, the "Buy Now" button seamlessly transitions to a "Join Waitlist" or "Pre-Order Now" button with a clear expected date, capturing their email/phone via a 1-tap form.
  2. **Owner View (Dashboard)**: The owner feed displays an actionable insight card: "The Pre-Order engine captured 45 waitlist signups for the Red Dress. You have $2,250 in pending demand. Tap to draft a restock order."

  ## 4. Implementation Prompt
  **Feature Name**: OHC Autonomous Pre-Order & Waitlist Engine
  **Target Persona**: Priya (Boutique Operator) / Fatima (Food Cart Operator)
  **Outcome**: When an item sells out, the system autonomously transitions to capturing waitlist signups or pre-orders. When stock is replenished, the AI agents automatically notify customers and process the pending revenue without manual intervention.

  **Next Actions**:
  1. Implement the core Data Models (`Product_Waitlist`, `PreOrder_Schedule`) ensuring strict multi-tenant isolation.
  2. Develop the customer-facing mobile UI (375px) to capture waitlist/pre-order intent seamlessly when inventory is zero.
  3. Create the Operations Agent capability to monitor stock and toggle product states autonomously.
  4. Build the Marketing Agent trigger to dispatch localized, personalized "Back in Stock" notifications with 1-tap checkout links.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
