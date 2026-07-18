import { test, expect } from './fixtures';

test.describe('Lead Magnet Generator Growth Loop', () => {
  test('Merchant uses Lead Magnet Generator and sees soft paywall', async ({ page, request, loginAs, adminUser }) => {
    // Navigate and login
    await loginAs(page, adminUser);

    // Navigate to dashboard
    await page.goto('/dashboard');

    // Ensure pro is false
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'false');
    });

    // Find the link to lead magnet generator
    const leadMagnetButton = page.locator('a[href="/lead-magnet-generator"]');
    await expect(leadMagnetButton).toBeVisible();
    await leadMagnetButton.click();

    // 1. Merchant navigates to lead magnet generator page
    await page.waitForURL('**/lead-magnet-generator');

    // Check baseline: the page should be loaded
    const titleHeader = page.locator('h1', { hasText: 'Lead Magnet Generator' });
    await expect(titleHeader).toBeVisible();

    // Configure Widget
    await page.fill('input[type="text"] >> nth=0', 'Get 10% Off');
    await page.fill('textarea', 'Sign up for our newsletter to get 10% off your first order.');
    await page.fill('input[type="text"] >> nth=1', 'Subscribe Now');

    // Check preview updates
    await expect(page.locator('h3', { hasText: 'Get 10% Off' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Sign up for our newsletter to get 10% off your first order.' })).toBeVisible();

    // 2. Merchant tries to remove branding without Pro
    const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
    await removeBrandingCheckbox.check();

    // 3. Soft paywall appears
    const upgradeHeader = page.locator('h3', { hasText: 'Upgrade to OHC Pro' });
    await expect(upgradeHeader).toBeVisible();

    // 4. Click Keep Branding
    const keepBrandingBtn = page.locator('button', { hasText: 'Keep Branding' });
    await keepBrandingBtn.click();

    // 5. Verify the soft paywall closes
    await expect(upgradeHeader).not.toBeVisible();
  });
});
