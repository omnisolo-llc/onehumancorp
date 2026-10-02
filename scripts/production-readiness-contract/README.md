# Mounted production readiness containment

This gate imports the exact seven route expressions from `src/server/lib.rs` and
executes them through real Axum HTTP requests. It does not replace handlers or
return test-only application data. Each endpoint is requested twice with an
untrusted fabricated success body; every response must be HTTP501, noncacheable,
and explicitly unavailable, with no invented receipt or business data.

The affected legacy endpoints are organization dashboard, cost summary, approval
request/decision, handoff creation, skill import and snapshot creation. Their old
constant E2E responses did not implement those capabilities. The replacement is
safe containment, not completion: each endpoint still needs authenticated tenant
scope, durable implementation, restart/replay tests and a genuine outcome before
it may claim success. Existing in-memory maps are not durable replacements.

Run `bash scripts/production-readiness-contract/run.sh`. The runner uses only
root-locked dependencies, requires at least seven non-skipped runtime tests and
rejects source changes during execution. The required native Rust CI lane runs
this gate; the ordinary root Node suite also guards the mounted wiring.

The focused HTTP router excludes the unrelated server startup/dependency graph.
It certifies these exact route handlers, not whole-application startup, UI error
presentation, live provider behavior or the broader feature set. Voice upload,
Twilio audio, delivery quotation, development seed-route exposure and registry
provider-state semantics remain separate repair and production-readiness work.
