import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { test, expect } from '../../fixtures';
import { createGrowthOwner } from '../../growth_owner';
import { e2eDbQuery } from '../../db_utils';

function watchBrowserPaymentWrites(page: Page) {
  const writes: string[] = [];
  page.on('request', request => {
    const path = new URL(request.url()).pathname;
    if (request.method() === 'POST' && (
      path === '/api/v1/terminal/charge'
      || /^\/api\/v1\/(payments\/terminal|pos\/terminal)\/(intent|payment-intent|create_payment_intent|capture|commit|sync|sync_offline)(\/|$)/.test(path)
    )) writes.push(path);
  });
  return writes;
}

test.describe('Legacy POS entry uses the maintained terminal', () => {
  test('redirects to staff authentication without a fabricated catalog or charge', async ({ page }) => {
    const writes = watchBrowserPaymentWrites(page);
    await page.goto('/pos');
    await expect(page).toHaveURL(/\/pos\/terminal$/);
    await expect(page.getByRole('heading', { name: 'Terminal Locked', exact: true })).toBeVisible();
    await expect(page.locator('#pos-keypad')).toBeVisible();
    await expect(page.getByRole('button', { name: /Custom Cake|Consultation Hour|Repair Kit|Charge via Tap-to-Pay/ })).toHaveCount(0);
    await expect(page.getByText(/Payment processed successfully|Offline transactions synced successfully/)).toHaveCount(0);
    expect(writes).toEqual([]);
  });

  test('rejects missing tenant payment credentials without creating a payment operation', async ({ page, baseURL }) => {
    const owner = await createGrowthOwner(page, baseURL);
    const writes = watchBrowserPaymentWrites(page);
    await page.goto('/pos');
    await expect(page).toHaveURL(/\/pos\/terminal$/);
    await expect(page.getByRole('heading', { name: 'Terminal Locked', exact: true })).toBeVisible();
    const headers = { origin: new URL(page.url()).origin, 'sec-fetch-site': 'same-origin' };
    const rejected = { success: false, status: 'rejected', error: 'A verified tenant payment connection is required.' };
    const token = await page.request.post('/api/v1/payments/terminal/token', { headers });
    expect(token.status()).toBe(503);
    expect(token.headers()['cache-control']).toBe('private, no-store');
    expect(await token.json()).toEqual(rejected);
    await token.dispose();

    const operationId = randomUUID();
    const intent = await page.request.post('/api/v1/payments/terminal/intent', {
      headers, data: { idempotency_key: operationId, amount_cents: 7000, currency: 'usd' },
    });
    expect(intent.status()).toBe(503);
    expect(await intent.json()).toEqual(rejected);
    await intent.dispose();
    expect(await e2eDbQuery('SELECT operation_id FROM terminal_payment_operations WHERE tenant_id=$1', [owner.tenantId])).toEqual([]);
    await expect(page.getByRole('button', { name: /Charge \$/ })).toHaveCount(0);
    await expect(page.getByText(/Payment processed successfully|Payment Successful!/)).toHaveCount(0);
    expect(writes).toEqual([]);
  });

  for (const [label, raw] of [
    ['unconfirmed records', '[ { "offline_id": "historical-attempt", "total": 100, "customer": "prior-owner@example.test" }, null ]'],
    ['unreadable records', '[{"offline_id":"truncated-attempt"'],
  ]) {
    test(`holds ${label} across reconnect, reload and the legacy route`, async ({ page, context }) => {
      const writes = watchBrowserPaymentWrites(page);
      await page.goto('/pos/terminal');
      await expect(page.getByRole('heading', { name: 'Terminal Locked', exact: true })).toBeVisible();
      // Seed once. Re-seeding on every navigation would hide accidental deletion.
      await page.evaluate(value => localStorage.setItem('pos_offline_queue', value), raw);
      await page.reload();
      const notice = page.getByRole('status').filter({ hasText: 'Historical POS data needs review' });
      await expect(notice).toBeVisible();
      await expect(notice).toContainText('Payment status and account ownership are unverified');
      await expect(notice).toContainText('Nothing in this legacy queue will be sent or removed');
      await expect(notice).toContainText('account owner compare provider transactions and receipts');
      await expect(notice).not.toContainText(/prior-owner@example.test|historical-attempt|truncated-attempt/);
      await context.setOffline(true);
      await expect(page.getByText('Offline Mode Active', { exact: true })).toBeVisible();
      await context.setOffline(false);
      await expect(page.getByText('Offline Mode Active', { exact: true })).toHaveCount(0);
      expect(await page.evaluate(() => localStorage.getItem('pos_offline_queue'))).toBe(raw);
      await page.goto('/pos');
      await expect(page).toHaveURL(/\/pos\/terminal$/);
      await expect(notice).toBeVisible();
      await page.reload();
      await expect(notice).toBeVisible();
      expect(await page.evaluate(() => localStorage.getItem('pos_offline_queue'))).toBe(raw);
      await expect(page.getByText(/Payment processed successfully|Offline transactions synced successfully/)).toHaveCount(0);
      expect(writes).toEqual([]);
    });
  }
});
