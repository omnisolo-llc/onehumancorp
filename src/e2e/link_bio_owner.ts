import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { authenticateRequest } from './authenticate';
import { e2eDbQuery } from './db_utils';
import type { E2EUser } from './identities';

/** Separate configuration per test while preserving its seeded role and plan. */
export async function createLinkBioActor(page: Page, baseURL: string | undefined, source: E2EUser) {
  if (!baseURL) throw new Error('An explicit local app baseURL is required.');
  const origin = new URL(baseURL);
  if (!['http:', 'https:'].includes(origin.protocol) || !['localhost', '127.0.0.1', '[::1]'].includes(origin.hostname)
      || !process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
    throw new Error('Link-bio fixtures require the isolated native runner app and database.');
  }
  const suffix = randomUUID();
  const tenantId = `e2e-bio-${suffix}`, userId = `e2e-bio-user-${suffix}`, email = `bio-${suffix}@example.test`;
  const rows = await e2eDbQuery(`WITH source_actor AS (
      SELECT u.password_hash,u.roles,t.industry,t.tier,t.plan_tier,t.has_claimed_trial_extension
      FROM users u JOIN tenants t ON t.id=u.tenant_id
      WHERE u.username=$4 AND u.tenant_id=$5 AND u.active=true
        AND u.roles=ARRAY[$6]::text[]
        AND EXISTS (SELECT 1 FROM identity_user_roles r WHERE r.user_id=u.id AND r.tenant_id=u.tenant_id AND r.role_name=$6)
    ), new_tenant AS (
      INSERT INTO tenants(id,name,industry,tier,plan_tier,has_claimed_trial_extension)
      SELECT $1,'Isolated private bio test',industry,tier,plan_tier,has_claimed_trial_extension FROM source_actor RETURNING id
    ), new_user AS (
      INSERT INTO users(id,username,email,password_hash,roles,active,tenant_id,created_at,updated_at)
      SELECT $2,$3,$3,source_actor.password_hash,source_actor.roles,true,new_tenant.id,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP
      FROM source_actor CROSS JOIN new_tenant RETURNING id,tenant_id
    ) INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)
      SELECT id,$6,tenant_id,0 FROM new_user RETURNING user_id,tenant_id,role_name`,
    [tenantId,userId,email,source.email,source.organizationId,source.role]);
  if(rows.length!==1 || rows[0].user_id!==userId || rows[0].tenant_id!==tenantId || rows[0].role_name!==source.role) throw new Error('Exact seeded actor role and plan are required.');
  await authenticateRequest(page.request,{username:email,password:source.password,organizationId:tenantId},origin.origin);
  const identity = await page.request.get(new URL('/api/v1/auth/session-identity',origin).href);
  if(identity.status()!==200)throw new Error('New test identity was not verified.');
  const verified: unknown = await identity.json();
  if(!verified || typeof verified!=='object' || !('userId' in verified) || !('tenantId' in verified) || verified.userId!==userId || verified.tenantId!==tenantId) throw new Error('New test identity mismatch.');
  return {tenantId,userId,email};
}
