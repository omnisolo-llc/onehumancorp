import { expect, test } from './fixtures';
import { seedFeedItem } from './feed-fixtures';
import { seedDashboardAuditOwner } from './support/dashboard_audit_fixture';
import { authenticateRequest } from './authenticate';
import { db } from './db_utils';

test.describe('Operations Agent Task Automation', () => {

  test('Persona: Jun the Location Manager completes daily prep checklist', async ({ anonymousPage: page, baseURL }) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const owner = await seedDashboardAuditOwner(baseURL);
    const origin = new URL(baseURL).origin;
    await authenticateRequest(page.request, {
      username: owner.email, password: owner.password, organizationId: owner.tenantId,
    }, origin);
    const identity = await page.request.get('/api/v1/auth/session-identity');
    try {
      expect(identity.status()).toBe(200);
      expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
    } finally { await identity.dispose(); }

    // Arrange the complete owned group before the first dashboard read. A read
    // already in flight can otherwise refill a cache after fixture invalidation.
    const id = await seedFeedItem(page, {
      event_source: 'operations',
      context_payload: { description: 'Review Daily Prep Checklist', feature_type: 'daily_prep_checklist' },
      proposed_action: { action_type: 'Daily Prep Checklist', feature_type: 'daily_prep_checklist' },
    }, owner.tenantId, { requestOrigin: origin });
    expect(await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2',
      [id, owner.tenantId])).toEqual([{ lifecycle_state: 'PENDING_APPROVAL' }]);
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const group = page.getByTestId('grouped-triage-card-daily_prep_checklist-Daily Prep Checklist');
    const operationsTaskCard = page.getByTestId(`triage-card-${id}`);
    await expect.poll(async () => await group.isVisible() || await operationsTaskCard.isVisible(),
      { timeout: 10000, message: 'Wait for the newly created task or its group before expanding' }).toBe(true);
    if (await group.isVisible()) await group.getByRole('button', { name: 'Review Individually' }).click();

    // 2. Check the individual task after any group has loaded and been expanded.
    await expect(operationsTaskCard).toBeVisible({ timeout: 10000 });

    // Ensure buttons are visible
    const markCompleteBtn = operationsTaskCard.getByRole('button', { name: 'Mark Complete', exact: true });
    const assignBtn = operationsTaskCard.getByTestId('feed-assign-btn');
    const dismissBtn = operationsTaskCard.getByTestId('feed-dismiss-btn');

    await expect(markCompleteBtn).toBeVisible();
    await expect(assignBtn).toBeVisible();
    await expect(dismissBtn).toBeVisible();

    // 3. Mark the task complete
    const decision = page.waitForResponse(response => response.url().endsWith(`/api/v1/agent-feed/${id}`) && response.request().method() === 'PUT');
    await markCompleteBtn.click();

    // Check that we hit the API successfully to approve it
    expect((await decision).status()).toBe(200);
    expect(await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2',
      [id, owner.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);

    // 4. Verify the task disappears from the feed
    // Depending on optimistic update or fast refresh, it should hide
    await expect(operationsTaskCard).not.toBeVisible({ timeout: 10000 });
  });
});
