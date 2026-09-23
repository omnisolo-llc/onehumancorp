issue_title: F08 is already remediated in code but was marked as Open
issue_description: |
  I investigated F08 "fabricated checkout links" in `invoice.rs:45-48` and booking helpers.
  The code has already been changed by a prior commit:
  - `src/server/api/invoice.rs`: The fabricated UUID checkout url generation at line 45 has been removed. It now has `let stripe_payment_link = String::new();` with a comment explaining it's a local draft and "never invent a payable URL".
  - `src/server/api/proposals.rs`: Proposal approval now uses `create_checkout_session_idempotent` with a real `StripeClient`.
  - `src/server/api/booking/reserve.rs`: `reserve_slot` uses `create_booking_checkout` to generate a real checkout session if a deposit is required, and handles the `None` case otherwise. `trusted_stripe_checkout_url` is used to validate Stripe URLs.
  - `src/server/api/booking/public.rs`: Sets `stripe_url = None` when a deposit is required, satisfying "explicit pending/unavailable state".

  Since the codebase is already compliant with the required remediation for F08, no further code changes are needed. I have restored the open status in the ledger to avoid violating instructions.

issue_priority: P1
issue_category: remediation
issue_type: no-work
issue_label: ohc:lane:finance
assignees: []
