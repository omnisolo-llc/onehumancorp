import { randomUUID } from 'node:crypto';
import { expect, type Page } from '@playwright/test';
import { createGrowthOwner } from '../growth_owner';
import { e2eDbQuery } from '../db_utils';
import { requireLoopbackUrl } from './recorded_invitation';

export async function createRecordedOrderOwner(page: Page, baseURL: string | undefined, count: number) {
  if (!baseURL) throw new Error('An explicit native-runner app URL is required');
  requireLoopbackUrl(baseURL);
  const database = new URL(process.env.DATABASE_URL ?? 'missing:');
  if (!process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-') || database.protocol !== 'postgres:'
    || !['127.0.0.1','[::1]'].includes(database.hostname) || database.pathname !== '/ohc' || !database.port || database.search || database.hash) {
    throw new Error('Recorded-order fixtures require the native runner isolated loopback PostgreSQL container');
  }
  if (!Number.isSafeInteger(count) || count < 0 || count > 1001) throw new Error('Fixture order count is out of bounds');
  const owner = await createGrowthOwner(page, baseURL);
  const identity = await page.request.get(new URL('/api/v1/auth/session-identity', baseURL).href);
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  const foreign = `e2e-milestone-foreign-${randomUUID()}`;
  await e2eDbQuery("INSERT INTO tenants(id,name)VALUES($1,'Foreign milestone fixture')", [foreign]);
  for (const [tenant, amount] of [[owner.tenantId,count],[foreign,100]] as const) {
    await e2eDbQuery(`INSERT INTO orders(id,tenant_id,status,total_amount,base_currency,transaction_currency)
      SELECT $1 || '-order-' || n::text,$1,
        CASE n % 3 WHEN 0 THEN 'fulfilled' WHEN 1 THEN 'pending' ELSE 'cancelled' END,
        n::numeric,CASE n % 2 WHEN 0 THEN 'USD' ELSE 'NZD' END,CASE n % 2 WHEN 0 THEN 'USD' ELSE 'NZD' END
      FROM generate_series(1,$2::int) n`, [tenant,amount]);
  }
  const rows = await e2eDbQuery('SELECT tenant_id,count(*)::int AS recorded_orders FROM orders WHERE tenant_id=ANY($1::text[]) GROUP BY tenant_id', [[owner.tenantId,foreign]]);
  expect(rows.find(row => row.tenant_id === owner.tenantId)?.recorded_orders ?? 0).toBe(count);
  expect(rows.find(row => row.tenant_id === foreign)?.recorded_orders).toBe(100);
  return { ...owner, count, foreign, origin: new URL(baseURL).origin };
}

export function captureRecordedOrders(page: Page, owner: { userId: string; tenantId: string; origin: string; count: number }) {
  return page.waitForResponse(response => new URL(response.url()).origin === owner.origin
    && new URL(response.url()).pathname === '/api/v1/growth/milestone' && response.request().method() === 'GET'
    && response.request().headers()['x-ohc-expected-user'] === owner.userId).then(async response => {
      expect(response.status()).toBe(200);
      expect(response.request().headers()['x-ohc-expected-tenant']).toBe(owner.tenantId);
      const data = await response.json();
      const reached = [1,10,50,100,1000].filter(limit => owner.count >= limit);
      expect(data).toMatchObject({ success: true, metric: 'recorded_orders', included_statuses: 'all_recorded_statuses',
        user_id: owner.userId, tenant_id: owner.tenantId, recorded_orders: owner.count, reached_thresholds: reached,
        highest_threshold: reached.at(-1) ?? null });
      expect(data.error ?? null).toBeNull(); expect(Number.isFinite(Date.parse(data.observed_at))).toBe(true);
      expect(data.reward).toBeUndefined(); expect(data.revenue).toBeUndefined();
      return data;
    });
}
