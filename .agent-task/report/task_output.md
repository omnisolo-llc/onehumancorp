issue_title: "Intelligent Multi-Tenant Tap-to-Pay & Instant Localized Invoicing Engine"
issue_description: |
  ## 1. Problem Statement
  Owners like Carlos (handyman) and Priya (boutique operator) need to accept payments effortlessly in person without relying on disconnected hardware or separate apps. Carlos often finishes a repair and needs to instantly generate a localized invoice and take a tap-to-pay payment on his Android phone. Priya needs a seamless tap-to-pay solution for her physical boutique that synchronizes perfectly with her online inventory. Currently, SMBs are forced to use fragmented tools (e.g., Square for in-person, Shopify for online, and manual invoicing), which creates a messy back-office and disconnected customer experiences.

  ## 2. Research Report
  - **Competitor Analysis**:
    - **Square**: Dominates in-person but requires a separate ecosystem from the online storefront, creating inventory and customer data silos.
    - **Shopify POS**: Powerful but often too complex for micro-SMBs, and typically pushes users toward proprietary hardware rather than native smartphone tap-to-pay.
    - **Stripe Terminal**: Offers robust SDKs for Tap to Pay on iPhone and Android, which is the perfect primitive for our solution.
  - **The OHC Opportunity**: By integrating Tap to Pay directly into the OHC mobile app, we eliminate the need for extra hardware. The integration of AI agents ensures that the moment a payment is taken, the invoice is generated, the ledger is updated, inventory is reconciled, and follow-up actions (like review requests) are drafted autonomously.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Mobile App - Flutter] -->|Tap to Pay SDK| B(Native OS: iOS/Android Tap to Pay)
      A -->|gRPC/REST| C(OHC Backend - Go)
      C -->|Stripe API| D(Stripe Terminal & Billing)
      C -->|Update Ledger| E[(PostgreSQL - Multi-tenant)]
      C -->|Trigger Events| F[AI Finance & Operations Agents]
      F -->|Update Feed| A
  ```

  ### Mobile UX Flow (375px First)
  1. **Job Completion / Checkout (375px)**: Carlos or Priya opens the OHC app, taps "New Charge" or converts an existing estimate into an invoice.
  2. **Payment Method Selection**: The UI presents large, touch-friendly buttons (44x44px min). The primary call-to-action is "Tap to Pay on Phone".
  3. **Native Tap to Pay**: The app invokes the native Stripe Terminal SDK, bringing up the OS-level Tap to Pay interface.
  4. **Instant Invoicing**: Upon successful payment, the OHC backend instantly generates a localized, PDF-ready invoice and sends a digital receipt to the customer via SMS or email.
  5. **Agent Handoff**: The mobile UI returns to the main feed, showing a success card: "Payment received. Invoice #1024 sent."

  ### AI Agent Integration
  - **The Finance Assistant**: Instantly reconciles the payment against the central ledger and updates the owner's daily revenue summary.
  - **The Customer Assistant**: Automatically drafts a personalized thank-you message and a request for a Google Review to be sent 24 hours later, appearing in the owner's Agent Feed for one-tap approval.

  ### Key Design Decisions
  - **Hardware-less**: We strictly utilize Tap to Pay on iPhone and Android via Stripe Terminal to eliminate friction and hardware costs.
  - **Zero Trust & Multi-Tenancy**: The backend must use strict tenant scoping (`tenant_id`) in PostgreSQL and secure SPIFFE/SPIRE identity for all Stripe API calls.
  - **Offline Resilience**: While Tap to Pay requires connectivity, the invoice drafting and cart building must work offline, syncing and executing the payment intent when connectivity is restored.

  ## 4. Implementation Prompt
  **Feature Name**: Intelligent Multi-Tenant Tap-to-Pay & Instant Localized Invoicing Engine

  **Target Personas**: Carlos (Handyman) & Priya (Boutique Operator)

  **Outcome**: Enable non-technical owners to collect in-person payments directly on their smartphones without extra hardware, while the system automatically generates localized invoices and triggers AI-driven customer follow-ups.

  **Acceptance Criteria**:
  1. Integrate the Stripe Terminal SDK into the Flutter mobile application to support Tap to Pay on iPhone and Android.
  2. Implement the Go backend services to handle PaymentIntents, coordinate with Stripe, and securely record the transaction in the multi-tenant PostgreSQL ledger.
  3. Build the backend logic to instantly generate and deliver a localized invoice upon successful payment.
  4. Integrate the event into the AI Agent Feed so the Finance Agent can summarize the revenue and the Customer Agent can draft a post-service review request.
  5. Ensure the mobile UI is fully responsive and optimized for a 375px viewport, utilizing translucent glass styling and clear touch targets.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
