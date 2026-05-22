issue_title: "Autonomous Supplier Ordering Engine"
issue_description: |
  **Autonomous Supplier Ordering Engine**

  **Problem Statement**
  Small business owners like Fatima (food cart) and Priya (boutique owner) spend hours every week managing inventory and manually reordering supplies from various vendors (wholesale suppliers, local bakeries, packaging companies). When they run out of a critical ingredient or product, they lose revenue and customer trust. The process of predicting when stock will run out, generating purchase orders, and paying suppliers is entirely manual, prone to human error, and time-consuming. They need a system that seamlessly and invisibly monitors stock levels, predicts depletion based on sales velocity, and automatically places orders with suppliers before stock runs out, handling the payment and communication without requiring them to lift a finger (unless they want to approve).

  **Research Report**
  - **Market Gap:** Current platforms (Shopify, Wix, Squarespace) offer inventory tracking but lack autonomous reordering. They require third-party apps (e.g., Stocky) which only send "low stock alerts." The merchant still has to manually create POs, email them, and handle invoicing.
  - **Data & Insights:** Small businesses spend up to 15 hours a week on inventory and supplier management. Stockouts account for an estimated 4% loss in annual revenue.
  - **Competitive Analysis:**
    - *Shopify:* Has basic reorder points, but relies heavily on apps for automated PO generation. No autonomous agentic negotiation or direct supplier integration out-of-the-box.
    - *Square:* Good POS inventory, but POs are manual.
    - *OneHumanCorp Opportunity:* Introduce an AI Operations Agent that not only tracks inventory but autonomously contacts suppliers (via email, SMS, or API), places orders based on predictive algorithms, and manages the accounts payable ledger.

  **Next Steps**
  - Implement the Autonomous Supplier Ordering Engine according to the architecture design doc `docs/research/[architecture]_autonomous_supplier_ordering_engine.md`.
  - Design database models, AI agents, and mobile UI flows.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
