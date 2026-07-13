import { test, expect } from '../../../../e2e/fixtures';

test.describe('Regression Audit: Verify Fake Data Removed and Features Rewired', () => {

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

  test('verify affiliate track API fails gracefully when backend is down instead of returning fake data', async ({ page, request }) => {
    // Attempting a direct API hit that previously faked the backend
    const response = await request.post('/api/v1/growth/affiliate/track', {
        data: { link: 'test' }
    });
    // With backend down or missing, it should no longer return 200 { tracked: true }
    // It should now return 500 error gracefully
    expect(response.status()).not.toBe(200);
    const body = await response.json();
    expect(body.error).toBe('Failed to track affiliate link');
  });

  test('verify whatsapp cloud api fails gracefully when backend is down instead of returning fake data', async ({ page, request }) => {
    const response = await request.post('/api/integrations/whatsapp_cloud_api/connect', {
        data: { token: 'test' }
    });
    // It should throw error instead of returning 200 success
    expect(response.status()).not.toBe(200);
  });
});
