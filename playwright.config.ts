import { defineConfig, devices, type ReporterDescription } from '@playwright/test';
import * as path from 'path';

const inBazel = !!process.env.TEST_TMPDIR;

const retries = process.env.PLAYWRIGHT_RETRIES
  ? Number.parseInt(process.env.PLAYWRIGHT_RETRIES, 10)
  : process.env.CI
    ? 2
    : 0;

const reporter = process.env.PLAYWRIGHT_HTML_REPORT
  ? ([['list'], ['html', { outputFolder: process.env.PLAYWRIGHT_HTML_REPORT }]] satisfies ReporterDescription[])
  : process.env.PLAYWRIGHT_LIST_REPORTER || inBazel
    ? 'list'
    : 'html';

const timeout = process.env.PLAYWRIGHT_TEST_TIMEOUT
  ? Number.parseInt(process.env.PLAYWRIGHT_TEST_TIMEOUT, 10)
  : 60000;

const actionTimeout = process.env.PLAYWRIGHT_ACTION_TIMEOUT
  ? Number.parseInt(process.env.PLAYWRIGHT_ACTION_TIMEOUT, 10)
  : 0;

const outputDir = process.env.PLAYWRIGHT_OUTPUT_DIR
  ?? (inBazel && process.env.TEST_UNDECLARED_OUTPUTS_DIR
    ? path.join(process.env.TEST_UNDECLARED_OUTPUTS_DIR, 'playwright')
    : './test-results/screenshots');

const chromiumExecutable = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE;
const testDir = process.env.PLAYWRIGHT_TEST_DIR || './src/e2e';

export default defineConfig({
  testDir,
  globalSetup: './src/e2e/global-setup.ts',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: Number.isFinite(retries) ? retries : 0,
  workers: process.env.CI || inBazel ? 1 : undefined,
  reporter,
  outputDir,
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
      use: {
        ...devices['Desktop Chrome'],
        ...(chromiumExecutable
          ? {
              launchOptions: {
                executablePath: chromiumExecutable,
              },
            }
          : {}),
      },
    },
  ],
});
