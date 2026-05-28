issue_title: "Implement Instant Localized Offline-First POS"
issue_description: |
  # Research Report: Offline-First Point of Sale

  **Findings:**
  Small businesses operating in variable-connectivity environments (food carts, pop-up markets) suffer significant revenue loss when cloud-reliant POS systems fail offline. Competitors like Shopify and Square either have limited offline modes or require proprietary hardware.

  **Proposed Next Steps:**
  Implement an offline-first POS architecture leveraging local device storage and CRDTs for seamless sync upon reconnection. This system should utilize native device capabilities like Tap-to-Pay, eliminating the need for external card readers.

  See full design details in `docs/research/[architecture]_instant_localized_offline_first_point_of_sale.md`.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
