import { test, expect } from './fixtures';

test.describe('Billing Services & Plan Limits E2E', () => {
  test('Dashboard displays proper warnings when AI action limit is reached', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=Your Current Usage')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=AI actions used this month')).toBeVisible();
    await expect(page.locator('text=Storage used')).toBeVisible();
  });

  test('Cost Dashboard displays proper warnings when AI action limit is reached', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=Cost Transparency Dashboard')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('#budget-health-alert')).toBeVisible();
    await expect(page.locator('#budget-health-alert-text')).toContainText('exceeding your typical baseline');
  });
});
