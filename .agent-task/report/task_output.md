issue_title: "[Architecture Design] Multi-tenant Tap-to-Pay Terminal SDK Integration for OHC"
issue_description: |
  # Title: Multi-tenant Tap-to-Pay Terminal SDK Integration for OHC

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) and Carlos (Field Service Owner) conduct significant portions of their business in-person. Currently, they have to rely on disjointed Point-of-Sale (POS) systems or physical card readers that do not sync seamlessly with their OneHumanCorp (OHC) digital storefront, inventory, or booking systems. They need a unified "Tap-to-Pay" solution on their mobile devices (using Stripe Terminal SDK) that directly communicates with OHC. Without this, they suffer from reconciliation issues, delayed inventory updates, and fragmented customer profiles.

  ## Research Report
  - **Market Landscape:** Shopify POS, Square, and Stripe Terminal provide physical and mobile tap-to-pay solutions. However, Square and Shopify force users into their entire ecosystems.
  - **Stripe Terminal:** Stripe offers a Tap-to-Pay on iPhone/Android SDK that allows mobile devices to act as card readers without extra hardware.
  - **OHC Opportunity:** By integrating Stripe Terminal SDK (Tap-to-Pay) into the OHC Flutter mobile app, OHC can offer Priya and Carlos a native, hardware-free POS. Payments made in person will immediately update the central PostgreSQL Ledger, deduct inventory (integrated with Redis Redlock), and trigger the Customer Success Agent to send a digital receipt via SMS/WhatsApp.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Flutter App - POS Mode] -->|Stripe Terminal SDK| B(Tap-to-Pay / NFC Reader)
      B --> C{Stripe Backend}
      A -->|Initiate PaymentIntent| D[OHC API Layer - gRPC/REST]
      D --> E[Stripe API - ConnectionToken]
      E --> A
      C -->|Webhook: payment_intent.succeeded| F[OHC Webhook Worker]
      F --> G[Central Ledger DB - RLS Enforced]
      F --> H[Operations Agent: Deduct Inventory]
      F --> I[Customer Agent: Send SMS Receipt]
  ```

  ### Mobile UX Flow (375px)
  1. **Charge Screen:** The owner taps "Charge" on the OHC mobile app for a custom amount or selected catalog items.
  2. **Payment Mode:** App presents a bottom sheet offering "Tap to Pay on Phone", "Send Payment Link", or "Cash".
  3. **NFC Interaction:** User selects "Tap to Pay". The native Stripe Terminal UI takes over, prompting the customer to hold their card/phone near the owner's device.
  4. **Confirmation & Receipt:** Upon success, a large green checkmark appears with an option to instantly text or email the receipt to the customer's on-file contact (handled by the Customer Agent).

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors real-time POS transactions to update inventory and alert the owner if in-person sales deplete online stock.
  - **Finance Agent:** Automatically categorizes Terminal payments and reconciles them with the daily business ledger.
  - **Customer Success Agent:** Captures customer details (if a loyalty card or digital wallet is used) and drafts a personalized follow-up or digital receipt.

  ## Implementation Prompt
  Implement the backend service and database schema required to support Stripe Terminal Tap-to-Pay sessions.
  - Create the `create_terminal_connection_token` endpoint.
  - Ensure the existing `payment_intents` table correctly handles `capture_method: manual` or terminal-specific metadata.
  - Ensure webhooks handle the terminal-specific payment success events, integrating with the multi-tenant `ledger`.
  - Provide a Playwright E2E test verifying the payment flow (mocking the external Stripe SDK interaction where necessary using official test-mode parameters).

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
