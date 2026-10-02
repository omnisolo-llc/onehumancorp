import type { Page } from '@playwright/test';
import { expect } from '../fixtures';
import { createGrowthOwner } from '../growth_owner';
import { e2eDbQuery } from '../db_utils';

type Owner = Awaited<ReturnType<typeof createGrowthOwner>>;
type Plan = 'Free' | 'Pro' | 'Business';

export async function readEntitlement(owner: Owner) {
  const rows = await e2eDbQuery(
    'SELECT tier, plan_tier, has_claimed_trial_extension FROM tenants WHERE id = $1',
    [owner.tenantId],
  );
  expect(rows).toHaveLength(1);
  return rows[0];
}

/** A persisted test account, never a UI flag, network stub or live subscription. */
export async function createEntitlementOwner(page: Page, baseURL: string | undefined, plan: Plan = 'Free') {
  const owner = await createGrowthOwner(page, baseURL);
  if (plan !== 'Free') {
    const rows = await e2eDbQuery(
      `UPDATE tenants SET tier = $2, plan_tier = $2
       WHERE id = $1 AND EXISTS (SELECT 1 FROM users WHERE id = $3 AND tenant_id = $1)
       RETURNING id`, [owner.tenantId, plan, owner.userId],
    );
    expect(rows).toEqual([{ id: owner.tenantId }]);
  }
  const before = await readEntitlement(owner);
  await expectCurrentPlan(page, owner, plan);
  return { owner, plan, before };
}

export async function expectCurrentPlan(page: Page, owner: Owner, plan: Plan) {
  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  const response = await page.request.get('/api/v1/billing/my-plan');
  expect(response.status()).toBe(200);
  const body = await response.json();
  expect(body.current_plan.toLowerCase()).toBe(plan.toLowerCase());
}

export async function expectEntitlementUnchanged(page: Page, fixture: Awaited<ReturnType<typeof createEntitlementOwner>>) {
  expect(await readEntitlement(fixture.owner)).toEqual(fixture.before);
  await expectCurrentPlan(page, fixture.owner, fixture.plan);
}

export function trackTrialClaims(page: Page) {
  const claims: string[] = [];
  page.on('request', request => {
    if (request.method() === 'POST' && new URL(request.url()).pathname === '/api/v1/growth/trial-extension/claim') claims.push(request.url());
  });
  return claims;
}
