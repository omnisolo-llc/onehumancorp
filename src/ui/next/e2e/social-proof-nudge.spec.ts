import { test, expect } from '@playwright/test';

test.describe('Social Proof Nudge Generator', () => {
  test('should allow merchant to configure social proof widget', async ({ page }) => {
    // 1. Merchant logs in and navigates to the Social Proof Nudge feature
    await page.goto('/social-proof-nudge');

    // 2. Ensure page loaded with title
    await expect(page.locator('h1')).toContainText('Social Proof Nudge');

    // 3. Enter product and location details
    await page.fill('input[placeholder="e.g. Signature Coffee Blend"]', 'Artisan Bread');
    await page.fill('input[placeholder="e.g. Someone in London"]', 'John in Seattle');

    // 4. Verify preview updates
    await expect(page.getByText('Artisan Bread', { exact: true })).toBeVisible();
    await expect(page.locator('p').filter({ hasText: 'John in Seattle' })).toBeVisible();

    // 5. Change Time Display
    await page.selectOption('select', '1 hour ago');
    await expect(page.locator('p').filter({ hasText: '1 hour ago' })).toBeVisible();

    // 6. Test soft paywall
    const removeBrandingCheckbox = page.locator('label', { hasText: /Remove "Powered by OHC" Badge/i });
    await removeBrandingCheckbox.click();

    // Paywall modal should appear
    await expect(page.locator('text=Upgrade to Remove Branding')).toBeVisible();

    // Close modal
    await page.click('button:has-text("×")');

    // 7. Verify embed code is generated with parameters
    const embedCode = await page.locator('#embed-code').textContent();
    expect(embedCode).toContain('data-product="Artisan Bread"');
    expect(embedCode).toContain('data-location="John in Seattle"');
    expect(embedCode).toContain('data-time="1 hour ago"');
  });
});
