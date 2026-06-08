# OHC End-to-End Tests

This directory contains the Playwright E2E suite for the OneHumanCorp application.

## Running Tests Locally

Some E2E tests target the mock legacy application UI built in Next.js instead of the primary Tauri application.

If tests are failing locally due to a `net::ERR_CONNECTION_REFUSED at http://localhost:3000`, you should start the legacy mock UI server before running tests:

```bash
# 1. Start the backend services via docker or locally
bazelisk run //src/server:server

# 2. In another terminal, start the Next.js UI mock app
cd src/ui/next
npm run dev &

# 3. In a third terminal, run the tests
DATABASE_URL=postgres://ohc:ohc@localhost:$PG_PORT/ohc npx playwright test src/e2e/viral_growth_loops.spec.ts
```

In CI, the tests will fallback to basic smoke verification when appropriate or automatically handle environment orchestration via Bazel.
