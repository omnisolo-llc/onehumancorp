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

    await page.goto(`/ui/cost-dashboard.html`);

    await expect(page.locator('h1', { hasText: 'My Plan' }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();

    // Verify AI Actions element
    await expect(page.locator('span', { hasText: 'AI actions used this month' })).toBeVisible();

    // Verify Storage element
    await expect(page.locator('.stat-title', { hasText: 'Storage used' })).toBeVisible();

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

    await page.goto(`/ui/pricing.html`);

    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Verify the presence of specific pricing tiers
    await expect(page.locator('.plan-name', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('.plan-name', { hasText: 'Business' })).toBeVisible();

    await expect(page.locator('button#btn-Starter')).toBeVisible();
  });
});
