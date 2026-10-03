import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';
import { authenticateRequest } from './authenticate';

test.describe('Owner-only storefront domain resolution', () => {
  test('unmapped domains return a private not-found response for a real UUID tenant', async ({ page, anonymousPage, baseURL, adminUser }) => {
    if (!baseURL || !process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
      throw new Error('Domain fixtures require the native isolated acceptance database.');
    }
    const tenantId = randomUUID(), userId = randomUUID(), email = `domain-${userId}@example.test`;
    const rows = await e2eDbQuery(`WITH source_owner AS (
      SELECT password_hash FROM users WHERE username=$4 AND tenant_id=$5 AND active=true
    ), new_tenant AS (
      INSERT INTO tenants(id,name,industry,tier,plan_tier) SELECT $1,'Domain test','Test','Pro','Pro' FROM source_owner RETURNING id
    ), new_owner AS (
      INSERT INTO users(id,username,email,password_hash,roles,active,tenant_id)
      SELECT $2,$3,$3,source_owner.password_hash,ARRAY['ADMIN'],true,new_tenant.id
      FROM source_owner CROSS JOIN new_tenant RETURNING id,tenant_id
    ) INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)
      SELECT id,'ADMIN',tenant_id,0 FROM new_owner RETURNING user_id`,
    [tenantId, userId, email, adminUser.email, adminUser.organizationId]);
    expect(rows).toHaveLength(1);
    await authenticateRequest(page.request, { username: email, password: adminUser.password, organizationId: tenantId }, new URL(baseURL).origin);
    const path = '/api/v1/storefront/resolve_domain';
    for (let read = 0; read < 2; read += 1) {
      const response = await page.request.get(path, { headers: { 'X-Forwarded-Host': `unmapped-${tenantId}.test` } });
      expect(response.status()).toBe(404);
      expect(response.headers()['cache-control']).toContain('private, no-store');
      expect(response.headers()['x-cache']).toBeUndefined();
    }
    expect((await anonymousPage.request.get(path)).status()).toBe(401);
  });
});
