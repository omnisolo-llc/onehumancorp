# Configured checkout session browser fixture

This fixture exercises real application session creation and inventory exclusion
at a configured, owned HTTP provider boundary. It is **not evidence of payment,
Stripe sandbox integration, hosted Checkout loading, or paid-webhook settlement**.
Those acceptance requirements remain open.

`src/e2e/support/configured_checkout_fixture.ts` starts a separate backend and
Next process pair only for the configured journey. The regular native runner
remains Stripe-unconfigured so missing-configuration tests retain their meaning.
The fixture reuses the same build outputs, runner-owned disposable PostgreSQL and
cache, and fresh tenant/product rows. It does not compile another application or
copy dependencies. Existing backend listeners bind `0.0.0.0`; callers use fresh
loopback origins. The provider, egress proxy, and Next bind only `127.0.0.1`.

Before startup, the helper validates the runner's private capability file, live
PostgreSQL container identity, native producer proof, and existing source-bound
Next manifest. Native proof includes byte hashes of both server and agent plus
a digest of tracked and untracked source. It rejects changes between compiler
snapshot and recording, later source drift, altered binaries, and source/file
symlinks. Build/dependency output, local environment files and Next-generated
declarations are excluded. Merely matching a Git commit is insufficient.

The existing native build is bracketed by these commands (paths should use the
selected Cargo target directory for local custom-target builds):

```sh
node scripts/native-binary-proof.mjs snapshot target/debug/native-source-snapshot.json
cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins
node scripts/native-binary-proof.mjs record target/debug/native-source-snapshot.json target/debug/native-binary-proof.json
```

The producer proof travels with the existing native artifact. These commands
must surround a successful actual compiler invocation; generating a receipt for
old unrelated binaries is not validation.

## Browser helper contract

```ts
const checkout = await startConfiguredCheckoutFixture();
try {
  await checkout.register({ tenantId, productId, title, amountCents });
  const context = await browser.newContext({
    baseURL: checkout.origin,
    storageState: { cookies: [], origins: [] },
    proxy: checkout.proxy,
    serviceWorkers: 'block',
  });
  // Authenticate through the real application and perform actual UI actions.
  // Always close each browser context before the fixture.
  const evidence = checkout.evidence();
  await context.close();
} finally {
  await checkout.close();
}
```

Registration checks exact owned PostgreSQL product terms. The provider accepts
only a registered `e2e-*` tenant/product pair and the complete matching Stripe
form: payment mode, USD minor-unit amount, quantity one, card method, product
metadata, owned return URLs, Basic synthetic authentication, and a checkout UUID
operation key. It records the exact raw body and decoded fields. Duplicated form
fields, mismatched terms, unregistered tenants, repeated operation IDs and
unknown endpoints fail. Issued sessions always have `payment_status: unpaid`.

The returned URL satisfies the unchanged production Stripe-hosted redirect
allowlist. A dedicated browser proxy forwards only the exact owned application
origin. Playwright's API client uses CONNECT even for HTTP, so tunnels are allowed
only to the exact validated application host and port. Every other CONNECT target,
including other loopback ports and host aliases, is rejected without resolving or
connecting to that target. An observed refused `checkout.stripe.com:443` CONNECT is navigation intent,
not a loaded hosted page. Do not use Playwright route substitution or replace
application API responses. Use both the observed browser URL request and the
proxy refusal when asserting this handoff.

Application child environments use an explicit allowlist, new authentication
secrets, an isolated home directory and a newly generated synthetic Stripe key;
inherited provider keys, proxy settings, and Node preload options are excluded.
Fresh owned process groups are stopped with bounded termination and forced
cleanup. Exact provider/proxy records and sanitized 32 KiB log tails per process
are retained in `test-results/checkout-fixtures/<run>/<fixture>/evidence.json`,
including application startup failures.

## Verification disposition

The focused Node suite covers the provider request/receipt contract, foreign
terms and credentials, safe return URL equivalence, real HTTP forwarding and
CONNECT refusal, environment filtering, source and executable drift, private
runner-proof rejection, and CI ordering. Run:

```sh
node --test --test-concurrency=1 scripts/checkout-browser-fixture.test.mjs
```

These small tests do not start a backend, Next, or Chromium and cannot certify
the application race. The maintained browser journey must assert exactly one
actual cash receipt or one validated unpaid session/stock hold, opposite-path
rejection, and PostgreSQL stock/receipt readback. Complete `make lint`, `make
test`, fresh native/Next startup, browser execution and hosted CI remain the
required acceptance gates; a focused fixture pass does not replace them.
