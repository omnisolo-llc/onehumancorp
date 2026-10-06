# Terminal Payment Integrity Implementation Plan

Goal: reject unowned/unbound capture and persist an exact amount-only terminal operation before provider effects.

Architecture: one authenticated Rust implementation and thin canonical Next aliases. Durable PostgreSQL operation state binds tenant, operation, provider connection fingerprint, integer amount/currency and Stripe intent ID; atomic state transitions forbid blind retries after unknown outcomes. Legacy/unbound records and catalog/cart flows fail closed pending a transactional cart/reservation implementation.

Constraints: base 4c56263; no live Stripe, credentials or remote writes. Audit checkout remains frozen. Preserve failed/unknown operation records. No claim of complete POS remediation. Root owns aggregate acceptance and publication.

1. Add failing real Next transport tests for Result-envelope normalization, canonical aliases, idempotency forwarding and rejection handling. Run red.
2. Add the durable operation SQL/state contract and provider-spy tests. Verify red against the absent/unsafe implementation; Rust/PG execution is explicitly blocked here because those runtimes are absent.
3. Implement validated, amount-only intents and capture preflight with scoped reads, exact binding, committed operation claims and checked provider receipts. Register migration 1040; unbound products/carts reject without a provider request.
4. Normalize all selected Next aliases and retain stable client attempt identity; repeated/uncertain attempts must not start another payment.
5. Run focused Node/SQL contracts, lint/typecheck as available, source-diff checks and independent review. Supply patch, manifest, exact results and remaining Rust/PG verification gaps.

Review focus: foreign/missing intent IDs; changed amount/provider; simultaneous/restarted requests; failed provider/DB commit; alias bypass. Do not mark provider authorization as capture, or checkout creation as delivery.

Review v3: replace the legacy offline worker settlement body with canonical persisted receipt acknowledgment; preserve all producer classifications and raw payloads. Explicit cash/inventory receipts may finish only after identity validation; unknown/card/legacy work remains held. Reject contradictory cash evidence before canonical producer effects. Add source-bound producer-envelope/worker tests and real producer persistence regressions. Native verification remains a hosted-CI gate.
