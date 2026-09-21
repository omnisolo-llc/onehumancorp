# Backend mutation outcomes

The authenticated Next transport must not interpret an interrupted write as proof
that no business action happened. In particular, the default web timeout is
shorter than the Stripe checkout client's timeout. Cancelling the web request
cannot roll back a payment request or an approval already received by Rust.

## Response contract

For a **proxy-generated failure after dispatch** of POST, PUT, PATCH or DELETE,
the transport preserves the HTTP status and existing `error` string and adds:

```json
{
  "error": "backend timeout",
  "code": "BACKEND_OUTCOME_UNKNOWN",
  "outcome": "unknown",
  "reconciliation_required": true,
  "retry_safe": false,
  "recovery": "The request may have been applied. Check its recorded outcome before retrying."
}
```

This applies to a failed fetch, timeout, caller cancellation, rejected redirect,
response size violation or interrupted buffered response. It is a conservative
classification: a transport exception does not establish whether the request
reached the server. `unknown` does not mean successful, failed, paid or cancelled.

Use the **backend method**, including trusted route overrides, to classify the
operation. POST-to-GET read adapters do not acquire mutation semantics just
because the browser used POST. Existing GET/HEAD errors, pre-dispatch validation
failures and complete backend responses retain their response contracts.

The transport rechecks cancellation after asynchronous body/path preparation,
immediately before calling fetch. Work cancelled before dispatch is not sent and
is not labelled as an unknown backend effect. The transport neither generates a
new idempotency key nor automatically repeats a write. An existing key remains
unchanged; its presence alone does not prove the backend implements safe replay.

All generated errors remain private and non-cacheable. Recovery metadata is
static and does not echo credentials, upstream exception text or request bodies.

## Caller responsibilities and boundaries

Business-action clients should show `recovery` when the unknown-outcome code is
present and consult the authoritative operation, approval or provider record
before offering a repeat action. Do not map an unknown response to either a
completed outcome or a fresh payment attempt. Existing screens that read only
`error` retain their old message; this transport patch does not update them.

This is not a durable execution queue, provider reconciliation worker, atomic
payment ledger or exactly-once delivery guarantee. It does not repair swallowed
Rust handler errors, global provider credential lookup or approval-to-execution
persistence. Complete upstream responses are passed through; the transport does
not certify the truth of their business claims. An SSE response already handed
to the caller cannot be replaced with this JSON envelope; stream interruption
handling still needs an operation-specific contract. Neither this code nor its
tests authorize deployment, outreach, live provider writes or customer charges.

## Verification

The regression suite is included automatically by the existing
`test:scripts` discovery (`scripts/*.test.mjs`):

```sh
node --test scripts/backend-mutation-outcome.test.mjs
```

It executes the production TypeScript transport and URL validator after syntax
transpilation. Session verification is substituted at the module boundary, so
these tests do not certify authentication. Simulated upstream failures cover
mutation methods, method overrides, timeouts, cancellation, redirection, bounded
bodies and unchanged successful/conflict responses. Two loopback HTTP tests use
native fetch to show that a server can receive a write before the client loses
its reply or aborts. No external provider is contacted.

On the reviewed source, the 23-case suite has 15 failing cases and 8 passing
compatibility cases. With the patch, all 23 pass, with no skips or cancellations.
Reverting only the transport change reproduces the failures. These are focused
results, not full repository acceptance: `make lint`, `make test`, the pinned Node
runtime, complete authentication tests, Rust, real-stack E2E and hosted CI remain
required. The development sandbox used Node 22.16.0 rather than the repository's
pinned 22.22.1 and did not contain Cargo, Docker or the locked dependency trees.
