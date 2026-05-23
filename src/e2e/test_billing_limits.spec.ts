import { test, expect } from './fixtures';

test.describe('Billing & Rate Limits', () => {
  test('should display dashboard', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should display agents page', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should show usage limit warning but not block access', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Check soft limits
    // The warning should appear when usage meets or exceeds the limit
    await page.route('**/api/billing/my-plan', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          current_plan: "Free",
          ai_actions_used: 150,
          ai_actions_limit: 100,
          storage_used_bytes: 600 * 1024 * 1024,
          storage_limit_bytes: 500 * 1024 * 1024,
          next_bill_estimated: 0,
        })
      });
    });

    // Reload the page to apply the mocked route
    await page.reload();

    await expect(page.getByText('You\'ve reached your free action limit.')).toBeVisible();
    await expect(page.getByText('Storage is getting full!')).toBeVisible();
  });

  test('should display cost dashboard metrics', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Cost & AI Usage' })).toBeVisible();

    await page.route('**/api/billing/cost-dashboard', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          total_revenue: 100000,
          total_costs: 20000,
          llm_cost: 15000,
          storage_cost: 2000,
          payment_fees: 3000,
          period_start: "2024-05-01",
          period_end: "2024-05-31",
        })
      });
    });

    await page.reload();

    // Expect the total costs string ($200.00)
    await expect(page.getByText('$200.00')).toBeVisible();
    await expect(page.getByText('$150.00')).toBeVisible(); // LLM Cost
    await expect(page.getByText('$20.00')).toBeVisible(); // Storage Cost
  });
});

test.describe('Navigation', () => {
  test('should navigate via nav links', async ({ page }) => {
    await page.goto('/');
    await page.locator('nav a:has-text("Agents")').click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('should display login page', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });
});
