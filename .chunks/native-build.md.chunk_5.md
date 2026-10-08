and build caches are excluded from standalone, public and static content. Never merge old artifacts into a new bundle. `OMNISOLO_PREBUILT_WEB` may reuse a validated artifact from the same workflow/source/platform, not an arbitrary cached directory.

`make build-e2e` also snapshots native producer source before Cargo and records the resulting server and agent hashes after Cargo succeeds. The proof lives beside those binaries in `${CARGO_TARGET_DIR:-target}/debug/native-binary-proof.json`. The configured checkout session tests require this proof and refuse stale or unproved binaries. A direct Cargo build alone does not produce it; use the maintained Make target before local E2E execution.

The inventory session tests start a separate backend and Next process pair from the verified outputs, using the runner-owned database, fresh test tenants and an owned loopback provider. The provider accepts only registered checkout terms and returns an unpaid session. A browser proxy allows only the owned application, including Playwright API CONNECT tunnels to that exact host and port, and refuses every other CONNECT target, including the observed HTTPS Stripe redirect. These tests establish cash receipt versus online session/stock-hold exclusion. They do not exercise hosted payment or a paid webhook. The existing backend binds all interfaces inside the test runner; the provider, proxy and Next bind loopback. Global unconfigured-provider tests keep their original environment.

The Rust API runs separately, either locally or on a configured HTTPS host. Desktop owns only its packaged Node process; it must not claim to provision or supervise a missing Rust backend. Mobile builds point at explicit HTTPS `OMNISOLO_MOBILE_WEB_URL` and do not bundle a desktop Node runtime. Platform SDKs, signing keys, store enrollment and actual device/install tests remain separate release prerequisites.

## Action-owned CI setup

`make init` and `make doctor` are local developer commands, not CI setup st