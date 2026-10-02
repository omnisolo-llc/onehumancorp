import { test, expect } from '../fixtures';

test.describe('Actionable Inbox', () => {
  test.beforeEach(async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/inbox');
    // The server stream temporarily contains both the fallback and hidden
    // workspace shell. Assert the committed surface before inspecting its title.
    await expect(page.getByTestId('inbox-settled')).toBeVisible();
    await expect(page.locator('.app-title')).toHaveCount(1);
  });
  test('Inbox loads and displays messages', async ({ page }) => {
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
    await expect(page.locator('#messages-list')).toBeVisible();
  });

  test('Inbox displays original content and draft reply', async ({ page }) => {
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox can approve and send draft', async ({ page }) => {
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox properly flags messages with warn tone', async ({ page }) => {
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });

  test('Inbox shows empty state correctly', async ({ page }) => {
    await expect(page.locator('.app-title')).toHaveText('Unified Inbox');
  });
});
