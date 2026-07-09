import { test, expect } from './fixtures';

test.describe('Tauri Billing & Pricing UI', () => {

  test('Cost Dashboard loads and displays "My Plan" and metrics', async ({ page }) => {

    // Tauri static files are hosted at /ui in testing
    await page.goto(`/ui/dashboard.html`);

    // Inject mock token into localStorage before navigating to the billing pages
    await page.evaluate(() => {
      localStorage.setItem('ohc_active_tenant_id', 'e2e-tenant');
      localStorage.setItem('token', 'e2e-dummy-token');
    });

    await page.goto(`/cost-dashboard`);

    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // Verify AI Actions element
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();

    // Verify Storage element
    await expect(page.locator('.stat-title', { hasText: 'Storage Used' })).toBeVisible();

    // Verify Estimated Next Bill
    await expect(page.locator('h2', { hasText: 'Estimated Next Bill:' })).toBeVisible();

    // Verify the presence of specific buttons
    await expect(page.locator('button', { hasText: 'Upgrade' })).toBeVisible();
    await expect(page.locator('button#view-detailed-costs')).toBeVisible();
  });

  test('Pricing page loads and displays tiers', async ({ page }) => {
    await page.goto(`/ui/dashboard.html`);

    await page.evaluate(() => {
      localStorage.setItem('ohc_active_tenant_id', 'e2e-tenant');
      localStorage.setItem('token', 'e2e-dummy-token');
    });

    await page.goto(`/pricing`);

    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Verify the presence of specific pricing tiers
    await expect(page.locator('.plan-name', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Business' })).toBeVisible();

    await expect(page.locator('button#btn-Starter')).toBeVisible();
  });

  test('Pricing page allows downgrade to Free for paid users', async ({ page }) => {
    // Mock the backend response to indicate the user is on the 'Starter' plan
    await page.route('**/api/billing/my-plan', async (route) => {
      const json = {
        current_plan: 'Starter',
        ai_actions_used: 10,
        ai_actions_limit: 1000,
        storage_used_bytes: 5000,
        storage_limit_bytes: 5000000000,
        next_bill_estimated: 2900
      };
      await route.fulfill({ json });
    });

    await page.goto(`/ui/dashboard.html`);

    await page.evaluate(() => {
      localStorage.setItem('ohc_active_tenant_id', 'e2e-tenant');
      localStorage.setItem('token', 'e2e-dummy-token');
    });

    await page.goto(`/pricing`);

    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Verify the Starter plan shows "Manage Plan"
    const starterBtn = page.locator('button#btn-Starter');
    await expect(starterBtn).toBeVisible();
    await expect(starterBtn).toHaveText('Manage Plan');

    // Verify the Free plan shows "Downgrade to Free" and is NOT disabled
    const freeBtn = page.locator('button#btn-Free');
    await expect(freeBtn).toBeVisible();
    await expect(freeBtn).toHaveText('Downgrade to Free');
    await expect(freeBtn).not.toBeDisabled();
  });

  test('Pricing page toggles between monthly and annual pricing', async ({ page }) => {
    await page.goto(`/ui/dashboard.html`);

    await page.evaluate(() => {
      localStorage.setItem('ohc_active_tenant_id', 'e2e-tenant');
      localStorage.setItem('token', 'e2e-dummy-token');
    });

    await page.goto(`/pricing`);

    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Verify initial monthly prices
    const proPrice = page.locator('.ohc-growth-card:has-text("Pro") .plan-price');
    const businessPrice = page.locator('.ohc-growth-card:has-text("Business") .plan-price');

    await expect(proPrice).toContainText('$79');
    await expect(proPrice).toContainText('/month');
    await expect(businessPrice).toContainText('$299');
    await expect(businessPrice).toContainText('/month');

    // Toggle to Annual (Subscribe & Save)
    const toggle = page.locator('label:has(input#billing-toggle)');
    await expect(toggle).toBeVisible();
    await toggle.click();

    // Verify annual prices with 20% discount
    await expect(proPrice).toContainText('$63');
    await expect(proPrice).toContainText('/month, billed annually');
    await expect(businessPrice).toContainText('$239');
    await expect(businessPrice).toContainText('/month, billed annually');

    // Toggle back to Monthly
    await toggle.click();

    // Verify it reverts to original prices
    await expect(proPrice).toContainText('$79');
    await expect(proPrice).toContainText('/month');
    await expect(businessPrice).toContainText('$299');
    await expect(businessPrice).toContainText('/month');
  });
});
