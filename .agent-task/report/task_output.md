issue_title: "Implement Intelligent Payment Routing & Idempotency Tracking"
issue_description: |
  # Research Report: Intelligent Payment Routing & Idempotency

  ## Executive Summary
  This report details an architectural improvement for OneHumanCorp's payment processing system. It addresses critical scaling and reliability gaps for our small business operators (like Maya and Carlos) by implementing robust idempotency for all payment API calls and introducing intelligent payment routing to minimize transaction fees.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Stripe and Adyen provide advanced features like payment routing, where the payment method is selected to minimize cost. However, typical SMB platforms leave users to pay the default (and often highest) credit card fees without intelligent defaults. OHC must abstract this away, saving the business owner money automatically.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Maya (Home Baker) & Carlos (Field Service Owner).
  - **The Gap:** The current payment flow does not explicitly handle network flakes gracefully with idempotency keys across all payment endpoints, leading to the risk of double charging. Additionally, we are not automatically routing payments to the lowest-cost method (e.g., ACH for large invoices vs. Credit Card for small tap-to-pay purchases), which directly impacts the owner's bottom line.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Invariants
  - **Payment Intents Table (PostgreSQL):** Track `payment_id`, `stripe_payment_intent_id`, and `idempotency_key`. The `idempotency_key` must be unique per `tenant_id` to prevent replay attacks and accidental double charges.
  - **Multi-Tenant Isolation:** Standard RLS policies apply to the new payment tables.

  ### AI Agent Coordination
  - **Finance Agent ("The Accountant"):** The Finance agent will analyze transaction history and proactively suggest to the user if they should encourage ACH over Credit Cards for certain services based on the historical savings data calculated by the intelligent router.

  ### Mobile-First Implementation
  - No direct UI changes required, but the backend must reliably accept `idempotency_key` headers from the mobile client.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Intelligent Payment Routing & Idempotency

  **Target Persona:** Carlos the Handyman (Large ticket sizes, high fee sensitivity).

  **Outcome:** All payment endpoints become idempotent, and large transactions are automatically routed to ACH or lower-fee methods where applicable, saving the owner money invisibly.

  **Critical User Journey (CUJ):**
  1. Carlos sends a $1000 invoice via OHC.
  2. The customer clicks the payment link.
  3. The OHC intelligent router analyzes the $1000 amount and dynamically configures the Stripe checkout to prioritize ACH payments (saving Carlos ~$24 in fees).
  4. The network flakes during payment capture. Carlos's phone retries the request using the same `idempotency_key`. The OHC backend recognizes the key and safely returns the successful response without charging the customer twice.

  **Next Actions for Engineering:**
  - **Step 1:** Implement idempotency tracking in `src/server/api/payment_ledger.rs` or similar payment API endpoints.
  - **Step 2:** Refine the `PaymentRouter` in `src/server/integrations/stripe/routing.rs` to expose the optimal payment method configuration to the checkout/invoice generation flows.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
