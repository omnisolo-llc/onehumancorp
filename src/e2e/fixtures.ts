import { test as base, expect, type Browser, type BrowserContext, type Page } from '@playwright/test';
import { authenticateRequest } from './authenticate';
import { E2E_SEED_DATA } from '../ui/next/src/lib/e2eSeedData';

export const E2E_ADMIN_USER = {
  email: 'test@example.com',
  password: 'password123',
  role: 'ADMIN',
  organizationId: 'e2e-tenant',
} as const;

export const E2E_UNLIMITED_ADMIN_USER = {
  email: 'pro@example.com',
  password: 'password123',
  role: 'ADMIN',
  organizationId: 'e2e-tenant-unlimited',
} as const;

export const E2E_MEMBER_USER = {
  email: 'member@example.com',
  password: 'MemberPass123!',
  role: 'OPERATOR',
  organizationId: 'e2e-tenant',
} as const;

type E2EUser = typeof E2E_ADMIN_USER | typeof E2E_UNLIMITED_ADMIN_USER | typeof E2E_MEMBER_USER;

async function loginAsAtBaseURL(page: Page, user: E2EUser, baseURL: string) {
  await authenticateRequest(page.request, {
    username: user.email,
    password: user.password,
    organizationId: user.organizationId,
  }, new URL(baseURL).origin);
  await page.goto(new URL('/dashboard', baseURL).toString());
}

function rejectNetworkStubbing(context: BrowserContext, page?: Page) {
  const reject = () => {
    throw new Error('E2E tests must use the real UI and real services. Playwright network substitution is not allowed.');
  };

  (context as unknown as { route: unknown }).route = reject;
  if (page) {
    (page as unknown as { route: unknown }).route = reject;
  }
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
  adminUser: async ({}, use) => {
    await use(E2E_ADMIN_USER);
  },
  unlimitedAdminUser: async ({}, use) => {
    await use(E2E_UNLIMITED_ADMIN_USER);
  },
  memberUser: async ({}, use) => {
    await use(E2E_MEMBER_USER);
  },
  loginAs: async ({ baseURL }, use) => {
    if (!baseURL) throw new Error('Playwright baseURL is required for E2E login.');
    await use((page, user) => loginAsAtBaseURL(page, user, baseURL));
  },
  seedData: async ({}, use) => {
    await use(E2E_SEED_DATA);
  },
  context: async ({ context }, use) => {
    rejectNetworkStubbing(context);
    await use(context);
  },
  page: async ({ page }, use) => {
    rejectNetworkStubbing(page.context(), page);
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
    const page = await context.newPage();
    rejectNetworkStubbing(page.context(), page);
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
    const page = await context.newPage();
    rejectNetworkStubbing(context, page);
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
  if (page.url() === 'about:blank') await page.goto('/login');
  await loginAsAtBaseURL(page, E2E_ADMIN_USER, new URL(page.url()).origin);
  return page;
}
