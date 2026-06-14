issue_title: "[architecture] Universal Tap-to-Pay POS and Centralized Inventory Sync"
issue_description: |
  ## Title
  Universal Tap-to-Pay POS and Centralized Inventory Sync

  ## Problem Statement
  For non-technical owners like Priya (Boutique Operator) and Carlos (Field Service), physical and digital operations are currently completely disconnected. While we have tools for digital orders, in-person sales rely on separate systems (e.g. basic card readers, disconnected cash registers). This creates fragmented inventory, manual double-entry, and an incomplete view of customer behavior. There is a massive friction point when Priya needs to mark a shirt "Sold Out" online right after selling it in-store, or when Carlos needs to instantly collect payment at a client's house without switching apps.

  ## Research Report
  - **Market Context**: Square, Shopify POS, and Stripe Terminal dominate because they unify online and offline contexts. However, Square and Shopify force users into heavy, complex point-of-sale administration interfaces.
  - **Competitive Gaps**:
    - *Shopify POS*: Requires a separate POS app download, complex sync settings, and expensive hardware for advanced features.
    - *Square*: Strong hardware ecosystem but weak at bridging service bookings (like Carlos's) with physical inventory in a single seamless mobile flow.
  - **User Need (OHC Target)**: The owner needs to open the main OHC Assistant app on their 375px phone, tap a "Charge" button, use Apple/Google native Tap-to-Pay on their existing device, and have the AI immediately sync the transaction to the unified ledger, deduct the specific inventory item, and text the receipt—all invisibly.

  ## Design Doc
  ### Mobile UX Flow (375px Baseline)
  1. **Omni-Search/Action Bar**: From the main feed, the owner taps a prominent "Charge" FAB.
  2. **Smart Cart Construction**: The owner types "Blue Dress M" or "Sink Repair". The Assistant auto-suggests from central inventory or services.
  3. **Payment Modality Selection**: User selects "Tap to Pay on Phone" (utilizing native OS features via Stripe Terminal SDK) or "Send Payment Link".
  4. **The Tap**: The native NFC prompt appears. Customer taps their card/phone.
  5. **Instant Sync & Receipt**: The screen flashes a success checkmark. The Finance Assistant logs the ledger entry, the Manager Assistant decrements inventory, and the Ambassador Assistant prompts to text the receipt.

  ### AI Agent Integration Points
  - **The Manager (Operations)**: Watches the POS event stream. When a Tap-to-Pay transaction completes, it automatically decrements the central inventory count. If stock reaches zero, it pushes an update to hide the item from the online Storefront Builder.
  - **The Accountant (Finance)**: Reconciles the Stripe Terminal payout with the centralized ledger, ensuring offline sales are visible alongside online subscriptions and deposits.
  - **The Ambassador (Customer Success)**: If a returning customer card is recognized (via Stripe tokenization), it links the in-store purchase to their existing CRM profile for future re-engagement.

  ### Key Design Decisions
  - **No Dedicated POS App**: POS functionality is built directly into the core OHC mobile shell. We do not require a separate app download.
  - **Hardware-Agnostic First**: We rely entirely on Tap-to-Pay on iPhone/Android via the Stripe Terminal SDK before offering dedicated card readers. This ensures Maya, Carlos, and Priya can take payments immediately with zero hardware investment.
  - **Single Source of Truth**: Inventory is never "synced" periodically; the Tap-to-Pay transaction and the online cart checkout hit the exact same PostgreSQL inventory mutation endpoint.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant OwnerApp as OHC Mobile App (375px)
      participant StripeDevice as Native Tap-to-Pay (NFC)
      participant Backend as OHC API Layer (Rust)
      participant Stripe as Stripe Terminal API
      participant Manager as Manager Agent (Inventory)
      participant DB as Postgres (Tenant Isolated)

      OwnerApp->>Backend: Create PaymentIntent (Item: Blue Dress)
      Backend->>Stripe: Init Terminal PaymentIntent
      Stripe-->>Backend: Intent Secret
      Backend-->>OwnerApp: Intent Secret
      OwnerApp->>StripeDevice: Present Tap-to-Pay UI
      StripeDevice-->>Stripe: Process NFC Card Data
      Stripe-->>Backend: Webhook (Payment Succeeded)
      Backend->>DB: Update Ledger & Record Sale
      Backend->>Manager: Trigger Inventory Sync Event
      Manager->>DB: Decrement "Blue Dress" Inventory
      Manager->>DB: If Stock == 0, Set "OnlineVisible" = false
      Backend-->>OwnerApp: Push Notification: Sale Complete!
  ```

  ## Implementation Prompt
  Implement the backend infrastructure and mobile-first frontend flow for Tap-to-Pay Point of Sale with centralized inventory sync.
  - **CUJ**: Priya (Boutique Owner) opens the OHC app, selects a physical product from her catalog, taps "Charge via Phone", accepts a customer's NFC payment, and sees the item's inventory decrement immediately so it cannot be sold online.
  - **Acceptance Criteria**:
    1. A single "Charge" interface exists that can pull from the unified catalog.
    2. The Stripe Terminal SDK is integrated for Tap-to-Pay on compatible devices (mocked in tests).
    3. A successful transaction immediately triggers a ledger entry and an inventory decrement.
    4. If inventory hits zero, the item must no longer be selectable in digital storefront APIs.
    5. The UI must be fully functional and usable on a 375px viewport without horizontal scrolling.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
