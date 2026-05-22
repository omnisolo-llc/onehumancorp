issue_title: "Implement Zero-Hardware Mobile Tap-to-Pay & Real-Time Omni-Channel POS Synchronization"
issue_description: |
  # [payments] Mobile Tap-to-Pay & Omni-Channel Inventory Sync Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart) face immense friction operating both in-person and online storefronts simultaneously. They often have to manage separate hardware POS terminals (e.g., Square readers, legacy registers) and standalone online stores (e.g., Shopify, Wix).

  For Priya, selling a "Medium Red Summer Dress" in-store without instantly syncing with her online inventory leads to double-selling and customer frustration. For Fatima, switching contexts between her phone's web orders and taking physical credit cards in the rain is slow and prone to errors.

  From a non-technical owner's perspective: "I shouldn't need to buy a $300 card reader or manually update my website when I sell something in person. I just want to tap their card on my phone, have the money go to my account, and the item disappear from my online store instantly."

  ## Research Report
  ### Competitor Analysis
  *   **Shopify POS**: Requires expensive, proprietary hardware for robust offline/in-person payments. Inventory syncs, but it's fundamentally a desktop-first architecture ported to mobile.
  *   **Square**: Excellent mobile POS but weak, disjointed online storefronts requiring complex third-party integrations for digital products/services.
  *   **Stripe Terminal (Tap to Pay on iPhone/Android)**: This is the critical unlock. Stripe recently released SDKs allowing any NFC-equipped smartphone to act as an EMV contactless terminal without extra hardware.

  ### Architectural Gap in OHC
  Currently, OHC provides rapid digital storefront creation, but lacks a native, frictionless bridge to physical, in-person transactions. To serve the "Physical Products" and "Food & Beverage" business matrices effectively, we must implement a deeply integrated, zero-hardware Mobile Tap-to-Pay system that treats the smartphone itself as the ultimate edge node.

  ## Design Doc

  ### 1. Business Journey Mapping
  *   **Acquisition/Onboarding**: Priya sets up her OHC store. The app prompts: "Will you sell in person?" She taps "Yes". OHC instantly provisions a Stripe Terminal virtual reader attached to her identity.
  *   **Activation**: In her boutique, a customer wants to buy a dress. Priya opens the OHC app on her iPhone, selects the dress from her visual catalog, and taps "Charge $45.00".
  *   **Transaction (The Magic)**: The native iOS/Android Tap-to-Pay modal appears (translucent, Apple/Google native UI). The customer taps their physical Visa card on Priya's phone.
  *   **Real-time Sync**: The payment clears. Instantly, the OHC Operations AI Agent deducts `1` from the online inventory, preventing double-selling. The Finance AI Agent logs the ledger entry. The CS AI Agent asks the customer (via the terminal screen) if they want a digital receipt texted to them.

  ### 2. Mobile UX Flow (375px Viewport)
  1.  **Catalog Screen**: Clean, glassmorphism grid of products. Priya taps "Red Dress (M)".
  2.  **Cart Drawer**: Slides up from bottom. Big, bold action button: `[ Pay $45.00 ]` (Primary Brand Color).
  3.  **Payment Method Sheet**: `[ Tap to Pay ]` (prominent), `[ Send Invoice Link ]`, `[ Cash ]`.
  4.  **Tap to Pay Native Modal**: System-level secure UI takes over. "Hold card near phone".
  5.  **Success State**: Haptic feedback. Green checkmark overlay. CS Agent prompt: "Text receipt to 555-..."

  ### 3. Architecture Diagram (Mermaid.js)

  ```mermaid
  erDiagram
      TENANT ||--o{ TERMINAL_SESSION : owns
      TERMINAL_SESSION ||--|| PAYMENT_INTENT : processes
      PAYMENT_INTENT ||--|| LEDGER_ENTRY : triggers
      PAYMENT_INTENT }o--|{ INVENTORY_RESERVATION : locks

      TENANT {
          string id
          string stripe_connected_account
      }
      TERMINAL_SESSION {
          string id
          string device_token
          string spiffe_id
          string status
      }
      PAYMENT_INTENT {
          string id
          float amount
          string currency
          string status
      }
      INVENTORY_RESERVATION {
          string id
          string product_id
          int quantity
          datetime lock_expires
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Priya (App)
      participant Stripe_SDK
      participant OHC_Edge_Gateway (SPIRE mTLS)
      participant Inventory_Service
      participant AI_Ops_Agent
      participant AI_Finance_Agent

      Priya (App)->>Stripe_SDK: Initialize Tap-to-Pay ($45.00)
      Stripe_SDK-->>Priya (App): Display Native Payment Modal
      actor Customer
      Customer->>Stripe_SDK: Taps physical credit card
      Stripe_SDK->>Stripe_API: Process EMV transaction
      Stripe_API-->>Stripe_SDK: Payment Success
      Stripe_SDK-->>Priya (App): Success Callback

      Priya (App)->>OHC_Edge_Gateway: Record Transaction (signed via device SPIFFE ID)
      OHC_Edge_Gateway->>Inventory_Service: Commit INVENTORY_RESERVATION
      OHC_Edge_Gateway->>AI_Ops_Agent: "Update Omni-Channel Catalog"
      OHC_Edge_Gateway->>AI_Finance_Agent: "Reconcile Ledger & Prep Payout"
  ```

  ### 4. Technical Integrity & Security
  *   **Zero-Trust Isolation**: The mobile client's communication with the OHC backend for POS transactions is verified via device-specific SPIFFE/SPIRE certificates. The `TERMINAL_SESSION` is strictly bounded to the `TENANT` to prevent cross-account charging.
  *   **Performance Targets**: The transition from Cart to the Native Stripe Modal must occur in < 200ms.
  *   **Offline Capability**: If OHC backend connectivity drops, the Stripe SDK handles secure offline queuing of the payment intent, and the OHC mobile app creates a local SQLite `INVENTORY_RESERVATION` to prevent immediate local double-selling until sync is restored.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the backend foundation and mobile UI scaffolding for Zero-Hardware Tap-to-Pay.
  1. The user must be able to select items from their existing OHC catalog in the mobile app and add them to a POS Cart.
  2. The UI must match our macOS-style translucent glass guidelines (375px viewport optimized).
  3. When checking out, trigger the necessary backend services to generate a secure Terminal session token.
  4. Ensure that upon successful payment completion, the system atomically updates the global inventory count and triggers the relevant AI Agents (Finance, Ops) to handle post-transaction workflows invisibly.
  5. All endpoints must enforce tenant isolation based on our SPIFFE identity model. Do not implement the raw Stripe SDK integration; stub the native SDK boundary. Focus on the secure backend lifecycle and the fluid mobile UI/state management.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
