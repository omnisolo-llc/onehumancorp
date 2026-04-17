import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for the React web app E2E tests.
 *
 * All AI model responses are mocked via page.route() so no external API
 * keys or live backend is required.  The web app must be served at the URL
 * specified by WEB_APP_BASE_URL (defaults to http://localhost:3000).
 *
 * The webServer section starts a dev server automatically when running
 * outside Bazel.  In Bazel, the test wrapper script starts the server and
 * passes WEB_APP_BASE_URL.
 */
export default defineConfig({
  testDir: './e2e',
  testMatch: ['**/*.e2e.spec.ts'],
  timeout: 30_000,
  retries: process.env.CI ? 1 : 0,
  reporter: [['list'], ['json', { outputFile: 'e2e-results.json' }]],
  use: {
    baseURL: process.env.WEB_APP_BASE_URL ?? 'http://localhost:3000',
    screenshot: 'only-on-failure',
    video: 'off',
    actionTimeout: 10_000,
    navigationTimeout: 20_000,
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  // webServer is only used when running outside Bazel (local development).
  // In Bazel, the server is started by the sh_test wrapper.
  webServer: process.env.PLAYWRIGHT_BASE_URL
    ? undefined
    : {
        command: 'npm run start',
        url: 'http://localhost:3000',
        reuseExistingServer: !process.env.CI,
        timeout: 60_000,
      },
});
