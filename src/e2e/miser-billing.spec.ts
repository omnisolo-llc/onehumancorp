import { test, expect } from './fixtures';

test.describe('Miser Billing Core User Journey', () => {
  test('User can view My Plan, check costs, and see pricing options', async ({ page }) => {
    // Navigate to Plan page
    await page.goto('/plan');

    // Check main elements
    await expect(page.locator('h1').filter({ hasText: 'My Plan' })).toBeVisible();
    await expect(page.locator('text=Current Plan')).toBeVisible();
    await expect(page.locator('text=Estimated Next Bill')).toBeVisible();
    await expect(page.locator('text=Your Current Usage')).toBeVisible();

    // Navigate to Pricing
    await page.click('text=View Upgrade Plans');
    await expect(page.locator('h1').filter({ hasText: 'Pricing Plans' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Business' })).toBeVisible();
  });
});
