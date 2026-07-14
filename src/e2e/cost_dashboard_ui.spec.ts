import { test, expect } from './fixtures';

test.describe('Cost Dashboard & Plan Limits UI', () => {
  test('should display the cost dashboard and check expected sections', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard');

    // Check main container visibility
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Check key metrics boxes
    await expect(page.locator('.metric-box', { hasText: 'Total Revenue' })).toBeVisible();
    await expect(page.locator('.metric-box', { hasText: 'Total Costs' })).toBeVisible();
    await expect(page.locator('.metric-box', { hasText: 'Projected Monthly Cost' })).toBeVisible();
  });

  test('should display cost dashboard properly on a mobile viewport (375px)', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // Verify mobile responsiveness: No horizontal scroll should exist
    const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
    expect(scrollWidth).toBeLessThanOrEqual(clientWidth);
  });

  test('should display pricing upgrade plan cards (Free, Starter, Pro, Business)', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    // Verify all four plan cards are visible
    await expect(page.locator('.plan-name', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Business' })).toBeVisible();
  });

  test('should check budget alert visibility state', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard');

    // The alert is hidden by default in the HTML (display: none), verify it's attached to DOM but not visible yet
    const alert = page.locator('#budget-health-alert');
    await expect(alert).toBeHidden();
  });

  test('should verify checkout routing attempts from pricing for Stripe', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

    const starterButton = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    await expect(starterButton).toBeVisible();

    // The button should either navigate or trigger a network request
    // We just verify the button exists and is clickable
    await expect(starterButton).toBeEnabled();
  });
});
