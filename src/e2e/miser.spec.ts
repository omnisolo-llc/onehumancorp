import { test, expect } from '@playwright/test';

test.describe('Miser Cost Optimizations', () => {
  test('Pricing Page and Select Plan Checkout Flow', async ({ page }) => {
    // 1. Log in or start at home
    await page.goto('/dashboard');
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('pricing-screen');
    });

    // Verify the "Pricing Plans" title is visible
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible();

    // Verify we can see the Starter, Pro, and Business plans
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();

    // Verify the buttons are present
    const upgradeStarterBtn = page.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeStarterBtn).toBeVisible();

    // Setup network interception to mock the select-plan API response
    await page.route('/api/billing/select-plan', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        const postData = JSON.parse(request.postData() || '{}');
        // Validate payload
        expect(postData.plan_id).toBe('Starter');

        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ url: 'https://checkout.stripe.com/mock-session' })
        });
      } else {
        await route.continue();
      }
    });

    // Intercept the redirect to the mock checkout
    await page.route('https://checkout.stripe.com/mock-session', async route => {
        await route.fulfill({ status: 200, body: 'Mock Stripe Checkout Page' });
    });

    // Click the upgrade button
    await upgradeStarterBtn.click();

    // Wait for the redirect and verify we land on the mock checkout
    await page.waitForURL('https://checkout.stripe.com/mock-session');

    // Check if the mock checkout page loaded
    const bodyText = await page.textContent('body');
    expect(bodyText).toContain('Mock Stripe Checkout Page');
  });

  test('Cost Dashboard Visibility', async ({ page }) => {
    await page.goto('/dashboard');
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('my-plan-screen');
    });

    // Intercept cost dashboard API call
    await page.route('/api/billing/cost-dashboard', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          total_revenue: 15000, // $150.00
          total_costs: 2000,    // $20.00
          llm_cost: 1500,       // $15.00
          storage_cost: 200,    // $2.00
          payment_fees: 300,    // $3.00
          network_cost: 0,
          bandwidth_savings: 0,
          period_start: '2024-05-01',
          period_end: '2024-05-31'
        })
      });
    });

    // Verify Cost Dashboard button
    const viewCostDetailsBtn = page.locator('button', { hasText: 'View Cost Details' });
    await expect(viewCostDetailsBtn).toBeVisible();

    // Click it
    await viewCostDetailsBtn.click();

    // Verify Cost Transparency Dashboard title
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' })).toBeVisible();

    // Wait for the API to render
    await expect(page.locator('#cost-dashboard-total')).toHaveText('$20.00');
    await expect(page.locator('#cost-dashboard-revenue')).toHaveText('$150.00');
    await expect(page.locator('#cost-dashboard-llm')).toHaveText('$15.00');
    await expect(page.locator('#cost-dashboard-storage')).toHaveText('$2.00');
    await expect(page.locator('#cost-dashboard-payment-fees')).toHaveText('$3.00');
    await expect(page.locator('#cost-dashboard-period')).toHaveText('Period: 2024-05-01 to 2024-05-31');
  });
});
