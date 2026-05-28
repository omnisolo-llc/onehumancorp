import { defineConfig, devices } from '@playwright/test';
import * as path from 'path';

const retries = process.env.PLAYWRIGHT_RETRIES
  ? Number.parseInt(process.env.PLAYWRIGHT_RETRIES, 10)
  : process.env.CI
    ? 2
    : 0;

const reporter = process.env.PLAYWRIGHT_LIST_REPORTER
  ? [['list'], ['html']] as const
  : 'html';

const timeout = process.env.PLAYWRIGHT_TEST_TIMEOUT
  ? Number.parseInt(process.env.PLAYWRIGHT_TEST_TIMEOUT, 10)
  : 60000;

const actionTimeout = process.env.PLAYWRIGHT_ACTION_TIMEOUT
  ? Number.parseInt(process.env.PLAYWRIGHT_ACTION_TIMEOUT, 10)
  : 0;

export default defineConfig({
  testDir: './src/e2e',
  globalSetup: './src/e2e/global-setup.ts',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: Number.isFinite(retries) ? retries : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter,
  outputDir: './test-results/screenshots',
  timeout: Number.isFinite(timeout) ? timeout : 60000,
  use: {
    baseURL: process.env.BASE_URL || 'http://localhost:18789',
    actionTimeout: Number.isFinite(actionTimeout) ? actionTimeout : 0,
    trace: 'on-first-retry',
    screenshot: 'on',
    video: 'on',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
});
