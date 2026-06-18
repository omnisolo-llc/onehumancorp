import { test, expect } from './fixtures';
import { e2eDbQuery } from './db_utils';

test.describe('Neighborhood Collective & Shared Loyalty Mesh', () => {
  const tenantA = 'e2e-collective-a';
  const tenantB = 'e2e-collective-b';
  const buyerId = 'buyer-123';
  const geohashA = '8828308281fffff'; // SF area
  const geohashB = '8828308283fffff'; // Neighbor

  test.beforeAll(async () => {
    // Seed tenants with geohashes
    await e2eDbQuery(`
      INSERT INTO tenants (id, name, industry, geohash, plan_tier)
      VALUES
        ('${tenantA}', 'Maya Bakery', 'Bakery', '${geohashA}', 'pro'),
        ('${tenantB}', 'Carlos Coffee', 'Cafe', '${geohashB}', 'pro')
      ON CONFLICT (id) DO UPDATE SET geohash = EXCLUDED.geohash, industry = EXCLUDED.industry;
    `);

    // Ensure we have users for both (reusing existing passwords from e2e-seed.sql for simplicity)
    await e2eDbQuery(`
      INSERT INTO users (id, username, email, password_hash, roles, tenant_id)
      VALUES
        ('user-a', 'maya@example.com', 'maya@example.com', '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a', ARRAY['ADMIN'], '${tenantA}'),
        ('user-b', 'carlos@example.com', 'carlos@example.com', '$2b$10$hmVhunI7Fq2ZzQ0PguAH5OeXUyb/gNAORUpLPD2g44Ik9/Fd9sM7a', ARRAY['ADMIN'], '${tenantB}')
      ON CONFLICT (id) DO NOTHING;
    `);
  });

  test('Maya invites Carlos and they share loyalty', async ({ page }) => {
    // 1. Manually trigger the discovery proposal (since we don't want to wait for cron)
    const proposalId = 'prop-' + Date.now();
    await e2eDbQuery(`
      INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state)
      VALUES (
        '${proposalId}',
        '${tenantA}',
        'marketing',
        '{"feature_type": "neighborhood_proposal", "partner_id": "${tenantB}", "partner_name": "Carlos Coffee", "partner_industry": "Cafe", "description": "Form a neighborhood collective with Carlos Coffee (Cafe nearby)?"}',
        '{"action_type": "INVITE_TO_COLLECTIVE", "partner_id": "${tenantB}"}',
        'PENDING_APPROVAL'
      )
    `);

    // 2. Maya logs in and invites Carlos
    await page.goto('/dashboard');
    // Note: In real E2E we'd handle multi-tenant login, but here we assume the fixture/auth handles it if we set localStorage or similar.
    // For simplicity, we'll force the tenant ID in localStorage if the app allows it.
    await page.evaluate((tid) => localStorage.setItem('tenant_id', tid), tenantA);
    await page.reload();

    await expect(page.locator(`text=Form a neighborhood collective with Carlos Coffee`)).toBeVisible();
    await page.click(`[data-testid="triage-approve-${proposalId}"]`);
    await expect(page.locator('text=Approved!')).toBeVisible();

    // 3. Verify Tenant B got the invitation
    const invitation = await e2eDbQuery(`
      SELECT id FROM agent_feed_items
      WHERE tenant_id = '${tenantB}'
      AND context_payload->>'feature_type' = 'neighborhood_invitation'
      ORDER BY created_at DESC LIMIT 1
    `);
    expect(invitation.length).toBe(1);
    const inviteId = invitation[0].id;

    // 4. Carlos logs in and joins
    await page.evaluate((tid) => localStorage.setItem('tenant_id', tid), tenantB);
    await page.reload();

    await expect(page.locator(`text=invited you to join a Neighborhood Collective`)).toBeVisible();
    await page.click(`[data-testid="triage-approve-${inviteId}"]`);
    await expect(page.locator('text=Approved!')).toBeVisible();

    // 5. Verify Neighborhood Widget is active
    await page.reload(); // Refresh to trigger loadNeighborhood
    await expect(page.locator('#neighborhood-widget')).toBeVisible();
    await expect(page.locator('text=Neighborhood Loyalty Mesh Active')).toBeVisible();

    // 6. Simulate Loyalty Loop: Earn at A, Redeem at B
    const collective = await e2eDbQuery(`SELECT id FROM ohc_collective WHERE tenant_id = '${tenantA}' LIMIT 1`);
    const collId = collective[0].id;

    // Earn 100 points at Maya's
    await page.request.post('/api/v1/growth/collectives/earn', {
      data: { collective_id: collId, buyer_id: buyerId, amount: 100, tenant_id: tenantA }
    });

    // Check balance at Carlos's (should be 100 via mesh)
    const balanceRes = await page.request.get(`/api/v1/growth/collectives/settlement?tenant_id=${tenantB}`);
    const balanceData = await balanceRes.json();
    // Accountant should show $0.00 until redemption
    await expect(page.locator('#settlement-net-position')).toHaveText('$0.00');

    // Redeem 50 points at Carlos's ($5.00 value)
    const redeemRes = await page.request.post('/api/v1/growth/collectives/redeem', {
      data: { collective_id: collId, buyer_id: buyerId, amount: 50, tenant_id: tenantB }
    });
    const redeemData = await redeemRes.json();
    expect(redeemData.success).toBe(true);
    expect(redeemData.new_balance).toBe(50);

    // 7. Verify Accountant Ledger
    await page.reload();
    await expect(page.locator('#settlement-net-position')).toHaveText('$5.00'); // Carlos is DUE $5.00
    await expect(page.locator('#settlement-details')).toContainText('$0.00 Owed · $5.00 Due');

    // Final Database Check
    const ledger = await e2eDbQuery(`
      SELECT value_cents, status FROM ohc_shared_loyalty_ledger
      WHERE originating_tenant_id = '${tenantA}' AND target_tenant_id = '${tenantB}'
    `);
    expect(ledger.length).toBe(1);
    expect(ledger[0].value_cents).toBe('500'); // 50 points * 10 cents
    expect(ledger[0].status).toBe('PENDING');
  });
});
