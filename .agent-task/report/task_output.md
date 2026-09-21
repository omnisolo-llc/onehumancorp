issue_title: "🔎 Scout: Tool Integration Research - Payment & Booking Checkout Remediation (OHC-01 / F08)"
issue_description: |
  # Payment & Booking Checkout Remediation (OHC-01 / F08)

  ## Problem Statement
  Currently, the system generates "checkout-looking" URLs manually (e.g., `https://checkout.stripe.com/pay/cs_test_{uuid}`) instead of generating real payment sessions via a payment provider like Stripe. This violates the `F08` finding, which states: "Invoice and booking helpers fabricate checkout-looking URLs. Required remediation: Real provider session or explicit pending/unavailable state; persist provider IDs, validate money, idempotent retries." It also blocks `OHC-01`: "Trace the active service journey; eliminate false booking/invoice payment success."

  ## Research Report
  - **Codebase Findings:**
    - In `src/server/api/invoice.rs`, `stripe_payment_link` is populated but no real Stripe interaction occurs for checkout sessions. There is some webhook logic for Stripe, but session generation is mocked or hardcoded to `cs_test_{uuid}` in several places.
    - In `src/server/api/booking/public.rs`, `create_checkout_session` hardcodes `format!("https://checkout.stripe.com/pay/cs_test_{}", booking_id)`.
    - In `src/server/services/booking.rs`, `create_conversational_checkout` also generates a fake `cs_test_` URL.
    - `src/server/domain/booking.rs` and `src/server/domain/quotes.rs` have similar hardcoded URL formatting.
  - **Provider Context (Stripe):**
    - Stripe Checkout Sessions provide a hosted payment page.
    - To replace the fabricated URLs, we need to call Stripe's `/v1/checkout/sessions` API to generate a real `url` and `id` (or return an explicit "unavailable" state if Stripe isn't configured, as required by the audit).
  - **Operator Reality:**
    - Non-technical operators need real payments to work out of the box (or explicitly show "Payments not connected"). Fabricated URLs that look real but fail confuse operators and their clients.

  ## Design Doc
  - **Architecture:**
    - Introduce a real Stripe integration service or use an existing one if available (likely `src/server/api/billing_webhook.rs` implies some Stripe interaction already).
    - If a Stripe key is not available, the system MUST return an explicit pending/unavailable state rather than a fabricated URL.
    - Persist the real Stripe Session ID and URL in the database where applicable (`bookings`, `invoices`, `quotes`).
  - **Integration Points:**
    - `src/server/api/booking/public.rs`: Modify `create_checkout_session` to attempt to create a real Stripe session. If Stripe is unconfigured, return `stripe_url: null` and a status indicating setup is required.
    - `src/server/api/invoice.rs`: Ensure invoice creation or payment link generation creates a real link.
    - `src/server/services/booking.rs`: Same for conversational checkout.
  - **UI/UX Flow:**
    - Mobile & Desktop flows currently expect a URL string. If `null`, the UI should prompt the owner to connect their payment provider.

  ## Implementation Prompt
  Implement real Stripe Checkout Session generation to replace fabricated `cs_test_` URLs across bookings, quotes, and invoices.
  1. Audit `src/server/api/booking/public.rs`, `src/server/services/booking.rs`, and `src/server/domain/` for fabricated `cs_test_` URLs.
  2. Implement a Stripe client function to create a checkout session using the `stripe` crate or direct HTTP.
  3. Update the endpoints to use this function. If Stripe is not configured (missing key), the endpoints must return an explicit `null` for the URL and/or an appropriate status, rather than generating a fake link.
  4. Ensure database records (e.g., `stripe_payment_link`, `stripe_invoice_id`) store the real values returned from Stripe.
  5. Add/update tests to verify this behavior (e.g., verifying that without Stripe, no fake URL is returned).

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - OHC Target ID: OHC-01 (eliminate false booking/invoice payment success), addresses F08.
  - Stage: Run
  - Gap: Observed defect (fake URLs).
  - Evidence: Direct codebase audit confirming hardcoded `cs_test_` URLs.
  - Result: Real payment sessions or explicit failure states; 0 fabricated URLs.
  - Non-goals: Full Stripe Connect onboarding in this ticket.
  - Authority Class: Routine external work.
  - Cost plan: N/A (Standard Stripe fees apply to customer).
  - Acceptance: `make test` passes, E2E tests for booking/invoices show real or explicitly missing URLs, NO `cs_test_` fake URLs generated.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
