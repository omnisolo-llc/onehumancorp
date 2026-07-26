import { test, expect } from '@playwright/test';

test.describe('WhatsApp Cloud API', () => {
  test('Loads settings', async ({ page }) => {
    await page.goto('/settings/whatsapp');
    await expect(page.locator('body')).toBeVisible();
  });
});
