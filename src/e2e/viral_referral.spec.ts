import { test, expect } from './fixtures';

test.describe('Viral Referral Loop on Referrals Page', () => {
  test('should display Referral Dashboard and copy referral link', async ({ page }) => {
    await page.goto('/referrals');

    // Check if the growth component is visible
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
    await expect(page.getByText('Grow Together & Earn Rewards')).toBeVisible();

    // Verify referral link value
    await expect(page.locator('#referral-link')).toContainText('ohc://join?ref=DEFAULT');

    // Test copy button interaction
    await page.getByRole('button', { name: 'Copy', exact: true }).click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
