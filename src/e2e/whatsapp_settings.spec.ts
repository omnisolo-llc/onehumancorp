import { test, expect } from './fixtures';

test.describe('WhatsApp Integration Settings', () => {
  test('User can connect their WhatsApp business account', async ({ page }) => {
    // Navigate to settings page
    await page.goto('/settings');

    // Check if the WhatsApp integration card is present
    await expect(page.locator('text=WhatsApp Business').first()).toBeVisible();

    // Fill in the credentials
    await page.fill('input[placeholder="e.g. 1029384756"]', '1234567890');
    await page.fill('input[placeholder="e.g. 9876543210"]', '0987654321');
    await page.fill('input[placeholder="EAABw..."]', 'EAABwFakeToken1234');

    // Click connect
    await page.click('button:has-text("Connect WhatsApp")');

    // Wait for connected state
    await expect(page.locator('text=Connected').first()).toBeVisible({ timeout: 5000 });

    // Verify saved information is shown
    await expect(page.locator('text=1234567890').first()).toBeVisible();
    await expect(page.locator('text=0987654321').first()).toBeVisible();
  });
});
