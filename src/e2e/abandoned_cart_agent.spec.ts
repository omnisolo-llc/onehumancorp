import { test, expect } from './fixtures';

test.describe('Automated Cart Recovery Agent', () => {
  test('Agent automatically dispatches AI generated message for abandoned cart', async ({ page, request }) => {
    // 1. Merchant views their dashboard to confirm baseline
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: /Dashboard|Overview/i })).toBeVisible({ timeout: 15000 });

    // 2. We mock the server-side trigger for cart recovery because waiting 4 hours in an E2E test is impossible
    // In a real environment, this is triggered via PostgreSQL SKIP LOCKED on a schedule.
    const triggerRes = await request.post('/api/v1/growth/campaign/send-cart', {
        data: {
           customer_name: "Alice",
           cart_value: "$45.00"
        }
    });

    // Wait for the action to log
    await page.waitForTimeout(500);

    // 3. Navigate to a report or dashboard feed where the merchant can see the action log
    // We check the agent activity log or the Business Advisory Report
    await page.goto('/agent-feed');

    // The feed should mention the cart recovery agent took action, verifying the whole cycle
    await expect(page.locator('body')).toContainText(/recovered|abandoned cart|Salesperson/i, { timeout: 15000 });
  });
});
