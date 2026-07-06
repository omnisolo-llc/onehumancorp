import { chromium, type FullConfig } from '@playwright/test';


export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  // The Bazel test runner starts a local postgres instance on a random port and exports it via DATABASE_URL
  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) {
    throw new Error('DATABASE_URL is not set in the environment. Tests must run with a valid database.');
  }

  // Ensure there are no hardcoded localhost:5432 ports in use
  if (databaseUrl.includes('localhost:5432') && process.env.CI) {
    throw new Error('Playwright tests must use the Bazel-provided test database URL/port. Hard-coded localhost:5432 is not allowed.');
  }

  // wait for app to be ready
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/', baseURL));
      if (response.ok) return;
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}
