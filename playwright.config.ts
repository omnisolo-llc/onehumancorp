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

const video = process.env.PLAYWRIGHT_VIDEO || 'on';
const screenshot = process.env.PLAYWRIGHT_SCREENSHOT || 'only-on-failure';
const storageState = process.env.PLAYWRIGHT_STORAGE_STATE;

export default defineConfig({
  testDir: process.env.PLAYWRIGHT_TEST_DIR || './src/e2e',
  globalSetup: './src/e2e/global-setup.ts',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: Number.isFinite(retries) ? retries : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter,
  outputDir: '/tmp/test-results/screenshots',
  timeout: Number.isFinite(timeout) ? timeout : 60000,
  use: {
    baseURL: process.env.BASE_URL || 'http://127.0.0.1:18789',
    ...(storageState ? { storageState } : {}),
    actionTimeout: Number.isFinite(actionTimeout) ? actionTimeout : 0,
    trace: 'on-first-retry',
    screenshot: screenshot as 'off' | 'on' | 'only-on-failure',
    video: video as 'on' | 'off' | 'retain-on-failure' | 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: {
        ...devices['Desktop Chrome'],
        launchOptions: {
          args: ["--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
          executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,
        },
      },
    },
  ],
});
