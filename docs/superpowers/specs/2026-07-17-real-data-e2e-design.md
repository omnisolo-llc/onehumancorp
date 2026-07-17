# Real-data browser E2E design

## Objective

Browser E2E tests must exercise the production UI against real application services, deterministic PostgreSQL seed records, and valid image files. Production and E2E code must not substitute mock, sample, dummy, or locally fabricated business data when a real dependency is unavailable.

## Data and authentication flow

The Bazel Playwright runner starts PostgreSQL, Valkey, the Rust server, and Next.js. It applies `src/e2e/e2e-seed.sql` before tests start. Seeded rows use stable identifiers so browser assertions can prove that visible content originated in PostgreSQL.

Playwright global setup authenticates a seeded administrator through the real `/api/v1/auth/login` endpoint and saves the returned cookie state. Protected-page tests reuse that genuine session. A dedicated login test starts with empty storage and submits the real login form, preserving end-to-end coverage of the browser login path. User-switching fixtures authenticate the requested seeded user through the same real endpoint; they never manufacture tokens or tenant headers.

## Real images

Image journeys use a valid tracked image fixture and send it through the same upload or application API used by production. Seeded image metadata must reference a resource the test stack can actually serve. Assertions verify that the browser decoded the image by checking non-zero intrinsic dimensions, not only the presence of an `img` element or filename.

## Enforcement

The CI-selected Playwright spec contract rejects network interception and response fabrication, including `page.route`, `context.route`, `route.fulfill`, and `page.setContent`. It also rejects fake image buffers and mock, dummy, or sample business payloads in selected browser tests. The shared runtime fixture rejects route interception as defense in depth.

The production real-data contract runs in CI. Production UI may show loading, empty, unavailable, or error states, but it must not replace failed or empty API results with sample records. Existing legacy exceptions are tracked as debt and may only shrink; new exceptions are forbidden.

## Failure behavior

Missing seed data, failed authentication, unavailable services, and undecodable images fail the E2E test with a boundary-specific message. Tests do not convert those failures into synthetic success states. Secrets and session values are never printed in logs or committed.

## Verification

The acceptance sequence is:

1. Contract tests prove selected E2E specs contain no prohibited substitutions.
2. Authentication regression tests prove seeded credentials create a real session and invalid credentials fail.
3. A representative protected-page test proves seeded database content is rendered.
4. A representative image test proves a valid image is transported and decoded.
5. The genuine Playwright CI shard passes.
6. A fresh `bazel test //...` passes before the changes are pushed.
