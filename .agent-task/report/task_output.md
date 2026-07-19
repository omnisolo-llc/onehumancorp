issue_title: "Zero-Hardware Tap-to-Pay POS Architecture and Unified Append-Only Ledger Integration"
issue_description: |
  ### Problem Statement
  Owners like Carlos (field services), Priya (boutique), and Fatima (food cart) require a low-friction, zero-hardware POS capability to collect in-person card-present tap-to-pay payments directly from their mobile phones. Additionally, these transaction flows must seamlessly reconcile and record state transitions within a unified, multi-tenant inventory management system and an append-only universal ledger (`ohc_universal_ledger`) without creating double-decrements or duplicate entries.

  ### Research Report
  - **Competitive Landscape**: Vertical SaaS tools like Square and Shopify POS utilize native Android/iOS Tap to Pay SDKs to capture contactless payments directly via device NFC chips. Transactions are managed asynchronously or captured instantly through backend processing layers.
  - **Multi-Tenant Offline Reconciliation**: For merchants operating in flaky network scenarios (e.g., Fatima's food cart), offline-staged POS payments must sync deterministically. Standard upserts without deep de-duplication tracking can execute duplicate API calls, resulting in multiple order entries or dual inventory decrements. Safe replay queues require tracking of `newly_inserted_ids` of synced offline transactions to process state mutations exactly once.

  ### Design Doc
  #### High-Level Architecture (Mermaid.js)
  ```mermaid
  sequenceDiagram
    participant App as Mobile Flutter/PWA
    participant API as POS Terminal API
    participant Stripe as Stripe SDK/Contactless NFC
    participant DB as Postgres (Tenant Scoped)

    App->>API: Create Payment Intent (Amount, Currency)
    API->>Stripe: Initiate Contactless Payment Intent
    Stripe-->>API: Contactless Card Capture
    API->>DB: Check Idempotency Token
    API->>DB: Update Payment State & Decrement Inventory
    API->>DB: Append Ledger Entry (ohc_universal_ledger)
    API-->>App: Return Capture Confirmation
  ```

  #### Mobile UX Flow (375px Breakpoints)
  - **Storefront Payment Request Screen**: Displays premium macOS-style Translucent Glass materials. Restrained translucent background with soft rounded edges (16px) containing clear, bold price typography.
  - **Tap-to-Pay NFC Capture Overlay**: Invokes the device-native NFC tap-to-pay interface with a 44x44px close control. Underflung background visualizes successful tap through clear visual pulse cues.
  - **Success State & Digital Receipt**: Immediately displays a prominent transaction confirmation card containing ledger transaction references, payment breakdown, and print/share options.

  #### AI Agent Integration Points
  - **CS/Support Assistant**: Intercepts terminal connection failures and suggests local troubleshooting protocols (e.g., enabling NFC, resetting device terminal sessions) in clear non-technical language.
  - **Finance & Decision Assistant**: Detects card-present patterns and automatically aggregates in-person transaction summaries within the plain-language daily performance briefing.

  ### Implementation Prompt
  Implement a complete, zero-hardware Tap-to-Pay POS collection system integrated with our backend terminal APIs:
  - **Terminal Intention Recovery**: Implement a function to retrieve card-present payment intent objects using Stripe's API.
  - **Idempotency & Atomic State Management**: Ensure in-person captured payments use Stripe terminal intents checked idempotently against local tables. Record transitions as transactional mutations that alter product inventory levels and record tracking within the global ledger.
  - **Offline Sync Queue Protection**: Prevent dual-captures and double inventory adjustments under flaky mobile networks when syncing staged offline entries. Use internal set-tracking to deduplicate processing rows during batch replays.
  - **Universal Ledger Integration**: Automatically record all synced POS transactions under the `sales_and_revenue` department within the multi-tenant append-only database ledger (`ohc_universal_ledger`).

  ### Priority & Scope
  - **Priority**: P1 (High)
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
