import { test, expect } from './fixtures';

test.describe('WhatsApp Cloud API Integrations Setting', () => {
  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    await expect(page.locator('h1:has-text("App Integrations")')).toBeVisible({ timeout: 15000 });

    // 3. Find the WhatsApp Cloud API integration card
    const waCloudCard = page.locator('h3:has-text("WhatsApp Cloud API")').locator('..');

    // 4. Verify its presence
    await expect(waCloudCard).toBeVisible({ timeout: 15000 });
  });
});
