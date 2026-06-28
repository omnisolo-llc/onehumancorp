issue_title: "Shopify API Integration"
issue_description: |
  **Problem Statement:**
  For owners like Priya (Boutique Operator) and Maya (Home Baker) who sell goods, managing inventory, orders, and customer data across separate tools is disjointed. They need OHC to capture online demand seamlessly without losing control of operations or needing manual data entry.

  **Research Report:**
  Shopify is a leading e-commerce platform globally, highly prevalent among small and medium businesses. Integrating Shopify allows OHC to sync products, inventory levels, orders, and customer details.
  - **Usability:** High for non-technical users. It provides clear APIs (Admin API for backend syncing). Webhooks are reliable for real-time order creation and inventory changes.
  - **Pricing:** Shopify API access is available on all standard plans.
  - **SaaS Viability:** Perfect fit for Cloud (multi-tenant) via OAuth. Standalone may be supported via custom app tokens.

  **Design Doc:**
  - **Trigger:** An owner connects their Shopify store via OAuth in the OHC integrations settings. Shopify webhooks are registered for `orders/create`, `products/update`, and `inventory_levels/update`.
  - **Action:** OHC listens to webhooks. Incoming orders are converted into prioritized tasks in the owner's feed (Work Triage). The Sales & Revenue Assistant can surface revenue summaries based on Shopify data. Customer details from orders enhance the Customer & Relationship Assistant's context.
  - **User Visibility:** The owner sees a clean "Shopify connected" status. When an order arrives, it shows up natively in the OHC feed as a task to fulfill. Inventory warnings are generated automatically.

  **Implementation Prompt:**
  Implement a Shopify integration that allows an owner to connect their store. Map incoming Shopify orders to OHC Work Tasks and sync basic product inventory status. Ensure the Work Triage UI elegantly displays new orders without requiring the user to switch apps, and that connection states are clearly communicated to the user.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []