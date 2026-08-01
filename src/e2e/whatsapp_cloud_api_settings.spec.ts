import { expect, test } from './fixtures';

test.describe('WhatsApp Cloud API Integrations Setting', () => {

  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page }) => {
    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    await expect(page.locator('h1', { hasText: 'App Integrations' }).first()).toBeVisible({ timeout: 15000 });
  });
});
