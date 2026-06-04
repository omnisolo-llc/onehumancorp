import { test, expect } from './fixtures';

test.describe('Zero-Configuration Subscriptions', () => {
  test('Merchant can view subscriptions dashboard and seeded data', async ({ page }) => {
    await page.goto('/subscriptions');

    // Verify the page title
    await expect(page.locator('h1', { hasText: 'Subscriptions' })).toBeVisible();

    // Verify Active Plans section
    await expect(page.locator('h2', { hasText: 'Active Plans' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Monthly Cake Box' })).toBeVisible();

    // Verify Subscribers section
    await expect(page.locator('h2', { hasText: 'Subscribers (1)' })).toBeVisible();
    await expect(page.locator('text=Customer #e2e-cu')).toBeVisible();

    // Verify Upcoming Fulfillments
    await expect(page.locator('h2', { hasText: 'Upcoming Fulfillments' })).toBeVisible();
    await expect(page.locator('text=1 boxes')).toBeVisible();
  });

  test('Customer checkout creates a subscription intent', async ({ request }) => {
    const payload = {
        plan_id: 'e2e-sub-plan-1',
        payment_method: 'apple_pay'
    };

    // Using unauthenticated request just to verify the stub logic
    const response = await request.post('/api/subscriptions/intent', {
        data: payload
    });

    expect(response.ok()).toBeTruthy();
    const data = await response.json();
    expect(data.intent_id).toBeDefined();
    expect(data.status).toBe('requires_payment_method');
  });
});
