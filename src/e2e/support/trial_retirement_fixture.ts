import type { Page } from '@playwright/test';
import { expect, E2E_ADMIN_USER, E2E_UNLIMITED_ADMIN_USER } from '../fixtures';
import type { E2EUser } from '../identities';
import { e2eDbQuery } from '../db_utils';
import { expectCurrentPlan, readEntitlement } from './entitlement_fixture';

/** Read-only trial coverage reuses actors genuinely authenticated by global setup. */
export async function useTrialSeedOwner(
  page: Page,
  loginAs: (page: Page, user: E2EUser) => Promise<void>,
  plan: 'Free' | 'Pro' = 'Free',
) {
  const actor = plan === 'Free' ? E2E_ADMIN_USER : E2E_UNLIMITED_ADMIN_USER;
  const rows = await e2eDbQuery(
    'SELECT id FROM users WHERE username = $1 AND tenant_id = $2 AND active = true',
    [actor.email, actor.organizationId],
  );
  expect(rows).toHaveLength(1);
  const owner = { userId: rows[0].id as string, tenantId: actor.organizationId, email: actor.email };
  const before = await readEntitlement(owner);
  expect(before).toMatchObject({ tier: plan, plan_tier: plan });
  await loginAs(page, actor);
  await expectCurrentPlan(page, owner, plan);
  return { owner, plan, before };
}

/** Keep duplicate query keys and their order while treating '+' and '%20' alike. */
export function trialUrlParts(url: URL) {
  return { origin: url.origin, pathname: url.pathname, query: [...url.searchParams.entries()], hash: url.hash };
}
