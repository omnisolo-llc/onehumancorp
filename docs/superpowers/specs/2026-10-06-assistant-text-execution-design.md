# Assistant text execution (AW-02 / AW-03)

The approved goal is to replace simulated task success with useful, admitted text
inference and verifiable persisted output. This lane implements one bounded text
request. Coding, files, web research, autonomous delegation, and long-running work
remain explicitly unsupported. No live provider setup, spending, or deployment
is part of the implementation checks.

## Reuse and authority

Reuse the mounted `WorkflowExecution` admission, tracked worker, configured
text-only provider, funding reservation/settlement, cancellation and immutable
receipts. Do not use the generic consumer, in-memory engine, or S3 placeholder.
Every read and mutation verifies the current owner/admin membership and token.
The same receipt-store transactions enforce current tenant/actor identity and
commit-time revocation fences for assistant metadata.

## Data and recovery

Canonical receipt-database tables associate assistant task metadata with immutable
receipt attempts; these are metadata, not a second queue. An admitted receipt is
reserved first; the scoped metadata and attempt link commit together before
worker dispatch. An association failure fails closed. Replaying the same request
can recover its association but can never recover a consumed in-memory execution
capability. An unclaimed receipt eventually becomes cancelled, while interrupted
in-flight effects remain unknown. Existing legacy assistant records remain
visible as unexecuted, with their stored metadata preserved.

Creation requires a browser-generated UUID idempotency key. The exact draft and
key survive uncertain transport and reload in owner-scoped browser local storage, bound to the
current signed session scope. Readback by that key reconciles acceptance; no
synthetic running task or automatic new request is created. Status and result
content are derived only from the receipt. Archive is independent of lifecycle.
Stop invokes the existing cancellation operation; dispatching stop means unknown,
not a promise to undo a provider effect. Resume requires a definitively cancelled
receipt and explicit new request identity/source receipt. A compare-and-swap
association admits at most one new attempt. Unknown attempts cannot resume.

## Delivery and verification

The UI exposes text-only capability, receipt identity, real output, status,
refresh, stop, archive/unarchive and safe new attempts. Legacy history remains
readable; unimplemented artifact/tool actions do not imply execution. Focused
production-source-bound HTTP, SQLite schema and browser-component contracts cover
replay, changed payload, denied authority, unavailable provider, unknown outcome,
restart/readback, tenant isolation, output and archival invariants. Native Rust,
PostgreSQL and full browser/native acceptance remain required gates when their
runtimes are unavailable locally.
