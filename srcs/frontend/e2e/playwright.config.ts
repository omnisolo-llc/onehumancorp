import { defineConfig, devices } from '@playwright/test';
export default defineConfig({
  testDir: '.',
  use: {
    baseURL: 'http://127.0.0.1:8081',
    trace: 'on-first-retry',
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
});
