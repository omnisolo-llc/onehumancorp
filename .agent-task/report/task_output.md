issue_title: "Research Report: Autonomous Supply Replenishment (The Quartermaster Agent)"
issue_description: |
  # Research Report: Autonomous Supply Replenishment (The Quartermaster Agent)

  ## Problem Statement
  Small business operators like Jun (Location Manager) or Fatima (Food Cart Operator) constantly battle supply runouts. Managing inventory levels for raw materials (flour, cups, cleaning supplies) is a highly manual, error-prone task. Traditional Point of Sale or inventory systems might alert when stock is low, but they require the owner to manually draft purchase orders, contact vendors via email/WhatsApp, and track deliveries. This context switching pulls operators away from serving customers and driving revenue.

  ## Research Report
  - **Market Context**: Legacy platforms (Shopify, Square) track *finished goods* inventory well but often struggle with *raw materials* or supplies without complex, expensive ERP integrations (like TradeGecko/QuickBooks Commerce). Even when they track raw materials, they are passive—they send an alert, leaving the physical ordering work to the human.
  - **The OHC Opportunity**: OneHumanCorp can turn passive low-stock alerts into proactive, drafted supply orders. By leveraging the AI Operations Agent (The Quartermaster), OHC can predict when supplies will run out based on sales velocity and autonomously draft reorder messages or purchase orders to suppliers.
  - **Competitor Gaps**:
    - *Square / Toast*: Excellent at recipes and raw material decrementing, but poor at the procurement step. They generate a report, not an action.
    - *Shopify*: Focuses on finished goods. Raw material tracking requires 3rd-party apps.
    - *Dedicated Inventory Apps (e.g., Katana, DEAR)*: Too complex, expensive, and technical for micro-SMEs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Sales / Daily Usage Event] --> B[Central Ledger / Inventory Decrement]
      B --> C{Threshold Check}
      C -- Below Minimum --> D[Operations Agent: The Quartermaster]
      D -->|Query Supplier Context| E[Tenant Memory / Vendor DB]
      D -->|Draft Reorder| F[Action Required Queue]
      F --> G[Mobile App Feed 375px]
      G -->|1-Tap Approve| H[Omnichannel Dispatcher: Email/WhatsApp to Vendor]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed (Mobile)**: Top card shows "Supply Alert: Coffee Cups running low. Order drafted."
  2. **Interaction**: Tapping the card opens the drafted order. The top half shows the context (Current stock: 50 cups, est. runout: 2 days). The bottom half shows the AI-drafted message to the supplier (e.g., "Hi Sam, please send 500 more 12oz cups to the Main St location.").
  3. **Action**: A prominent primary button "Approve & Send" and a secondary "Edit".
  4. **Visual Design**: Uses OHC Premium Token library with glassmorphism cards and a clean Ubiquiti-style hierarchy.

  ### AI Agent Integration Points
  - **Operations Agent (The Quartermaster)**: Triggered by the inventory system when stock dips below a threshold. It uses RAG to pull vendor contact info, previous order quantities, and preferred communication channels (email vs. SMS). It drafts the communication and places it in the `ActionRequiredQueue` for the tenant.

  ### Key Design Decisions
  - **Action-Oriented Alerts**: We do not just show a red dot next to an item. We show the drafted solution.
  - **Vendor Agnosticism**: Small vendors don't have APIs. They use text messages, WhatsApp, and emails. The Omnichannel Dispatcher must support these unstructured outbound channels.
  - **Predictive (Future)**: Initially threshold-based, moving towards predictive velocity-based drafting.

  ## Implementation Prompt
  **User-Facing Outcome:** As a location manager, I open the OHC app and see a card saying we are low on coffee cups, with a pre-written text message to my supplier. I tap "Approve" and the text is sent. I didn't have to check the stockroom or type a message.

  **CUJ & Acceptance Criteria:**
  1. An inventory deduction event triggers the item stock to fall below the configured `reorder_point`.
  2. The Quartermaster Agent is invoked via the job queue.
  3. The Agent successfully retrieves the associated vendor details (name, contact method) and drafts a supply order.
  4. A pending action card appears in the mobile-sized Agent Feed for the tenant.
  5. The user taps "Approve" via the UI, triggering the dispatch of the drafted message to the simulated vendor endpoint.
  6. Provide Playwright E2E tests: A user logs in, sees the drafted supply order card on the feed, taps "Approve," and the system confirms dispatch.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
