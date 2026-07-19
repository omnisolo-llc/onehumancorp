import { test, expect } from '@playwright/test';

test.describe('Referrals Growth Flow', () => {
  test('User can generate and copy referral link on the referrals dashboard', async ({ page }) => {
    // Intercept the API call and provide a mock dynamic referral link
    // Go to referrals page
    await page.goto('/referrals');

    // It should initially show a loading state
    await expect(page.locator('text=Generating your unique link...')).toBeVisible();

    // After loading, it should display the dynamically fetched link
    const referralLink = page.locator('#referral-link');
    await expect(referralLink).toBeVisible();
    await expect(referralLink).toHaveText('https://ohc.app/ref/.*');

    // The copy button should be available
    const copyButton = page.locator('button', { hasText: 'Copy' }).first();
    await expect(copyButton).toBeEnabled();

    // Try clicking copy
    await copyButton.click();
    await expect(page.locator('button', { hasText: 'Copied!' })).toBeVisible();
  });

  test('Fallback to basic tenant link when API fails', async ({ page }) => {
    // Intercept with an error
    await page.route('/api/v1/growth/referrals/generate', async route => {
    // Mock localStorage tenant
    await page.addInitScript(() => {
      window.localStorage.setItem('tenant', 'maya-cakes');
    });

    await page.goto('/referrals');

    const referralLink = page.locator('#referral-link');
    await expect(referralLink).toBeVisible();
    await expect(referralLink).toHaveText('ohc://join?ref=maya-cakes');
  });
});
