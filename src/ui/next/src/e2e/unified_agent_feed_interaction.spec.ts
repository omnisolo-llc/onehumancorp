import { expect, test } from '../../../../e2e/fixtures';
import { db } from '../../../../e2e/db_utils';
import { seedFeedItem } from '../../../../e2e/feed-fixtures';
import { authenticateRequest } from '../../../../e2e/authenticate';
import { seedDashboardAuditOwner } from '../../../../e2e/support/dashboard_audit_fixture';
import { expectMobileCardGeometry } from '../../../../e2e/support/mobile_feed_geometry';

test.describe('Unified Agent Feed Interactive Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });
  let itemId: string;
  let description: string;
  let tenantId: string;
  let remainingId: string;
  const draft = `Review the owner work before recording a decision. ${'Long draft details for a narrow screen. '.repeat(10)}`;
  const readItem = (id: string) => db.query(
    'SELECT lifecycle_state, proposed_action FROM agent_feed_items WHERE id = $1 AND tenant_id = $2',
    [id, tenantId],
  );

  test.beforeEach(async ({ page, baseURL }, testInfo) => {
    if (!baseURL) throw new Error('The isolated local app URL is required');
    const owner = await seedDashboardAuditOwner(baseURL);
    tenantId = owner.tenantId;
    remainingId = `${owner.namespace}-e2e-feed-reschedule`;
    const origin = new URL(baseURL).origin;
    await authenticateRequest(page.request, {
      username: owner.email, password: owner.password, organizationId: tenantId,
    }, origin);
    await page.context().addInitScript(tenant => {
      localStorage.setItem('tenant_id', tenant);
      localStorage.setItem('tenant', tenant);
      localStorage.setItem('business_display_name', tenant);
    }, tenantId);
    description = `Owner proposal ${testInfo.testId}: ${'Long context for mobile owner review. '.repeat(8)}`;
    itemId = await seedFeedItem(page, {
      event_source: 'operations',
      context_payload: { description, feature_type: testInfo.testId },
      proposed_action: { message: draft, generated_response: draft },
    }, tenantId, { requestOrigin: origin });
    expect(await readItem(itemId)).toEqual([{
      lifecycle_state: 'PENDING_APPROVAL',
      proposed_action: { message: draft, generated_response: draft },
    }]);
  });

  test('should render properly, expand for details, and show approval transition', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    const card = page.getByTestId(`triage-card-${itemId}`);
    const remainingCard = page.getByTestId(`triage-card-${remainingId}`);
    await expect(card).toContainText(description);
    await expect(remainingCard).toBeVisible();
    await expectMobileCardGeometry(page, card);
    expect(await page.evaluate(() => document.body.scrollWidth)).toBeLessThanOrEqual(375);
    for (const button of await page.locator('button').all()) {
      if (await button.isVisible()) {
        const box = await button.boundingBox();
        expect(box).not.toBeNull();
        expect(box!.width).toBeGreaterThanOrEqual(44);
        expect(box!.height).toBeGreaterThanOrEqual(44);
      }
    }
    await card.getByTestId('edit-proposal').click();
    await expect(card.getByTestId('edit-proposal-textarea')).toBeVisible();
    await expect(card.getByTestId('edit-proposal-textarea')).toHaveValue(draft);
    await expectMobileCardGeometry(page, card);
    const decisionUrl = new URL(`/api/v1/agent-feed/${itemId}`, page.url()).href;
    const submittedDecisions: unknown[] = [];
    page.on('request', request => {
      if (request.url() === decisionUrl && request.method() === 'PUT') submittedDecisions.push(request.postDataJSON());
    });
    const [decision] = await Promise.all([
      page.waitForResponse(response => response.url() === decisionUrl && response.request().method() === 'PUT'),
      card.getByTestId('feed-approve-btn').click(),
    ]);
    expect(decision.status()).toBe(200);
    expect(await decision.json()).toMatchObject({
      id: itemId, tenant_id: tenantId, lifecycle_state: 'APPROVED', decision_recorded: true,
      proposed_action: { generated_response: draft },
    });
    await expect(card).toHaveClass(/border-green-500/);
    await expect(card).toHaveClass(/scale-95/);
    await expect(card).toBeHidden({ timeout: 2000 });
    expect(await readItem(itemId)).toEqual([{
      lifecycle_state: 'APPROVED',
      proposed_action: expect.objectContaining({ generated_response: draft }),
    }]);
    await expect(remainingCard.getByTestId('feed-approve-btn')).toBeEnabled();
    expect((await readItem(remainingId))[0].lifecycle_state).toBe('PENDING_APPROVAL');
    await page.reload();
    await expect(remainingCard.getByTestId('feed-approve-btn')).toBeEnabled();
    await expect(card).toBeHidden();
    expect((await readItem(itemId))[0].lifecycle_state).toBe('APPROVED');
    expect((await readItem(remainingId))[0].lifecycle_state).toBe('PENDING_APPROVAL');
    const readback = await page.request.get(`${decisionUrl}/decision`);
    try {
      expect(readback.status()).toBe(200);
      expect(await readback.json()).toMatchObject({
        id: itemId, tenant_id: tenantId, lifecycle_state: 'APPROVED', decision_recorded: true,
        proposed_action: { generated_response: draft },
      });
    } finally { await readback.dispose(); }
    expect(submittedDecisions).toEqual([{ state: 'APPROVED', modified_content: draft }]);
  });

  test('should queue actions optimistically when offline', async ({ page, context }) => {
    await page.goto('/dashboard');
    const card = page.getByTestId(`triage-card-${itemId}`);
    await expect(card).toBeVisible();
    await expect(page.getByTestId('offline-queue-readiness')).toHaveAttribute('data-state', 'ready');
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.getByText('You are offline. Actions will sync when online.')).toBeVisible();
    await card.getByTestId('feed-approve-btn').click();
    await expect(card).toBeHidden({ timeout: 2000 });
    const synced = page.waitForResponse(response => response.url().endsWith(`/api/v1/agent-feed/${itemId}`) && response.request().method() === 'PUT');
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
    expect((await synced).status()).toBe(200);
    await expect(async () => {
      const rows = await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2', [itemId, tenantId]);
      expect(rows).toHaveLength(1);
      expect(rows[0].lifecycle_state).toBe('APPROVED');
    }).toPass({ timeout: 15000 });
    await expect(page.getByText('You are offline. Actions will sync when online.')).toBeHidden();
  });

  test('Feed Page should load items and approve', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    const card = page.getByTestId('agent-feed-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByRole('button', { name: 'Approve', exact: true })
      .and(card.getByTestId('feed-approve-btn')).click();
    await expect(card).toBeHidden();
  });

  test('Feed Page should load items and dismiss', async ({ page }) => {
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    const card = page.getByTestId('agent-feed-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByRole('button', { name: 'Dismiss', exact: true })
      .and(card.getByTestId('feed-dismiss-btn')).click();
    await expect(card).toBeHidden();
  });

  test('Dashboard should have functional UnifiedAgentFeed component', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.getByRole('region', { name: 'Unified Agent Feed' })).toBeVisible();
    await expect(page.getByTestId(`triage-card-${itemId}`)).toContainText(description);
  });

  test('should handle inline editing of a proposal', async ({ page }) => {
    await page.goto('/dashboard');
    const card = page.getByTestId(`triage-card-${itemId}`);
    await expect(card).toBeVisible();
    await card.getByTestId('edit-proposal').click();
    const textarea = card.getByTestId('edit-proposal-textarea');
    await expect(textarea).toBeVisible();
    await textarea.fill('This is my manually edited draft text');
    await card.getByTestId('cancel-edit-proposal').click();
    await expect(textarea).toBeHidden();
    await card.getByTestId('edit-proposal').click();
    await textarea.fill('Second edited text');
    await card.getByTestId('save-proposal').click();
    await expect(textarea).toBeHidden({ timeout: 2000 });
    await expect(card).toBeHidden({ timeout: 2000 });
  });

  test('should handle inline editing of an ambassador reply', async ({ page }) => {
    await page.goto('/feed');
    await seedFeedItem(page, {
      event_source: 'Ambassador',
      context_payload: { feature_type: 'ambassador_reply', original_message: description, source: 'SMS' },
      proposed_action: { feature_type: 'ambassador_reply', generated_response: 'Original ambassador reply' },
    }, tenantId);
    // The full feed owns the Ambassador card's supported edit controls.
    await page.goto('/feed');
    const card = page.getByTestId('ambassador-reply-card').filter({ hasText: description });
    await expect(card).toBeVisible();
    await card.getByTestId('feed-edit-btn').click();
    const textarea = card.getByTestId('feed-edit-input');
    await expect(textarea).toBeVisible();
    await textarea.fill('Edited ambassador reply');
    await card.getByTestId('feed-save-edit-btn').click();
    await expect(textarea).toBeHidden({ timeout: 2000 });
    await expect(card).toBeHidden({ timeout: 2000 });
  });
});
