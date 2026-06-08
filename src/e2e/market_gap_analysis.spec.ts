import { test, expect } from '@playwright/test';

test.describe('Autonomous Inventory, CRM, and Pricing Synchronization', () => {

  test('should present proactive Reorder and Price Adjust action card on low stock / stagnant inventory', async ({ page, request }) => {
    // 1. Simulate the "Red Dress sold out" scenario by injecting the agent approval via API
    const res = await request.post('/api/agents/approvals/simulate-stock-out', {
      headers: {
        'x-spiffe-id': 'spiffe://ohc/org/e2e/agent/browser',
        'x-tenant-id': 'e2e-tenant',
      }
    });

    expect(res.status()).toBe(200);

    // 2. Load the dashboard where the Unified Agent Feed lives
    await page.goto('/dashboard');
    await expect(page.locator('text=Cross-Agent Feed').first()).toBeVisible();

    // 3. Find the action card in the feed
    const cardText = 'Operations Agent drafted a reorder for 50 units. Finance Agent suggests raising price from $40 to $46.';
    const actionCard = page.locator(`text=${cardText}`).first();
    await expect(actionCard).toBeVisible();

    // 4. Click the Approve button for the proposal
    const approveButton = page.locator('button[data-testid="approve-proposal"]').first();

    // We expect the click to trigger the backend API which updates DB and dispatches a job
    const responsePromise = page.waitForResponse(response =>
      response.url().includes('/api/agents/approvals/') && response.status() === 200 && response.request().method() === 'POST'
    );
    await approveButton.click();
    await responsePromise;

    // 5. Assert the UI changes (the specific card shouldn't show in the pending approvals, or should show in the activity log)
    // Switch to Activity Feed tab
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.locator(`text=APPROVED`).first()).toBeVisible();
    await expect(page.locator('text=Red Dress sold out in 2 days. Demand is high. Operations Agent drafted a reorder for 50 units. Finance Agent suggests raising price from $40 to $46.').first()).toBeVisible();

  });
});
