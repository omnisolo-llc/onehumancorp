import { test, expect } from './fixtures';

test.describe('Footer Branding Growth Loop', () => {
  test('booking.html should contain dynamic powered-by link with tenant context', async ({ page }) => {
    // Navigate to booking widget with a specific tenant
    await page.goto('/booking.html?tenant=e2e-tenant');
    await page.waitForLoadState('networkidle');

    // Find the Powered by OHC link
    const brandingLink = page.locator('#branding-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toHaveText('⚡ Powered by OHC');

    // Verify it resolves to the referral tracking URL with the correct tenant and source
    const href = await brandingLink.getAttribute('href');
    expect(href).toBe('/api/v1/growth/referrals/click?target=/onboarding&ref=e2e-tenant&source=booking_widget');
  });

  test('instant-quote.html should contain dynamic powered-by link with tenant context', async ({ page }) => {
    // Navigate to instant-quote widget with a specific tenant
    await page.goto('/instant-quote.html?tenant=e2e-tenant');
    await page.waitForLoadState('networkidle');

    // Find the Powered by OHC link
    const brandingLink = page.locator('#powered-by-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toHaveText('⚡ Powered by OHC');

    // Verify it resolves to the referral tracking URL with the correct tenant and source
    const href = await brandingLink.getAttribute('href');
    expect(href).toBe('/api/v1/growth/referrals/click?target=/onboarding&ref=e2e-tenant&source=instant_quote_widget');
  });
});
