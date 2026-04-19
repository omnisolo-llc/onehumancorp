import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for One Human Corp end-to-end tests.
 *
 * Prerequisites:
 *   cd deploy && docker compose up -d
 *   # Wait for all services to be healthy, then:
 *   cd tests/e2e && npm install && npm test
 *
 * The OHC_E2E_BASE_URL environment variable can override the target URL.
 * Defaults to http://localhost:8080 (the `server` service in docker-compose).
 *
 * Admin credentials are sourced from:
 *   OHC_E2E_ADMIN_USER  (default: admin)
 *   OHC_E2E_ADMIN_PASS  (default: admin)
 */
export default defineConfig({
  testDir: '.',
  testMatch: ['**/*.spec.ts'],
  timeout: 60_000,
  retries: process.env.CI ? 1 : 0,
  reporter: [
    ['list'],
    ['html', { outputFolder: 'playwright-report', open: 'never' }],
    ['json', { outputFile: 'playwright-results.json' }],
  ],
  use: {
    baseURL: process.env.OHC_E2E_BASE_URL ?? 'http://localhost:8080',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    trace: 'retain-on-failure',
    actionTimeout: 20_000,
    navigationTimeout: 30_000,
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
});
