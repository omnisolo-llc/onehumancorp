import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { authenticateRequest } from './authenticate';
import { e2eDbQuery } from './db_utils';
import { E2E_ADMIN_USER } from './identities';

/** A test-owned free account; never resets another test's one-time entitlement. */
export async function createGrowthOwner(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('Playwright baseURL is required for growth account authentication.');
  if (!process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Growth accounts may only be created in the native runner isolated test database.');
  }
  const suffix = randomUUID();
  const tenantId = `e2e-growth-${suffix}`;
  const userId = `e2e-growth-owner-${suffix}`;
  const email = `growth-${suffix}@example.test`;
  const rows = await e2eDbQuery(
    `WITH source_owner AS (
       SELECT password_hash FROM users
       WHERE username = $4 AND tenant_id = $5 AND active = true
     ), new_tenant AS (
       INSERT INTO tenants (id, name, industry, tier, plan_tier, has_claimed_trial_extension)
       SELECT $1, 'Isolated growth test', 'Test', 'Free', 'Free', false FROM source_owner
       RETURNING id
     ), new_owner AS (
       INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, created_at, updated_at)
       SELECT $2, $3, $3, source_owner.password_hash, ARRAY['ADMIN'], true, new_tenant.id,
              CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
       FROM source_owner CROSS JOIN new_tenant
       RETURNING id, tenant_id
     )
     INSERT INTO identity_user_roles (user_id, role_name, tenant_id, position)
     SELECT id, 'ADMIN', tenant_id, 0 FROM new_owner
     RETURNING user_id, tenant_id`,
    [tenantId, userId, email, E2E_ADMIN_USER.email, E2E_ADMIN_USER.organizationId],
  );
  if (rows.length !== 1 || rows[0].user_id !== userId || rows[0].tenant_id !== tenantId) {
    throw new Error('The seeded owner password and normalized role are required for growth test setup.');
  }
  await authenticateRequest(page.request, {
    username: email,
    password: E2E_ADMIN_USER.password,
    organizationId: tenantId,
  }, new URL(baseURL).origin);
  return { tenantId, userId, email };
}
