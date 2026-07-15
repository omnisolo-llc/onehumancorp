# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: website-builder-bugfix.spec.ts >> Website Builder Tool (E2E Validation) >> can enter business type and advance
- Location: src/e2e/website-builder-bugfix.spec.ts:10:9

# Error details

```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://127.0.0.1:18789/dashboard
Call log:
  - navigating to "http://127.0.0.1:18789/dashboard", waiting until "load"

```

# Test source

```ts
  1  | import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';
  2  |
  3  | export const E2E_ADMIN_USER = {
  4  |   email: 'test@example.com',
  5  |   password: 'password123',
  6  |   role: 'ADMIN',
  7  | } as const;
  8  |
  9  | export const E2E_UNLIMITED_ADMIN_USER = {
  10 |   email: 'pro@example.com',
  11 |   password: 'password123',
  12 |   role: 'ADMIN',
  13 | } as const;
  14 |
  15 | export const E2E_MEMBER_USER = {
  16 |   email: 'member@example.com',
  17 |   password: 'MemberPass123!',
  18 |   role: 'OPERATOR',
  19 | } as const;
  20 |
  21 | type E2EUser = typeof E2E_ADMIN_USER | typeof E2E_UNLIMITED_ADMIN_USER | typeof E2E_MEMBER_USER;
  22 |
  23 | async function loginAs(page: Page, user: E2EUser) {
  24 |   // We need to inject the tenant ID context for the mock app if possible.
  25 |   // The actual tenant_id comes from a header or cookie in a real deployment.
  26 |   // In the real system, it's determined by the login session. But in our e2e fixture,
  27 |   // we can use Playwright to set the context or navigate.
> 28 |   await page.goto(process.env.BASE_URL ? `${process.env.BASE_URL}/dashboard` : 'http://127.0.0.1:18789/dashboard');
     |              ^ Error: page.goto: net::ERR_CONNECTION_REFUSED at http://127.0.0.1:18789/dashboard
  29 | }
  30 |
  31 | function rejectNetworkStubbing(context: BrowserContext, page?: Page) {
  32 |   const reject = () => {
  33 |     throw new Error('E2E tests must use the real UI and real services. Playwright network substitution is not allowed.');
  34 |   };
  35 |
  36 |   (context as unknown as { route: unknown }).route = reject;
  37 |   if (page) {
  38 |     (page as unknown as { route: unknown }).route = reject;
  39 |   }
  40 | }
  41 |
  42 | export const test = base.extend<{
  43 |   adminUser: typeof E2E_ADMIN_USER;
  44 |   unlimitedAdminUser: typeof E2E_UNLIMITED_ADMIN_USER;
  45 |   memberUser: typeof E2E_MEMBER_USER;
  46 |   loginAs: (page: Page, user: E2EUser) => Promise<void>;
  47 |   memberPage: Page;
  48 | }>({
  49 |   adminUser: async ({}, use) => {
  50 |     await use(E2E_ADMIN_USER);
  51 |   },
  52 |   unlimitedAdminUser: async ({}, use) => {
  53 |     await use(E2E_UNLIMITED_ADMIN_USER);
  54 |   },
  55 |   memberUser: async ({}, use) => {
  56 |     await use(E2E_MEMBER_USER);
  57 |   },
  58 |   loginAs: async ({}, use) => {
  59 |     await use(loginAs);
  60 |   },
  61 |   context: async ({ context }, use) => {
  62 |     rejectNetworkStubbing(context);
  63 |     await use(context);
  64 |   },
  65 |   page: async ({ page, adminUser }, use) => {
  66 |     rejectNetworkStubbing(page.context(), page);
  67 |     await loginAs(page, adminUser);
  68 |     await use(page);
  69 |   },
  70 |   memberPage: async ({ browser, memberUser }, use) => {
  71 |     const page = await browser.newPage();
  72 |     rejectNetworkStubbing(page.context(), page);
  73 |     await loginAs(page, memberUser);
  74 |     await use(page);
  75 |     await page.close();
  76 |   },
  77 | });
  78 |
  79 | export { expect };
  80 |
  81 | export async function adminPage(browserOrPage: any, context?: any) {
  82 |   let page;
  83 |   if (browserOrPage && browserOrPage.newPage) {
  84 |       page = await browserOrPage.newPage();
  85 |   } else if (browserOrPage && browserOrPage.goto) {
  86 |       page = browserOrPage;
  87 |   } else if (context && context.newPage) {
  88 |       page = await context.newPage();
  89 |   } else {
  90 |       throw new Error('No valid browser or page object provided to adminPage');
  91 |   }
  92 |   await loginAs(page, E2E_ADMIN_USER);
  93 |   return page;
  94 | }
  95 |
```