import { mkdir } from 'node:fs/promises';
import path from 'node:path';
import { request as playwrightRequest, type FullConfig } from '@playwright/test';

import { authenticateRequest } from '../../../../e2e/authenticate';

export default async function globalSetup(config: FullConfig) {
  const projectUse = config.projects[0]?.use;
  const baseURL = projectUse?.baseURL as string | undefined;
  const storageStatePath = projectUse?.storageState;

  if (!baseURL) {
    throw new Error('Playwright baseURL is required for Next.js E2E setup.');
  }
  if (typeof storageStatePath !== 'string' || storageStatePath.length === 0) {
    throw new Error('Playwright storageState path is required for Next.js E2E setup.');
  }

  await mkdir(path.dirname(storageStatePath), { recursive: true });

  const request = await playwrightRequest.newContext({ baseURL });
  try {
    await authenticateRequest(request, {
      username: 'test@example.com',
      password: 'password123',
      organizationId: 'e2e-tenant',
    }, new URL(baseURL).origin);
    await request.storageState({ path: storageStatePath });
  } finally {
    await request.dispose();
  }
}
