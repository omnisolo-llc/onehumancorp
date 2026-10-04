import { test, expect } from '../../../e2e/fixtures';
import { e2eDbQuery } from '../../../e2e/db_utils';
import { createGrowthOwner } from '../../../e2e/growth_owner';
import { createEntitlementOwner, expectEntitlementUnchanged, trackTrialClaims } from '../../../e2e/support/entitlement_fixture';

test.describe('AI Agent Department Architecture', () => {
  test('should display approval inbox and activity feed', async ({ page, baseURL }) => {
    if (!baseURL) throw new Error('Playwright baseURL is required for the agent records test.');
    const origin = new URL(baseURL).origin;
    const owner = await createGrowthOwner(page, baseURL);
    const activityId = `${owner.tenantId}-activity`;
    const approvalId = `${owner.tenantId}-approval`;
    const activityDescription = 'Recorded owner decision';
    const approvalDescription = 'Review the owner proposal';
    const inserted = await e2eDbQuery(`INSERT INTO agent_feed_items
      (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
      VALUES ($1, $3, 'operations', jsonb_build_object('description', $4::text), '{}'::jsonb, 'APPROVED', NOW(), NOW()),
             ($2, $3, 'operations', jsonb_build_object('description', $5::text), '{}'::jsonb, 'PENDING_APPROVAL', NOW(), NOW())
      RETURNING id`, [activityId, approvalId, owner.tenantId, activityDescription, approvalDescription]);
    expect(inserted).toEqual([{ id: activityId }, { id: approvalId }]);

    const activityRead = page.waitForResponse(response => response.request().method() === 'GET'
      && new URL(response.url()).origin === origin
      && new URL(response.url()).pathname === '/api/v1/agents/approvals/activity');
    const approvalsRead = page.waitForResponse(response => response.request().method() === 'GET'
      && new URL(response.url()).origin === origin
      && new URL(response.url()).pathname === '/api/v1/agents/approvals');
    await page.goto('/agents');

    await expect(page.getByRole('button', { name: 'My Team', exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'The Manager', exact: true })).toBeVisible();
    const [activityResponse, approvalsResponse] = await Promise.all([activityRead, approvalsRead]);
    expect(activityResponse.status()).toBe(200);
    expect(approvalsResponse.status()).toBe(200);
    expect((await activityResponse.json()).pending_approvals).toEqual([expect.objectContaining({
      id: activityId, tenant_id: owner.tenantId, description: activityDescription, status: 'Approved',
    })]);
    expect((await approvalsResponse.json()).pending_approvals).toEqual([expect.objectContaining({
      id: approvalId, tenant_id: owner.tenantId, description: approvalDescription, status: 'PendingApproval',
    })]);

    await page.getByRole('button', { name: 'Activity Feed', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Activity Feed', exact: true })).toBeVisible();
    await expect(page.getByText(activityDescription, { exact: true })).toBeVisible();
    await expect(page.getByText(approvalDescription, { exact: true })).toHaveCount(0);

    await page.getByRole('button', { name: 'Needs Approval', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Needs Approval', exact: true })).toBeVisible();
    await expect(page.getByText(approvalDescription, { exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve & Send', exact: true })).toBeVisible();
    await expect(page.getByText(activityDescription, { exact: true })).toHaveCount(0);
  });

  test('Pro Mode remains gated by the server plan when trial availability cannot be verified', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL);
    const claims = trackTrialClaims(page);
    await page.addInitScript(() => localStorage.setItem('has_pro', 'true'));
    await page.goto('/agents');
    const toggle = page.getByRole('button', { name: 'Toggle Pro Mode' });
    await expect(toggle).toHaveAttribute('aria-pressed', 'false');
    await toggle.click();
    const paywall = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywall).toBeVisible();
    await expect(page.getByRole('link', { name: 'Upgrade to Pro', exact: true })).toHaveAttribute('href', '/pricing');
    await page.getByRole('button', { name: 'Check trial availability' }).click();
    await expect(page.getByRole('alert').filter({ hasText: 'durable grant is not verified' })).toBeVisible();
    await expect(paywall).toBeVisible();
    await expect(toggle).toHaveAttribute('aria-pressed', 'false');
    await page.getByRole('button', { name: 'Close', exact: true }).click();
    await expect(paywall).not.toBeVisible();
    await toggle.click();
    await expect(paywall).toBeVisible();
    expect(claims).toEqual([]);
    await expectEntitlementUnchanged(page, fixture);
  });

  test('Pro Mode still recognizes an existing persisted Pro entitlement', async ({ page, baseURL }) => {
    const fixture = await createEntitlementOwner(page, baseURL, 'Pro');
    await page.goto('/agents');
    const toggle = page.getByRole('button', { name: 'Toggle Pro Mode' });
    await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    await toggle.click();
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
    await expect(toggle).toHaveAttribute('aria-pressed', 'true');
    await expectEntitlementUnchanged(page, fixture);
  });
});
