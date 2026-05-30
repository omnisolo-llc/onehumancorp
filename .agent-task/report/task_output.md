issue_title: "[Architecture] Omni-Channel Tap-to-Pay Terminal Engine Design"
issue_description: |
  # Omni-Channel Tap-to-Pay Terminal Engine

  ## Problem Statement
  Priya (Boutique Owner, 35) struggles to maintain unified inventory between her physical storefront and her online OHC website. When she sells a dress in-store, she currently has to manually update her online inventory. Furthermore, accepting in-person payments currently requires an external POS terminal and a completely disjointed workflow from her online Stripe checkout. She needs a seamless, mobile-first Tap-to-Pay solution on her iPhone that instantly reconciles offline sales with her central tenant inventory ledger and triggers the same OHC Operations/Finance AI agents as an online purchase.

  ## Research Report
  **Competitive Landscape:**
  - **Shopify POS:** Offers unified inventory, but requires expensive proprietary hardware (Shopify Terminal) or a clunky, separate app setup process. It's often too "e-commerce heavy" for service-based or blended businesses.
  - **Wix/Squarespace:** Extremely limited native in-person POS capabilities. Relies on third-party integrations (like Square) which fragment the customer data and break the unified "AI Agent" promise.
  - **Square:** The dominant incumbent for in-person POS, but historically weak on complex, AI-driven online storefront capabilities out-of-the-box.

  **The OHC Advantage:**
  By utilizing Stripe Terminal's SDK (specifically Tap to Pay on iPhone/Android), OHC can turn the merchant's *existing* smartphone into the POS hardware. This requires zero upfront hardware cost. More importantly, because OHC's backend natively integrates Stripe Payment Intents with a rigorous multi-tenant inventory ledger (`tenant_id` RLS in PostgreSQL), an offline tap-to-pay transaction can fire the exact same domain events to the KAIROS engine as a web checkout.

  ## Design Doc

  ### Architecture Summary
  The Omni-Channel Tap-to-Pay Terminal Engine allows mobile clients (Flutter) to initiate in-person Stripe Terminal transactions that execute against the same unified API and data models as online checkouts.

  1. **Frontend (Flutter PWA/App):** Integrates Stripe Terminal SDK (via a Flutter plugin or native channels). Provides a 375px-optimized POS screen for building a cart from synced inventory and prompting the tap-to-pay UI.
  2. **Backend (Go/gRPC):** Exposes endpoints to provision Stripe Terminal connection tokens (`/api/v1/terminal/connection_token`) and process Terminal-based Payment Intents (`/api/v1/terminal/process_payment`).
  3. **Data Model:** The `orders` and `inventory_ledger` tables treat in-person and online sales identically, merely tagging the `channel` as `POS` vs. `WEB`.
  4. **AI Coordination:** Upon successful payment intent capture, the Finance Agent logs the revenue, the Operations Agent decrements inventory, and the Customer Success Agent (if the customer's email/phone was collected for a digital receipt) handles follow-ups.

  ### Mobile UX Flow
  - **Screen 1 (Catalog):** 375px layout. 2-column grid of products with large, tappable image cards. A persistent bottom bar shows "Cart (1 item) - $50".
  - **Screen 2 (Checkout):** A clean summary. A massive, primary "Tap to Pay" CTA button spanning the width of the screen.
  - **Screen 3 (Stripe Native):** The OS-level Tap-to-Pay overlay.
  - **Screen 4 (Success):** A satisfying micro-animation checkmark. Options to "Email Receipt" or "Done".

  ## Implementation Prompt
  **Objective:** Implement the backend foundation for the Omni-Channel Tap-to-Pay Terminal Engine.
  **Acceptance Criteria:**
  1. Create a secure, multi-tenant endpoint to generate a Stripe Terminal Connection Token. Ensure it enforces strict `tenant_id` isolation.
  2. Create an endpoint to generate a Stripe Payment Intent specifically structured for Terminal capture (e.g., `payment_method_types: ['card_present']`).
  3. Ensure these operations interact smoothly with the existing `inventory` and `order` tables. An order created via POS should look identical to a web order but marked with a distinct channel identifier.
  4. Emit an event upon successful payment capture that the KAIROS Orchestrator can route to the Finance and Operations AI Agents.
  **Constraints:** Adhere to all existing Go backend multi-tenancy patterns. Ensure 100% unit test coverage for the new API endpoints.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
