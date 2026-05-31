import { test, expect } from './fixtures';

test.describe('Viral Storefront E2E', () => {
  test('should display referral block on viral storefronts', async ({ page }) => {
    // Generate the storefront with Referral block
    await page.goto('/storefront-builder');

    // Check if storefront builder exists
    await expect(page.locator('.builder-block').first()).toBeVisible();

    // Check for referral elements
    await expect(page.getByText('Refer a Friend & Earn')).toBeVisible();
    await expect(page.getByText('WhatsApp')).toBeVisible();
    await expect(page.getByText('Share')).toBeVisible();
    await expect(page.getByText('Get Code')).toBeVisible();
  });
});
