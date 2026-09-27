issue_title: 'Blocked No-Work Finding: F08 Checkout Link Fabrication Re-evaluation'
issue_description: |
  The user requested an evaluation of OHC-01 and F08 ("fabricated checkout links").
  Previous audit noted: `invoice.rs:45-48` fabricated a checkout URL, returning a fake UUID link for Stripe payments without creating a real session.

  However, examining `src/server/api/invoice.rs`, the `create_invoice` handler inserts a draft invoice and explicitly sets `let stripe_payment_link = String::new();`.
  There is no UUID generation for the URL anymore. The file contains a comment:
  `// This operation creates a local draft, not a provider checkout session.`
  `// Empty means payment has not been configured; never invent a payable URL.`

  Furthermore, `generate_invoice_handler` explicitly returns `StatusCode::NOT_IMPLEMENTED` and a clear message: "Create an invoice with approved line items; no invoice was generated for this job."
  In `proposals.rs`, the system now correctly interacts with the StripeClient to create a session: `stripe_client.create_checkout_session_idempotent(...).await`, and `safe_checkout.rs` implements real HTTPS calls to Stripe API.

  The issue F08 is already implemented according to the "Current continuation" section of `native_migration_and_remediation.md` ("Invoice creation returns a draft without invented Stripe IDs/URLs"). The user's prompt indicated: "Do not reopen an already repaired defect from its historical description."
issue_priority: P0
issue_category: backend
issue_type: blocked
issue_label: ohc:lane:finance
assignees: []
