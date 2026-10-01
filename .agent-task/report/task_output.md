issue_title: Blocked: Unable to perform Lens Audit due to test infrastructure failures
issue_description: |
  **Observation:**
  The `lens_audit.spec.ts` E2E test suite cannot be run due to underlying test infrastructure failures.

  Running `make build-e2e` fails during `npm run build:web` with `next: not found`. After installing dependencies with `npm install` and running `make build-web`, the E2E test scripts run but their output was truncated. Due to the inability to complete the UI audit and visual verification natively, the UI visual audit process cannot proceed.
