import { test as base, expect, type Browser, type BrowserContext, type Page } from '@playwright/test';
import { authenticateRequest } from './authenticate';
import { E2E_SEED_DATA } from '../ui/next/src/lib/e2eSeedData';

import { E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER, E2E_MEMBER_USER, type E2EUser } from './identities';
import { loadAuthenticatedState } from '../../scripts/playwright/session-state.mjs';
export { E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER, E2E_MEMBER_USER, E2E_STARTER_USER } from './identities';

async function loginAsAtBaseURL(page: Page, user: E2EUser, baseURL: string) {
  const origin = new URL(baseURL).origin;
  const directory = process.env.OMNISOLO_E2E_SESSION_STATE_DIR;
  if (directory) {
    // Setup authenticated each actor against the real backend. Restore only a
    // matching, unexpired state; missing states fallback to direct authentication.
    try {
      await page.context().setStorageState(await loadAuthenticatedState(directory, origin, user));
    } catch {
      await authenticateRequest(page.request, {
        username: user.email, password: user.password, organizationId: user.organizationId,
      }, origin);
    }
  } else {
    await authenticateRequest(page.request, {
      username: user.email, password: user.password, organizationId: user.organizationId,
    }, origin);
  }
  await page.goto(new URL('/dashboard', baseURL).toString());
}

export const e2ePage = {
  setupSession: async (page: Page) => {
    const baseURL = (page.context() as unknown as { _options?: { baseURL?: string } })._options?.baseURL || 'http://localhost:3000';
    await loginAsAtBaseURL(page, E2E_ADMIN_USER, baseURL);
  },
};

function rejectNetworkStubbing(context: BrowserContext, page?: Page) {
  const reject = () => {
    throw new Error('E2E tests must use the real UI and real services. Playwright network substitution is not allowed.');
  };

  (context as unknown as { route: unknown }).route = reject;
  if (page) {
    (page as unknown as { route: unknown }).route = reject;
  }
}

export function wrapPage(page: Page): Page {
  const origWaitForLoadState = page.waitForLoadState.bind(page);
  page.waitForLoadState = async (
    state?: 'load' | 'domcontentloaded' | 'networkidle',
    options?: { timeout?: number },
  ) => {
    if (state === 'networkidle') {
      try {
        await origWaitForLoadState('networkidle', { timeout: Math.min(options?.timeout ?? 2000, 2000) });
      } catch {
        await origWaitForLoadState('domcontentloaded', options);
      }
      return;
    }
    return origWaitForLoadState(state, options);
  };
  return page;
}

export const test = base.extend<{
  adminUser: typeof E2E_ADMIN_USER;
  unlimitedAdminUser: typeof E2E_UNLIMITED_ADMIN_USER;
  memberUser: typeof E2E_MEMBER_USER;
  loginAs: (page: Page, user: E2EUser) => Promise<void>;
  anonymousPage: Page;
  memberPage: Page;
  seedData: typeof E2E_SEED_DATA;
}>({
  adminUser: E2E_ADMIN_USER,
  unlimitedAdminUser: E2E_UNLIMITED_ADMIN_USER,
  memberUser: E2E_MEMBER_USER,
  loginAs: async ({ baseURL }, use) => {
    if (!baseURL) throw new Error('Playwright baseURL is required for E2E login.');
    await use((page, user) => loginAsAtBaseURL(page, user, baseURL));
  },
  seedData: E2E_SEED_DATA,
  context: async ({ context }, use) => {
    rejectNetworkStubbing(context);
    context.on('page', (p) => { wrapPage(p); });
    await use(context);
  },
  page: async ({ page }, use) => {
    rejectNetworkStubbing(page.context(), page);
    wrapPage(page);
    await use(page);
  },
  anonymousPage: async ({ browser, baseURL, contextOptions }, use) => {
    if (!baseURL) throw new Error('Playwright baseURL is required for anonymous E2E pages.');
    const context = await browser.newContext({
      ...contextOptions,
      baseURL,
      storageState: { cookies: [], origins: [] },
    });
    rejectNetworkStubbing(context);
    context.on('page', (p) => { wrapPage(p); });
    const page = await context.newPage();
    rejectNetworkStubbing(page.context(), page);
    wrapPage(page);
    await use(page);
    await context.close();
  },
  memberPage: async ({ browser, memberUser, baseURL, contextOptions }, use) => {
    if (!baseURL) throw new Error('Playwright baseURL is required for member E2E pages.');
    const context = await browser.newContext({
      ...contextOptions,
      baseURL,
      storageState: { cookies: [], origins: [] },
    });
    context.on('page', (p) => { wrapPage(p); });
    const page = await context.newPage();
    rejectNetworkStubbing(context, page);
    wrapPage(page);
    await loginAsAtBaseURL(page, memberUser, baseURL);
    await use(page);
    await context.close();
  },
});

export { expect };

export async function adminPage(
  browserOrPage: Browser | Page,
  context?: BrowserContext,
): Promise<Page> {
  let page: Page;
  if ('newPage' in browserOrPage) {
      page = await browserOrPage.newPage();
  } else if ('goto' in browserOrPage) {
      page = browserOrPage;
  } else if (context) {
      page = await context.newPage();
  } else {
      throw new Error('No valid browser or page object provided to adminPage');
  }
  wrapPage(page);
  if (page.url() === 'about:blank') await page.goto('/login');
  await loginAsAtBaseURL(page, E2E_ADMIN_USER, new URL(page.url()).origin);
  return page;
}
