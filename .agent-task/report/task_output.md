issue_title: "Agentic Tap-to-Pay & Unified In-Person POS"
issue_description: |
  # Research Report: Agentic Tap-to-Pay & Unified In-Person POS

  ## 1. Problem Statement
  Small business owners (like Priya the Boutique Operator or Fatima the Food Cart Operator) struggle with disjointed online and offline sales. They often rely on separate physical POS systems (like Square or standalone Stripe Terminals) that do not sync inventory in real-time with their online storefronts. This leads to overselling, manual reconciliation, and a fragmented customer experience. They need a unified system where in-person tap-to-pay transactions immediately update online inventory, trigger the Operations Agent for restocking, and populate the daily revenue summary, all seamlessly operated from their 375px mobile device without extra hardware.

  ## 2. Research Report
  - **Market Context**: Square dominates the SMB physical POS market but lacks deep agentic AI integrations. Shopify POS is robust but expensive and often requires separate hardware for full functionality. Stripe Terminal provides SDKs for Tap-to-Pay directly on iPhone/Android, allowing any modern smartphone to become a fully capable POS without additional dongles or hardware.
  - **The OHC Opportunity**: By integrating Stripe Tap-to-Pay natively into the OHC Flutter mobile app, OHC allows owners to take secure in-person payments directly on their phones. The Sales & Revenue Agent can immediately log the transaction, the Operations Agent can update inventory, and the CS Agent can draft a digital receipt.
  - **Competitor Gaps**:
    - *Square*: Weak agentic workflows; mostly a passive register.
    - *Shopify POS*: Complex to set up, requires hardware add-ons for many tiers, not AI-assistant-first.
    - *Wix/Squarespace*: Limited native Tap-to-Pay capabilities on mobile devices without separate reader hardware.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Flutter] -->|Stripe Terminal SDK| B(Tap-to-Pay Interface)
      B -->|Payment Intent| C[OHC Backend - Go/Bazel]
      C -->|Verify| D(Stripe API)
      D -->|Webhook/Response| C
      C -->|Record Transaction| E[(PostgreSQL - Unified Ledger)]
      C -->|Emit Event| F[Event Mesh]
      F --> G[Sales & Revenue Agent]
      F --> H[Operations Agent]
      G -->|Update Summary| I[Owner Dashboard]
      H -->|Deduct Inventory| E
      H -->|Low Stock Alert| I
  ```

  ### Mobile UX Flow (375px)
  1. **Checkout Flow**: The owner opens the "Sell In-Person" tab on the OHC app. They select items from their catalog using large, touch-friendly 44x44px targets.
  2. **Payment Screen**: The app transitions to a clean "Tap to Pay" screen using the Stripe Terminal SDK overlay.
  3. **Customer Action**: The customer taps their NFC-enabled card or mobile wallet on the owner's phone.
  4. **Confirmation**: A glassmorphism success card appears. The Operations Agent immediately deducts inventory. The owner can optionally send a digital receipt to the customer's phone number or email (which links back to their unified customer profile).

  ### AI Agent Integration Points
  - **Operations Agent**: Automatically triggered via the event mesh upon successful payment. Deducts stock quantities and flags items for restocking if they fall below the tenant's threshold.
  - **Sales & Revenue Agent**: Ingests the transaction into the daily financial summary, categorizing it as "In-Store Sales" to differentiate from online purchases.
  - **Customer Success Agent**: If the customer provides an email for the receipt, it links this purchase to their Omnichannel Identity Graph, remembering their preferences for future DMs or online visits.

  ### Key Design Decisions
  - **No Extra Hardware**: Leverage Stripe Terminal's Tap-to-Pay on iPhone/Android to minimize friction for new businesses like Fatima's Food Cart.
  - **Immediate Sync**: Ensure the unified ledger handles the transaction as a first-class order, identical to an online cart checkout.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Tap-to-Pay POS
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya can ring up customers in her physical store by having them tap their card on her phone. The inventory instantly updates, ensuring online shoppers don't buy out-of-stock items, and her daily summary reflects both in-store and online revenue.

  **Next Actions**:
  1. Integrate the `stripe_terminal` plugin into the Flutter frontend, providing the UI flows for assembling an in-person cart and initiating Tap-to-Pay.
  2. Implement the Go backend endpoints to generate Stripe Connection Tokens and capture Payment Intents from the Terminal SDK.
  3. Ensure the unified order creation logic handles in-person orders, correctly linking them to the `Sales & Revenue Agent` and `Operations Agent` for inventory deduction.
  4. Design the "Sell In-Person" 375px mobile UI using the OHC Premium Token library (Translucent materials, strong typography).

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []