import { test, expect } from './fixtures';
import { createEntitlementOwner, expectEntitlementUnchanged } from './support/entitlement_fixture';

test.describe('Pricing Page', () => {
  test('should display Pricing Plans page', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display FAQ section', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h2', { hasText: 'Frequently Asked Questions' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'How do I upgrade, downgrade, or cancel?' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'What is the storage limit?' })).toBeVisible();
  });


  test('should display My Plan section', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h2', { hasText: 'My Plan: Free' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'AI Actions Used' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Storage Used' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Estimated Next Bill' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Manage Plan & Billing' })).toBeVisible();
  });

  test('should display all four pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('should verify Back button functions', async ({ page }) => {
    await page.goto('/pricing');
    const backButton = page.locator('a', { hasText: 'Back to Dashboard' });
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page).toHaveURL(/.*\/dashboard/);
  });

  // The native runner intentionally excludes payment credentials. A browser
  // click cannot manufacture a Stripe session or change the persisted plan.
  for (const tier of ['Starter', 'Pro', 'Business']) {
    test(`keeps the real account unchanged when ${tier} checkout is unconfigured`, async ({ page, baseURL }) => {
      const fixture = await createEntitlementOwner(page, baseURL);
      await page.goto('/pricing');
      const upgrade = page.getByRole('button', { name: `Upgrade to ${tier} via Stripe`, exact: true });
      await expect(upgrade).toBeEnabled();
      for (let attempt = 0; attempt < 2; attempt++) {
        const responsePromise = page.waitForResponse(response =>
          new URL(response.url()).pathname === '/api/v1/billing/create-checkout-session'
          && response.request().method() === 'POST');
        await upgrade.click();
        const response = await responsePromise;
        expect(response.status()).toBe(503);
        expect(response.request().postDataJSON()).toEqual({ tier, is_subscription: true, subscription_interval: 'month' });
        await expect(page.getByRole('alert')).toHaveText('Checkout is unavailable. Your plan has not changed. Please try again.');
        await expect(page).toHaveURL(/\/pricing$/);
        await expect(upgrade).toBeEnabled();
        await expectEntitlementUnchanged(page, fixture);
      }
    });
  }
});
