import { test, expect } from './fixtures';
import { db } from './db_utils';

test.describe('Omni-Channel Payment & Ledger System', () => {
  test('records a pending deposit request without counting uncollected revenue', async ({ page, adminUser }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    const balance = await page.request.get('/api/v1/payments/ledger/balance');
    expect(balance.status()).toBe(200);
    const initialRevenue = (await balance.json()).total_revenue;
    await page.goto('/payments');
    const revenue = page.getByTestId('total-revenue');
    await expect(revenue).toHaveText(`$${Number(initialRevenue).toFixed(2)}`);
    await page.getByTestId('payment-amount-input').fill('50');

    const created = page.waitForResponse(response => response.url().endsWith('/api/v1/payments/ledger/intent') && response.request().method() === 'POST');
    await page.getByTestId('request-payment-button').click();
    const response = await created;
    expect(response.status()).toBe(201);
    const intent = await response.json();
    expect(intent.status).toBe('pending');
    expect(intent.payment_id).toBeTruthy();
    const records = await db.query('SELECT status, amount, currency FROM payment_intents WHERE payment_id = $1 AND tenant_id = $2', [intent.payment_id, adminUser.organizationId]);
    expect(records).toHaveLength(1);
    expect(records[0].status).toBe('pending');
    expect(Number(records[0].amount)).toBe(50);
    expect(records[0].currency).toBe('USD');
    await expect(page.getByTestId('payment-status')).toHaveText('Awaiting payment confirmation');
    await expect(revenue).toHaveText(`$${Number(initialRevenue).toFixed(2)}`);
    expect((await (await page.request.get('/api/v1/payments/ledger/balance')).json()).total_revenue).toBe(initialRevenue);
    await expect(page.getByText('Approved', { exact: true })).toHaveCount(0);
  });
});
