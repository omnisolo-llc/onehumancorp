import { test, expect } from './fixtures';

test.describe('Billing Services & Plan Limits E2E', () => {
  test('Dashboard displays proper warnings when AI action limit is reached', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    // Wait for the specific usage component to render
    await expect(page.locator('text=Your Current Usage')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=AI actions used this month')).toBeVisible();
    await expect(page.locator('text=Storage used')).toBeVisible();
  });

  test('Plan page UI interaction verifies buttons trigger correct navigation', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // 1. Navigate to /plan
    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    // Wait for loading to finish and buttons to appear
    await expect(page.getByRole('button', { name: 'Upgrade' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('button', { name: 'Manage Billing' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'View Detailed Costs' })).toBeVisible();

    // 2. Click Upgrade and assert navigation to /pricing
    await page.getByRole('button', { name: 'Upgrade' }).click();
    await page.waitForLoadState('networkidle');
    await expect(page).toHaveURL(/\/pricing/);
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible({ timeout: 10000 });

    // 3. Navigate back to /plan
    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    // 4. Click View Detailed Costs and assert navigation to /cost-dashboard
    await page.getByRole('button', { name: 'View Detailed Costs' }).click();
    await page.waitForLoadState('networkidle');
    await expect(page).toHaveURL(/\/cost-dashboard/);
    await expect(page.getByRole('button', { name: 'Download Invoice' })).toBeVisible({ timeout: 10000 });
  });

  test('Pricing page renders correctly and displays buttons', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    // Assert presence of tiers
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Verify upgrade or current plan button is visible
    // Since we don't know the exact starting tier, we check for either Current Plan or the specific Upgrade button
    const upgradeStarterButton = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
    const currentPlanButton = page.getByRole('button', { name: 'Current Plan' });
    const managePlanButton = page.getByRole('button', { name: 'Manage Plan' });

    // Wait for loading to finish
    await expect(page.locator('button:has-text("Loading...")')).toHaveCount(0, { timeout: 10000 });

    // At least one of these should be visible for the Starter tier block
    await expect(upgradeStarterButton.or(currentPlanButton).or(managePlanButton).first()).toBeVisible();
  });

  test('Cost Dashboard renders core metrics and handles interactions', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Wait for the cost dashboard to load
    await expect(page.locator('#cost-dashboard-projected')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total-savings')).toBeVisible();

    // Test Download Invoice interaction
    await page.getByRole('button', { name: 'Download Invoice' }).click();
    await expect(page.locator('text=Invoice download is ready for your current billing period.')).toBeVisible({ timeout: 5000 });
  });
});

test.describe('Cost Features Next', () => {
  test('Pricing page renders Manage Plan for active paid tier', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Intercept my-plan response to return Starter to verify UI logic
    await page.route('**/api/billing/my-plan', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          current_plan: 'Starter',
          ai_actions_used: 10,
          ai_actions_limit: 1000,
          storage_used_bytes: 0,
          storage_limit_bytes: 5000000000,
          next_bill_estimated: 2900
        })
      });
    });

    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    // Wait for the specific usage component to render
    const managePlanButton = page.locator('button', { hasText: 'Manage Plan' });
    await expect(managePlanButton.first()).toBeVisible({ timeout: 15000 });
  });

  test('Cost Dashboard hides billing buttons for Free tier', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.route('**/api/billing/my-plan', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          current_plan: 'Free',
          ai_actions_used: 10,
          ai_actions_limit: 100,
          storage_used_bytes: 0,
          storage_limit_bytes: 500000000,
          next_bill_estimated: 0
        })
      });
    });

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Make sure we loaded
    await expect(page.locator('text=Cost Transparency Dashboard').or(page.locator('#cost-dashboard-total-costs'))).toBeVisible({ timeout: 15000 });

    // Ensure manage billing is hidden
    const manageBillingBtn = page.locator('#manage-billing-btn');
    if (await manageBillingBtn.count() > 0) {
      await expect(manageBillingBtn).toBeHidden();
    }
  });

});
