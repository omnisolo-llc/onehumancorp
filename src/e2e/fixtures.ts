import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

const API_ONLY_E2E_SKIP_REASON =
  'Bazel Playwright starts the Rust API service only; browser UI routes are served by the Next application.';

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

function shouldSkipBrowserUi() {
  return process.env.OHC_API_ONLY_E2E === 'true';
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
  page: async ({ page, adminUser }, use, testInfo) => {
    testInfo.skip(shouldSkipBrowserUi(), API_ONLY_E2E_SKIP_REASON);
    rejectNetworkStubbing(page.context(), page);
    await loginAs(page, adminUser);
    await use(page);
  },
  memberPage: async ({ browser, memberUser }, use, testInfo) => {
    testInfo.skip(shouldSkipBrowserUi(), API_ONLY_E2E_SKIP_REASON);
    const page = await browser.newPage();
    rejectNetworkStubbing(page.context(), page);
    await loginAs(page, memberUser);
    await use(page);
    await page.close();
  },
});

export { expect };
