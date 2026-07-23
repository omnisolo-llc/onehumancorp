import { expect, test } from '@playwright/test';

test.use({ storageState: { cookies: [], origins: [] } });

test('the Next.js login form creates a real authenticated session', async ({ context, page }) => {
  await page.goto('/login');

  await page.getByLabel('Email or username').fill('test@example.com');
  await page.getByLabel('Password').fill('password123');
  await page.getByLabel(/Organization/).fill('e2e-tenant');

  await Promise.all([
    page.waitForURL('**/dashboard'),
    page.getByRole('button', { name: 'Log in' }).click(),
  ]);

  const sessionCookie = (await context.cookies()).find((cookie) => cookie.name === 'ohc_session');
  expect(sessionCookie?.httpOnly).toBe(true);
  expect(sessionCookie?.value.length).toBeGreaterThan(0);

  const catalogResponse = await page.request.get('/api/v1/catalog/products');
  expect(catalogResponse.status()).toBe(200);
});
test('dummy', async ({ page }) => {
  // empty
});
});
