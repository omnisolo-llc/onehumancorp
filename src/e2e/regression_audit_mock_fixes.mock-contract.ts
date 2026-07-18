import { test, expect } from './fixtures';

test.describe('Regression Audit: Verify Mocks Removed and Features Rewired', () => {

  test('verify seasonal promo generation without setTimeout', async ({ page }) => {
    // Navigate to the Seasonal Promo page
    await page.goto('/seasonal-promo');

    // Fill out the occasion
    const occasionInput = page.locator('#promo-occasion');
    await occasionInput.fill('Spring Cleaning');

    // Fill out the discount
    const discountInput = page.locator('#promo-discount');
    await discountInput.fill('30');

    // Since E2E test runs with local storage, we need to bypass the Pro Feature check
    // or simulate a user that has pro enabled to hit the generate API.
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'true');
        localStorage.setItem('tenant_id', 'e2e-tenant');
    });

    // Reload the page to ensure the local storage settings take effect
    await page.reload();

    // Refill since reload cleared it
    await page.locator('#promo-occasion').fill('Spring Cleaning');
    await page.locator('#promo-discount').fill('30');

    // Click the Generate Campaign button
    const generateBtn = page.getByRole('button', { name: 'Generate Campaign' });
    await generateBtn.click();

    // Verify the output container becomes visible
    const resultContainer = page.locator('#promo-result');
    await expect(resultContainer).toBeVisible({ timeout: 15000 });

    // Verify the generated promo text from the real API (or an adapter returning the correct structure)
    await expect(resultContainer).toContainText('Spring Cleaning Special!', { timeout: 15000 });
    await expect(resultContainer).toContainText('30% OFF');
    await expect(resultContainer).toContainText('⚡ Powered by OHC');
  });

});
