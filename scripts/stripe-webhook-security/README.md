# Active Stripe billing and ledger authenticity gate

Run `bash scripts/stripe-webhook-security/run.sh` with the repository Rust toolchain.
The focused gate derives its lock from the root lockfile and rejects different
registry versions/checksums. `generated.rs`, the derived lock and source manifest
are local generated artifacts, never checked-in absolute workspace paths.

## Production scope

- `/api/v1/webhooks/stripe` authenticates before JSON decoding, Redis claims or
  asynchronous dispatch. Its existing route is covered by a source-wiring guard.
- `/api/v1/payments/ledger/webhook` authenticates in its actual router before its
  JSON extractor or business handler. Standard Stripe `type` and existing legacy
  `type_field` are accepted individually; supplying both is rejected.
- Issuing remains unmounted and unchanged in this partial checkpoint. Its
  signature repair and new issuing-specific regressions are held separately;
  this gate does not certify its existing helper or approve/decline behavior.

The shared verifier computes HMAC-SHA256 over the original timestamp text, a dot,
and the untouched request bytes; it compares every bounded `v1` candidate using
`Mac::verify_slice`. Other signature versions cannot authorize the request.
Exactly one timestamp and at least one valid-length hexadecimal `v1` are required.
Past and future clock skew are bounded to 300 seconds; arithmetic cannot overflow.
Duplicate HTTP signature headers are rejected. Bounds are 1 MiB body, 4096-byte
header/secret and 16 signatures. Raw-body collection precedes parsing and restores
the same bytes for the existing handler. Invalid authentication returns 401;
oversized bodies return 413; missing or invalid secret configuration returns 503.

Billing reads the endpoint signing secret from `STRIPE_WEBHOOK_SECRET` or
`STRIPE_WEBHOOK_SECRET_FILE`. Ledger independently reads
`STRIPE_LEDGER_WEBHOOK_SECRET` or `STRIPE_LEDGER_WEBHOOK_SECRET_FILE`.
Exactly one source per endpoint is allowed, using the existing bounded secret
loader. There is no API-key or cross-endpoint fallback. This code does not install
or change any secret. An unconfigured endpoint deliberately fails closed.

## Verification fidelity and limitations

The gate also compiles the full real Stripe library as a path dependency and
exercises its public shared signature verifier. The original client/issuing
source and tests are unchanged; its full crate unit suite is separate. The
generated harness imports the actual cryptographic verifier and HTTP adapter. It extracts the exact shared billing middleware and
ledger router/types, then sends real loopback TCP HTTP requests through Axum.
Redis access and business dispatch are recording sentinels. No issuing provider
boundary is exercised by this partial gate. No provider request, real credential or payment is
used. The independent fixed HMAC vector was produced with Python `hmac/hashlib`.
The existing billing test signing helper is also compiled and checked, but the
old full DB/Redis billing suite is not included. The full main server, actual
ledger mutations, issuing provider transport and whole-workspace acceptance
remain separate gates. Focused results do not certify those boundaries.

The harness process clears the four endpoint secret variables before testing,
sets only public fixture strings, and runs tests serially. Production tests that
need a fixture key serialize and restore their process-local configuration.
A before/after source fingerprint rejects concurrent input changes.

## Explicit remaining acceptance requirements

Signature authenticity does not establish completion, tenant authority, duplicate
safety, currency/amount correctness, account binding or provider delivery.
The existing billing middleware still acknowledges before its in-memory task
finishes, uses a pre-effect Redis marker and can proceed when Redis is unavailable.
Crash recovery and durable idempotent effects need separate work. Ledger and
billing still need authoritative account/livemode/tenant and local payment-object
binding checks; signed provider metadata alone must not become tenant authority.
Issuing still needs a separately approved hard-budget/concurrent-reservation and
real `pending_request.amount` policy before any activation. Its current top-level
amount policy is not certified by these authenticity tests. The relay URL mismatch
(`/api/v1/billing/webhook/stripe`) is not remounted or silently redirected.
Other provider branches of the shared middleware are unchanged and not certified.

Protocol sources:
- https://docs.stripe.com/webhooks#verify-webhook-signatures-manually
- https://docs.stripe.com/webhooks/signature
- https://github.com/stripe/stripe-go/blob/master/webhook/client.go
- https://docs.stripe.com/connect/webhooks
- https://docs.stripe.com/issuing/controls/real-time-authorizations

## Held issuing scope

This partial checkpoint leaves `issuing.rs` and `client.rs` byte-identical to
remote recovery commit `33da305a3af0faf2d4c9c8a34091a9578e59e9db`. The complete
reviewed issuing implementation and its new regression tests remain preserved
in held commit `2c34ff035947f15a2e9e8858d63eee9e758688e7`; they are not silently
reported as passing here. Running those two new issuing authentication tests
against this unchanged helper produced two failures, while the21 active-route
and verifier cases passed. The helper still does not authenticate ignored
events, and the original two needless-borrow lint findings remain. Do not
activate that unmounted helper or infer full Stripe safety from this gate.

Only the new issuing-specific tests/implementation were left out of the partial
change. All original repository tests and the original client test remain.
The active route tests and complete shared-verifier regressions are retained.
