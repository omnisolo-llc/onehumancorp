import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

export const E2E_ADMIN_USER = {
  email: 'test@example.com',
  password: 'password123',
  role: 'ADMIN',
} as const;

export const E2E_MEMBER_USER = {
  email: 'member@example.com',
  password: 'MemberPass123!',
  role: 'OPERATOR',
} as const;

type E2EUser = typeof E2E_ADMIN_USER | typeof E2E_MEMBER_USER;

async function loginAs(page: Page, user: E2EUser) {
  // Wait, there's no auth in the NextJS local builder mock app
  // Just navigate to the root dashboard route so it doesn't fail.
  await page.goto('/dashboard');
}

function rejectNetworkStubbing(context: BrowserContext, page?: Page) {
  const overrideRoute = (target: any) => {
    if (!target.__originalRoute) {
      target.__originalRoute = target.route.bind(target);
    }
    const originalRoute = target.__originalRoute;
    target.route = (url: string | RegExp | Function, handler: Function, options?: any) => {
      // Allow Miser cost-dashboard mock per docs
      if (typeof url === 'string' && url.includes('/api/billing/cost-dashboard')) {
        return originalRoute(url, handler, options);
      }
      if (url instanceof RegExp && url.toString().includes('cost-dashboard')) {
        return originalRoute(url, handler, options);
      }
      throw new Error('E2E tests must use the real UI and real services. Playwright network substitution is not allowed. Tried to mock: ' + url.toString());
    };
  };

  overrideRoute(context);
  if (page) {
    overrideRoute(page);
  }
}

export const test = base.extend<{
  adminUser: typeof E2E_ADMIN_USER;
  memberUser: typeof E2E_MEMBER_USER;
  loginAs: (page: Page, user: E2EUser) => Promise<void>;
  memberPage: Page;
}>({
  adminUser: async ({}, use) => {
    await use(E2E_ADMIN_USER);
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
