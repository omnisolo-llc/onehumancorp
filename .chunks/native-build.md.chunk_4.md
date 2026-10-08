ri package and is excluded only from focused headless checks. Full `make test` and `make lint` include it and require native desktop dependencies.

```sh
cargo check --locked --workspace --exclude app --all-targets
cargo test --locked -p server_services_billing
cargo test --locked -p server_pricing
cargo test --locked -p server_harness
cargo test --locked -p server_integrations_stripe
cargo test --locked --workspace --exclude app
npm run test:contracts
npm run typecheck:web
npm run typecheck:e2e
npm run test:web
```

`typecheck:e2e` checks every source included by `playwright.tsconfig.json`, including archived contract files. It is required by `make lint` and the Node CI job. Static checking does not change native browser discovery or certify archived mock contracts as runtime acceptance.

Focused checks accelerate iteration; they do not replace the complete regression suite. Test output must include nonzero executed tests where tests are expected. No `--pass-with-no-tests`, hidden failures, disabled assertions or changed business expectations merely to obtain green output.

## Build once, consume real artifacts

```sh
make build-e2e
npm run desktop:build -- --debug --no-bundle
npx --no-install playwright install chromium
npm run test:e2e -- --workers=1
```

`build:web` compiles the maintained Next application and only then creates a source-bound proof and standalone package in `target/native-web`. Tauri owns a loopback Node server for this package. The authenticated Next server routes are retained; a static HTML export would remove necessary application behavior. The packaged executable is a separate official Node distribution, verified against the pinned release checksums and shipped with its license—not a copied CI `process.execPath` that might depend on shared host libraries.

The package manifest binds source, dependency lock, build ID, Node version, OS and architecture. Packaging refuses stale output even when dependency locks are unchanged. Environment files