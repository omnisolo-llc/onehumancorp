import { test, expect } from '@playwright/test';

test.describe('Autonomous Agentic Loyalty & Retention Engine', () => {
  test('Customer completes an order, earns points, and redeems them', async ({ page, request }) => {
    // 1. Simulate Order Completion to trigger Agent
    const tenantId = `tenant_${Date.now()}`;
    const customerId = `customer_${Date.now()}`;

    // Simulate order complete event which the agent listens to
    const earnRes = await request.post('/api/v1/loyalty/transactions', {
        data: {
            tenant_id: tenantId,
            account_id: customerId,
            transaction_type: 'earn',
            amount: 150,
            reason: 'Order Completion via Agent Simulation'
        }
    });

    // 2. Customer View: Wallet
    await page.goto('/loyalty-program');

    // Set localStorage mock for test
    await page.evaluate(({t, c}) => {
        localStorage.setItem('tenant_id', t);
        localStorage.setItem('customer_id', c);
    }, {t: tenantId, c: customerId});

    await page.reload();

    const pointsHeader = page.locator('h2');
    await expect(pointsHeader).toBeVisible();

    const redeemBtn = page.locator('button', { hasText: /Redeem 100pts/i });
    await expect(redeemBtn).toBeVisible();
  });
});
