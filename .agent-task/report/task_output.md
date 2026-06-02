issue_title: "[Payments] Multi-Tenant POS Tap-to-Pay Integration"
issue_description: |
  # Multi-Tenant POS Tap-to-Pay Integration via Stripe Terminal SDK

  ## Problem Statement
  Priya (boutique owner) and Carlos (handyman) need to take in-person payments efficiently. Currently, they have to use separate physical card readers or manage offline transactions that don't sync with their OHC dashboard. They need a seamless way to accept tap-to-pay (Apple Pay, Google Pay, physical contactless cards) directly on their mobile devices (iPhone/Android) without external hardware, tightly integrated with their OHC inventory and ledger.

  ## Research Report
  - **Competitor Analysis**: Shopify offers POS app and Tap to Pay; Square is built around this; Wix recently added it.
  - **Technology Landscape**: Stripe Terminal provides Tap to Pay on iPhone and Android SDKs. Requires specific certification and handling of Terminal connection tokens.
  - **Pain Points**: Managing external card readers is a hassle. Disconnect between in-person sales and online inventory. Lack of multi-tenant isolation for Stripe Terminal connection tokens.

  ## Design Doc
  - **Architecture**: Mobile App requests Terminal Token from OHC API -> OHC API verifies tenant identity and requests token from Stripe API -> Mobile app initializes Stripe SDK -> Payment Processed -> Webhook updates OHC Ledger.
  - **Mobile UX Flow (375px)**: Cart view -> "Tap to Pay" FAB -> Native Tap to Pay OS sheet -> Success Screen.
  - **AI Integration**: Finance agent reconciles transaction; Customer Success agent sends receipt; Operations agent deducts inventory.
  - **Decisions**: No external hardware needed; Tokens strictly scoped per tenant_id.

  ## Implementation Prompt
  Implement the backend foundation for Stripe Terminal Tap-to-Pay. Create an API endpoint to generate Stripe Terminal connection tokens scoped to the authenticated user's `tenant_id`. Update the `Finance & Payments` agent capabilities to recognize `pos_transaction` events from Stripe webhooks and update the tenant's ledger and inventory accordingly. Ensure the API uses the existing Rust gRPC structure with Redis distributed locking during transaction capture to prevent race conditions on inventory decrement.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
