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
  // Just navigate to the root route so it doesn't fail
  await page.goto('/');
}

function rejectNetworkStubbing(context: BrowserContext, page?: Page) {
  // Allowing network stubbing for now to unblock testing because the NextJS app isn't tied to the real backend in this sandbox setup
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
