import { test, expect } from '@playwright/test';

test.describe('Tool Integrations', () => {
  test('can connect Meta and Cal integrations', async ({ page }) => {
    // Navigate to integrations page
    await page.goto('/dashboard/integrations');

    // Connect Meta
    await page.click('#connect-meta');
    await expect(page.locator('#connect-meta')).toHaveText('Connected');

    // Connect Cal.com
    await page.click('#connect-cal');
    await expect(page.locator('#connect-cal')).toHaveText('Connected');
  });
});
