# Next.js Mock Server Port and Timeout Issues

This task ticket documents the issues encountered while trying to run the Playwright E2E tests targeting the Next.js UI mock app.

## Description
When running Playwright tests locally (e.g., `src/e2e/cross_device_onboarding.spec.ts` or `src/e2e/onboarding.spec.ts`), the tests fail because:
1. The Next.js development server running on `src/ui/next` either fails to start on port 3000 due to port collisions, or automatically attempts to fall back to ports 3001, 3002, etc.
2. Playwright tests are strictly hardcoded or configured to expect the server on `http://localhost:3000`. This leads to `net::ERR_CONNECTION_REFUSED at http://localhost:3000`.
3. Even when port `3000` is freed and the Next.js server runs successfully, the tests timeout after 60,000ms while trying to resolve basic locators (`page.getByPlaceholder`, `page.getByText`, etc.), indicating that the Next.js frontend state does not match the expectations of the tests, or there is an issue syncing up the Playwright environment with the mocked API responses.

## Action Items
- Investigate process cleanup/port exhaustion issues with Next.js in the local workspace.
- Re-align the Next.js UI components and labels with the current Playwright test assertions (e.g. "Tell us about your business" vs "What's the name of your business?").
- Consider migrating the E2E tests to run against the Tauri app instead, as `src/ui/next/` is marked as a legacy prototype in the `README.md`.
