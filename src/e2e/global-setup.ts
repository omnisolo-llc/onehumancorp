import { FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  // Extract baseURL from the first project config, or fall back to an environment variable
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!process.env.DATABASE_URL) {
    throw new Error('Playwright DATABASE_URL is missing. Please provide a DATABASE_URL to run e2e tests.');
  }

  if (!baseURL) {
    throw new Error('Playwright baseURL is missing. Ensure it is set in playwright.config.ts or via BASE_URL environment variable.');
  }

  // Optionally perform any setup steps required before the test suite runs:
  // e.g., Authenticate once and reuse state, initialize a database connection, etc.
  // const browser = await chromium.launch();
  // const page = await browser.newPage();
  // await page.goto(baseURL);
  // ... login ...
  // await page.context().storageState({ path: 'storageState.json' });
  // await browser.close();

  // console.log(`Playwright global setup complete. Target environment: ${baseURL}`);
}

export default globalSetup;
