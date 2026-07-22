import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './src/e2e/tests',
  fullyParallel: true,
  reporter: 'line',
  use: {
    trace: 'on-first-retry',
  },
});
