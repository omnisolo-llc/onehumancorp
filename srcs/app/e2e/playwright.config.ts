import { PlaywrightTestConfig } from '@playwright/test';

const baseURL = process.env.PLAYWRIGHT_BASE_URL || 'http://localhost:8080';

const config: PlaywrightTestConfig = {
  testDir: '.',
  use: {
    baseURL: baseURL,
  }
};
export default config;
