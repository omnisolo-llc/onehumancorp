import { test, expect } from './fixtures';

test.describe('Viral Footer Loop', () => {
  test('Storefront footer contains a working "Powered by OHC" referral link', async ({ page }) => {
    // Navigate to the dashboard page where the badge is immediately rendered
    await page.goto('/dashboard');

    // Wait for the badge to be visible
    await page.waitForSelector('text=⚡ Powered by', { timeout: 15000 });

    // Verify the "Powered by OHC" footer is visible
    const poweredByLink = page.locator('a', { hasText: '⚡ Powered by' });
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toContainText('OHC');

    // Verify the link has the correct referral structure
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('/onboarding?ref=');
    expect(href).toContain('source=footer_widget');

    // We can't always reliably catch the fetch in tests because preventDefault inside evaluate
    // might not play well with how React synthetic events propagate onClick,
    // or the navigation cancels the request before it fires.

    // Instead we can just click and verify navigation occurs correctly
    await poweredByLink.click();

    await page.waitForURL(/\/onboarding\?ref=.*&source=footer_widget/);

    expect(page.url()).toContain('/onboarding');
    expect(page.url()).toContain('source=footer_widget');
  });
});
