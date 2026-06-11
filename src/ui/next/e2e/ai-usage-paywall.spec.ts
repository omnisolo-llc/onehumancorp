import { test, expect } from '@playwright/test';

test.describe('AI Usage Paywall Growth Loop', () => {
  test('displays usage data, upgrade CTA, and Powered by OHC viral branding', async ({ page }) => {
    // Navigate to the new AI Usage Paywall page
    await page.goto('/ai-usage-paywall');

    // Wait for the data to load (simulated or real depending on E2E environment)
    // The page shows "Loading AI Usage..." initially, so we wait for the main heading to appear
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
    const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(footerLink).toBeVisible();
    await expect(footerLink).toHaveAttribute('href', 'https://ohc.store');

    // Click the Upgrade button and verify it navigates to the pricing page
    await upgradeBtn.click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});
