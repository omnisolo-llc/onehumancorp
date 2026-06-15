import { test, expect } from './fixtures';

test.describe('Automated Cart Recovery Agent', () => {
  test('Agent automatically dispatches AI generated message for abandoned cart', async ({ page, request }) => {
    // 1. Merchant views their dashboard to confirm baseline
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // 2. We trigger the server-side action for cart recovery because waiting 4 hours in an E2E test is impossible
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
    await page.goto('/dashboard');

    // The feed should mention the cart recovery agent took action, verifying the whole cycle
    await expect(page.locator('body')).toContainText(/recovered|abandoned cart|Salesperson/i, { timeout: 15000 });
  });

  test('Shows zero-click AI recovery draft card on the agent feed and allows 1-Tap Approval', async ({ memberPage: page, request }) => {
    // 1. Manually add an abandoned cart session representing a customer dropping off
    await request.post('/api/v1/growth/campaign/generate-cart', {
        data: { session_id: '12345678-1234-1234-1234-1234567890ab', customer_name: 'Test Customer', cart_value: '$50.00' }
    });

    // 2. Trigger the job queue (simulating the cron that detects abandoned carts)
    await request.post('/api/v1/growth/campaign/send-cart', {
         data: { customer_name: 'Test Customer', cart_value: "$50.00", checkout_session_id: "12345678-1234-1234-1234-1234567890ab" }
    });

    // 3. Verify it shows up in the Agent Feed on mobile view
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/feed');
    await expect(page.locator('body')).toContainText(/recovered 1 abandoned cart/i, { timeout: 15000 });

    // 4. Click the 1-Tap Approve button
    const approveButton = page.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // 5. Card should disappear and be marked as processed
    await expect(page.locator('body')).not.toContainText('Approve', { timeout: 15000 });
  });
});
