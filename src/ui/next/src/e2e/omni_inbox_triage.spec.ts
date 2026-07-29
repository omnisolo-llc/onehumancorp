import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Triage', () => {
  test('Can view omni inbox', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.locator('text="Inbox"')).toBeVisible();
  });
});
