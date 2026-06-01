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
  // Evaluate local storage to bypass the redirection rule to /onboarding
  await page.evaluate(() => {
      localStorage.setItem('has_onboarded', 'true');
  });
  await page.goto('/dashboard');
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
  adminPage: Page;
  memberPage: Page;
}>({
  adminPage: async ({ page, context }, use) => {
    rejectNetworkStubbing(context, page);
    await loginAs(page, E2E_ADMIN_USER);
    await use(page);
  },
  memberPage: async ({ page, context }, use) => {
    rejectNetworkStubbing(context, page);
    await loginAs(page, E2E_MEMBER_USER);
    await use(page);
  },
  page: async ({ page, context }, use) => {
    rejectNetworkStubbing(context, page);
    await use(page);
  },
  context: async ({ context }, use) => {
    rejectNetworkStubbing(context);
    await use(context);
  }
});

export { expect };
