import { test, expect } from '../../../../e2e/fixtures';

test.describe('AI Usage Paywall Growth Loop', () => {
  test('displays usage data, upgrade CTA, and Powered by OHC viral branding', async ({ page }) => {
    // Navigate to the new AI Usage Paywall page
    await page.goto('/dashboard');
    await page.evaluate(() => {
      window.localStorage.setItem('has_onboarded', 'true');
      window.localStorage.setItem('tenant', 'test-tenant');
    });


    await page.goto('/ai-usage-paywall');

    // Wait for the main heading to appear
    await page.waitForLoadState('networkidle');
    await page.waitForSelector('h1:has-text("AI Agent Usage")', { state: 'visible', timeout: 30000 });
    await expect(page.locator('h1', { hasText: 'AI Agent Usage' })).toBeVisible();

    // Verify the page renders the soft paywall elements
    await expect(page.locator('p', { hasText: 'Monitor your AI automation and unlock unlimited capabilities.' })).toBeVisible();

    // Verify the upgrade CTA button
    const upgradeBtn = page.locator('button', { hasText: 'Upgrade to Pro' });
    await expect(upgradeBtn).toBeVisible();

    // Verify the viral share button
    const shareBtn = page.locator('button', { hasText: 'Share to get 10 free tasks' });
    await expect(shareBtn).toBeVisible();

    // Check the "Powered by OHC" footer loop branding
    const footerLink = page.locator('a:has-text("⚡ Powered by OHC")');
    await expect(footerLink).toBeVisible();
    await expect(footerLink).toHaveAttribute('href', /.*\/api\/v1\/growth\/referrals\/click.*/);

    // Click the Upgrade button and verify it navigates to the pricing page
    await upgradeBtn.click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});
