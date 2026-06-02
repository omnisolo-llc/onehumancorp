import { test, expect } from './fixtures';

test.describe('CUJ: Pricing and Plan Upgrade', () => {
  test('Owner can navigate to pricing page and view plan details', async ({ page }) => {
    // Navigate starting from home page
    await page.goto('/');

    // In our test environment, we expect a 'Go to Dashboard' link or button to appear
    const dashLink = page.getByRole('link', { name: 'Go to Dashboard' });
    if (await dashLink.isVisible()) {
      await dashLink.click();
    } else {
      const loginBtn = page.getByRole('button', { name: 'Log In' });
      if (await loginBtn.isVisible()) {
        await loginBtn.click();
      }
    }

    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // From dashboard, go to billing first
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // Now from billing, navigate to pricing
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();

    // Verify pricing screen is visible
    await expect(page.locator('#pricing-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    // Verify all tiers are present
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Verify upgrade buttons exist and work
    const starterUpgradeBtn = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    await expect(starterUpgradeBtn).toBeVisible();

    // Click upgrade to starter and verify it navigates to checkout
    await starterUpgradeBtn.click();

    // Verify checkout page
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
  });
});
