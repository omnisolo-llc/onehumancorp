import { request as playwrightRequest, type FullConfig } from '@playwright/test';
import { authenticateRequest } from './authenticate';
import { chmod } from 'node:fs/promises';
import { E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER, E2E_MEMBER_USER, E2E_STARTER_USER } from './identities';
import { saveAuthenticatedState } from '../../scripts/playwright/session-state.mjs';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for E2E global setup.');
  }

  const storageStatePath = process.env.PLAYWRIGHT_STORAGE_STATE;
  if (!storageStatePath) {
    throw new Error('PLAYWRIGHT_STORAGE_STATE is required for E2E global setup.');
  }

  // The native runner starts an isolated PostgreSQL instance on a random port.
  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) {
    throw new Error('DATABASE_URL is required; E2E tests must use the isolated test PostgreSQL database.');
  }

  // Ensure there are no hardcoded localhost:5432 ports in use
  if (databaseUrl.includes('localhost:5432') && process.env.CI) {
    throw new Error('E2E tests must use the isolated random PostgreSQL port, not localhost:5432.');
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

  const directory = process.env.OMNISOLO_E2E_SESSION_STATE_DIR;
  const actors = directory ? [E2E_ADMIN_USER, E2E_MEMBER_USER, E2E_UNLIMITED_ADMIN_USER, E2E_STARTER_USER] : [E2E_ADMIN_USER];
  for (const actor of actors) {
    const request = await playwrightRequest.newContext({ baseURL, storageState: { cookies: [], origins: [] } });
    try {
      const user = await authenticateRequest(request, {
        username: actor.email, password: actor.password, organizationId: actor.organizationId,
      }, new URL(baseURL).origin);
      const state = await request.storageState();
      if (directory) await saveAuthenticatedState(directory, new URL(baseURL).origin, actor, user, state);
      if (actor === E2E_ADMIN_USER) {
        await request.storageState({ path: storageStatePath });
        await chmod(storageStatePath, 0o600);
      }
    } finally { await request.dispose(); }
  }
}
