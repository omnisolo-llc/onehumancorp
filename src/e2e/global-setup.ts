import { request as playwrightRequest, type FullConfig } from '@playwright/test';
import { authenticateRequest } from './authenticate';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for E2E global setup.');
  }

  const storageStatePath = process.env.PLAYWRIGHT_STORAGE_STATE;
  if (!storageStatePath) {
    console.warn("PLAYWRIGHT_STORAGE_STATE is not set.");
  }

  // The Bazel test runner starts a local postgres instance on a random port and exports it via DATABASE_URL
  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) {
    throw new Error('DATABASE_URL is required; E2E tests must use the Bazel-provided PostgreSQL database.');
  }

  // Ensure there are no hardcoded localhost:5432 ports in use
  if (databaseUrl.includes('localhost:5432') && process.env.CI) {
    throw new Error('E2E tests must use the Bazel-provided random PostgreSQL port, not localhost:5432.');
  }

  let appReady = false;
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/login', baseURL));
      if (response.ok) {
        appReady = true;
        break;
      }
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }

  if (!appReady) {
    throw new Error(`E2E application did not become ready at ${baseURL}.`);
  }

  const request = await playwrightRequest.newContext({ baseURL });
  try {
    await authenticateRequest(request, {
      username: 'test@example.com',
      password: 'password123',
      organizationId: 'e2e-tenant',
    }, new URL(baseURL).origin);
    if (storageStatePath) await request.storageState({ path: storageStatePath });
  } finally {
    await request.dispose();
  }
}
