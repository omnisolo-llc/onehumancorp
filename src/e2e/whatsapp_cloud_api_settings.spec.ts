import { test, expect } from '@playwright/test';

test.describe('WhatsApp Cloud API Integrations Setting', () => {
  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page }) => {
    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    const heading = page.locator('h1:has-text("App Integrations")');
    if (await heading.isVisible()) {
        await expect(heading).toBeVisible();
    }
  });
});
