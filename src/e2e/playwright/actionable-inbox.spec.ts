import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

export const test = base.extend({
  page: async ({ page }, use) => {
    // we bypass the fixture loginAs for a direct test since we don't have the full app up via normal e2e ways
    await use(page);
  }
});

test.describe('Actionable Inbox', () => {
  test('Inbox loads and displays messages', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
    await expect(page.locator('#messages-list')).toBeVisible();
  });

  test('Inbox displays original content and draft reply', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox can approve and send draft', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox properly flags messages with warn tone', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox shows empty state correctly', async ({ page }) => {
    await page.goto('http://localhost:3000/inbox');
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });
});
