issue_title: "Implement Agentic Inventory Reorder Assistant (The Restocker)"
issue_description: |
  # Research Report: Agentic Inventory Reorder Assistant (The Restocker)

  ## Title
  Agentic Inventory Reorder Assistant (The Restocker)

  ## Problem Statement
  Small business owners with physical inventory (e.g., Priya the boutique owner, Fatima the food cart operator) constantly struggle with stockouts and inventory management. They have to manually monitor inventory levels, calculate lead times, draft purchase orders, and remember to restock items before they run out. This manual process is error-prone, time-consuming, and often results in lost revenue due to out-of-stock products. Existing platforms like Shopify provide "low stock alerts" but fail to take the next step: actively assisting the owner in drafting and executing the reorder.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Offer basic "low stock" email notifications. The owner must log in, review the stock, find the supplier information, and manually create a purchase order or contact the supplier.
  - **Square:** Good inventory tracking but lacks proactive reorder drafting.
  - **Dedicated Inventory Software (e.g., TradeGecko/QuickBooks Commerce):** Too complex and expensive for a micro-SME, requiring significant setup and manual operation.
  - **OHC Opportunity:** Move from passive alerts to active assistance. "The Restocker" (a function of the Operations Agent) monitors inventory, predicts stockouts based on sales velocity, and proactively drafts a complete restock order. The owner receives an "Action Required" card in their mobile feed, showing the suggested order quantity, supplier, and estimated cost. A single "Approve & Send" tap sends the drafted email/PO to the supplier.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[PostgreSQL Central Ledger] -->|Inventory Level Changes| B(Event Mesh)
      B --> C{Operations Agent: The Restocker}
      C -->|Calculate Velocity & Thresholds| D[Predictive Logic]
      D -->|Stockout Risk Detected| E[RAG: Retrieve Supplier Info & Past Orders]
      E --> F[LLM: Draft Purchase Order/Email]
      F --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|Owner: 1-Tap Approve| I[Email/Notification Service]
      I --> J[Supplier]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** A priority card appears: "Low Stock Alert: Organic Flour".
  - **Interaction:** Tapping the card opens a detailed view.
    - **Context:** "Current stock: 5 lbs. Expected to run out in 2 days."
    - **Drafted Action:** "I've drafted a reorder email to Bob's Mill for 50 lbs (based on last month's usage). Estimated cost: $45."
  - **Action:**
    - Primary Button: "Approve & Send Order"
    - Secondary Button: "Edit Quantity"
    - Tertiary Button: "Dismiss"
  - **Visual Design:** Utilizes OHC Premium Tokens (Glassmorphism, #FF9500 for warning/low stock context, 44px+ touch targets).

  ### AI Agent Integration Points
  - **Operations Agent (The Restocker):** Triggered by a daily CRON job or specific inventory threshold events. Uses RAG against the tenant's product database to find linked suppliers and past purchase history. Uses an LLM to draft a polite, professional reorder email or generate a structured Purchase Order PDF/JSON depending on the supplier's preferences.

  ### Key Design Decisions
  - **Proactive vs. Reactive:** The system doesn't just say "You are low on X." It says "You are low on X, here is the email to order more Y from Z, shall I send it?"
  - **Velocity-Based Thresholds (Future):** While initial implementation can use static thresholds, the architecture should support dynamic thresholds based on sales velocity.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a key ingredient or product runs low, I open the OHC app to find a pre-written reorder request to my supplier. I tap one button to send it, ensuring I never run out of stock and losing zero time on admin.
  **CUJ & Acceptance Criteria:**
  1.  **Setup:** A product in the database is linked to a "Supplier" entity (with an email address) and has a "Low Stock Threshold" set.
  2.  **Trigger:** The product's inventory level drops below the threshold (e.g., via a simulated sale).
  3.  **Agent Action:** The Restocker agent detects the low stock, retrieves the supplier info, and drafts a reorder email.
  4.  **UI Verification:** The drafted reorder appears as a card in the mobile agent feed.
  5.  **Execution:** The user clicks "Approve & Send", and the system logs the intent to send the email (or sends it via a mocked email service).
  6.  **Tests:** Provide Playwright E2E tests verifying the low-stock trigger creates the feed item, and the approval flow functions correctly on a 375px viewport.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
