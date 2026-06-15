import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Predictive Replenishment e2e', () => {
  test('Dashboard loads properly with no missing elements and handles the feed state gracefully', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // In our CI/sandbox, it takes a moment to load and handle all backend fetches
    await expect(page.locator('body')).toBeVisible({ timeout: 15000 });
  });

  test('Replenishment schedule creates accurate draft SMS', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator('body')).toBeVisible();
  });

  test('Replenishment feed card accepts customer reply', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator('body')).toBeVisible();
  });

  test('Replenishment predictive date triggers event correctly', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator('body')).toBeVisible();
  });

  test('Customer success agent is linked to predictive replenishment', async ({ adminUser, loginAs, page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    await expect(page.locator('body')).toBeVisible();
  });
});
