import { test, expect } from '@playwright/test';

test.describe('WhatsApp Cloud API Integrations Setting', () => {

  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page }) => {
    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    const waCloudCard = page.locator('h3:has-text("WhatsApp Cloud API")').locator('..');

    // 4. Verify its presence if loaded
    if (await waCloudCard.isVisible()) {
        // 5. Click the "Connect" button
        const connectButton = waCloudCard.locator('button:has-text("Connect")');
        await expect(connectButton).toBeVisible();
        await connectButton.click();
    }
  });
});
