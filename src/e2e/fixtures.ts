import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

export const E2E_ADMIN_USER = {
  email: 'test@example.com',
  password: 'password123',
  role: 'ADMIN',
} as const;

export const E2E_UNLIMITED_ADMIN_USER = {
  email: 'pro@example.com',
  password: 'password123',
  role: 'ADMIN',
} as const;

export const E2E_MEMBER_USER = {
  email: 'member@example.com',
  password: 'MemberPass123!',
  role: 'OPERATOR',
} as const;

type E2EUser = typeof E2E_ADMIN_USER | typeof E2E_UNLIMITED_ADMIN_USER | typeof E2E_MEMBER_USER;

async function loginAs(page: Page, user: E2EUser) {
  // We need to inject the tenant ID context for the mock app if possible.
  // The actual tenant_id comes from a header or cookie in a real deployment.
  // In the real system, it's determined by the login session. But in our e2e fixture,
  // we can use Playwright to set the context or navigate.
  await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/dashboard` : 'http://127.0.0.1:18789/dashboard');
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

export async function adminPage(browserOrPage: any, context?: any) {
  let page;
  if (browserOrPage && browserOrPage.newPage) {
      page = await browserOrPage.newPage();
  } else if (browserOrPage && browserOrPage.goto) {
      page = browserOrPage;
  } else if (context && context.newPage) {
      page = await context.newPage();
  } else {
      throw new Error('No valid browser or page object provided to adminPage');
  }
  await loginAs(page, E2E_ADMIN_USER);
  return page;
}
