import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('pricing');

test.describe('Pricing Page', () => {
  test('should display Pricing Plans header', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
  });

  test('should display all 4 pricing tiers', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();
  });

  test('should redirect to checkout when clicking Upgrade to Starter via Stripe', async ({ page }) => {
    await page.goto('/pricing');
    const upgradeButton = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    await expect(upgradeButton).toBeVisible();
    await upgradeButton.click();
    await page.waitForURL('**/checkout?tier=Starter');
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
  });

  test('should navigate back to Dashboard when clicking Back to Dashboard', async ({ page }) => {
    await page.goto('/pricing');
    const backButton = page.getByRole('button', { name: 'Back to Dashboard' });
    await expect(backButton).toBeVisible();
    await backButton.click();
    await page.waitForURL('**/dashboard');
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
