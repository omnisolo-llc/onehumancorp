import { test as base, expect, type Browser, type BrowserContext, type Page } from '@playwright/test';
import { authenticateRequest } from './authenticate';

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

async function loginAs(page: Page, user: E2EUser) {
  const baseURL = process.env.BASE_URL ?? 'http://127.0.0.1:18789';
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
  memberPage: Page;
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
  loginAs: async ({}, use) => {
    await use(loginAs);
  },
  context: async ({ context }, use) => {
    rejectNetworkStubbing(context);
    await use(context);
  },
  page: async ({ page, adminUser }, use) => {
    rejectNetworkStubbing(page.context(), page);
    await loginAs(page, adminUser);
    await use(page);
  },
  memberPage: async ({ browser, memberUser }, use) => {
    const page = await browser.newPage();
    rejectNetworkStubbing(page.context(), page);
    await loginAs(page, memberUser);
    await use(page);
    await page.close();
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
  await loginAs(page, E2E_ADMIN_USER);
  return page;
}
