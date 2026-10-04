outcome: no_work
issue_title: "GitHub Issue #34535: Mobile Tap-to-Pay & Omnichannel POS Integration"
issue_description: |
  The requested Mobile Tap-to-Pay feature using the Stripe Terminal SDK is already fully implemented in the current repository's technology stack (Next.js/Rust) instead of the obsolete technology requested (Flutter/Dart). The UI correctly handles selecting an item from the inventory or entering a custom amount (`quick_charge`), initiates a Tap to Pay session using the native mobile hardware via `StripeTerminalClient.tsx`, and updates the backend. The backend successfully processes the capture payment intent (`terminal_api.rs`), decrements the inventory (`commit_inventory`), and updates the unified ledger via the `orders` and `order_items` tables. The UI is optimized for mobile at 375px width, and connection tokens are provided by the backend without requiring the user to configure Stripe keys. End-to-end testing is verified through the `terminal_pos.mock-contract.ts` and `e2e_mobile_pos_optimistic.mock-contract.ts` files. Thus, no further implementation is necessary.
issue_priority: "P1"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
