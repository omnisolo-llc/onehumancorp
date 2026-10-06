import type { Locator, Page } from '@playwright/test';
import { test, expect } from '../fixtures';
import { authenticateRequest } from '../authenticate';
import { e2eDbQuery, e2eDbTransaction } from '../db_utils';
import { seedFeedItem } from '../feed-fixtures';
import { seedDashboardAuditOwner } from '../support/dashboard_audit_fixture';
import { expectMobileCardGeometry } from '../support/mobile_feed_geometry';

async function ownedSession(page: Page, baseURL: string | undefined) {
  if (!baseURL) throw new Error('The isolated local app URL is required');
  // anonymousPage creates its own context, so apply the viewport to that page explicitly.
  await page.setViewportSize({ width: 375, height: 812 });
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
  await page.context().addInitScript(tenant => {
    localStorage.setItem('tenant_id', tenant);
    localStorage.setItem('tenant', tenant);
    localStorage.setItem('business_display_name', tenant);
  }, owner.tenantId);
  return { ...owner, origin };
}

async function pressControl(page: Page, control: Locator, key = 'Enter') {
  await expect(control).toBeVisible();
  await expect(control).toBeEnabled();
  await control.focus();
  await expect(control).toBeFocused();
  await page.keyboard.press(key);
}

test.describe('Mobile Unified Agent Feed @mobile', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display seeded Action Center cards and controls on mobile dashboard', async ({ anonymousPage: page, baseURL }) => {
    const owner = await ownedSession(page, baseURL);
    const description = `Mobile owner proposal: ${'Review the context and preserve a readable narrow layout. '.repeat(6)}`;
    const draft = `Draft details ${'with enough content to wrap across several lines. '.repeat(8)}`;
    const itemId = await seedFeedItem(page, {
      event_source: 'operations', context_payload: { description },
      proposed_action: { generated_response: draft, message: draft },
    }, owner.tenantId, { requestOrigin: owner.origin });
    const readItem = () => e2eDbQuery(
      'SELECT lifecycle_state, context_payload, proposed_action FROM agent_feed_items WHERE id=$1 AND tenant_id=$2',
      [itemId, owner.tenantId],
    );
    const persisted = [{ lifecycle_state: 'PENDING_APPROVAL', context_payload: { description },
      proposed_action: { generated_response: draft, message: draft } }];
    expect(await readItem()).toEqual(persisted);

    await page.goto('/dashboard');
    const feedSection = page.getByRole('region', { name: 'Unified Agent Feed' });
    const card = feedSection.getByTestId(`triage-card-${itemId}`);
    await expect(card).toContainText(description);
    await expect(feedSection).toHaveClass(/max-w-full/);
    await expect(feedSection).toHaveClass(/dark:bg-slate-950/);
    await expect(feedSection.getByRole('button', { name: /Proposals/i })).toBeVisible();
    await expect(feedSection.getByRole('button', { name: /Activity Feed/i })).toBeVisible();
    await expect(card.getByTestId('feed-approve-btn')).toBeEnabled();
    await expect(card.getByTestId('edit-proposal')).toBeEnabled();
    await expect(card.getByTestId('feed-dismiss-btn')).toBeEnabled();
    await expect(card.getByTestId('feed-approve-btn')).toHaveClass(/min-h-\[44px\]/);
    await expect(card.getByTestId('feed-approve-btn')).toHaveClass(/min-w-\[44px\]/);
    await expectMobileCardGeometry(page, card);
    expect(await readItem()).toEqual(persisted);
  });

  test('keyboard triage expands, edits, approves and dismisses durably at 375px', async ({ anonymousPage: page, baseURL }) => {
    const owner = await ownedSession(page, baseURL);
    const ids = ['approve', 'dismiss', 'pending'].map(suffix => `${owner.namespace}-mobile-triage-${suffix}`);
    const [approveId, dismissId, pendingId] = ids;
    const context = `Owner review context ${'with detailed background for the decision. '.repeat(8)}`;
    const originalDraft = `Prepared draft ${'for review on a narrow screen. '.repeat(10)}`;
    const editedDraft = `Owner edited draft ${'with the verified details retained. '.repeat(9)}`;
    const readRows = () => e2eDbQuery(`SELECT t.id, t.status, a.payload
      FROM triage_items t JOIN triage_proposed_actions a ON a.triage_item_id=t.id AND a.tenant_id=t.tenant_id
      WHERE t.tenant_id=$1 AND t.id=ANY($2::text[]) ORDER BY t.id`, [owner.tenantId, ids]);
    const expectedRows = (approved = false, dismissed = false) => [
      { id: approveId, status: approved ? 'resolved' : 'pending', payload: approved ? editedDraft : originalDraft },
      { id: dismissId, status: dismissed ? 'dismissed' : 'pending', payload: originalDraft },
      { id: pendingId, status: 'pending', payload: originalDraft },
    ];
    // The guarded transaction requires the native runner's owned PostgreSQL proof.
    // SocialPostDraft records a decision only; it does not request provider work.
    await e2eDbTransaction(async query => {
      for (const id of ids) {
        await query(`INSERT INTO triage_items (id, tenant_id, source, priority, context, status)
          VALUES ($1, $2, 'Owner review', 'high', $3, 'pending')`, [id, owner.tenantId, context]);
        await query(`INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload)
          VALUES ($1, $2, $3, 'SocialPostDraft', $4)`, [`${id}-action`, id, owner.tenantId, originalDraft]);
      }
    });
    expect(await readRows()).toEqual(expectedRows());
    const submittedDecisions: unknown[] = [];
    page.on('request', request => {
      const url = new URL(request.url());
      if (url.origin === owner.origin && url.pathname === '/api/v1/triage/action' && request.method() === 'POST') {
        submittedDecisions.push(request.postDataJSON());
      }
    });

    await page.goto('/triage');
    const card = page.getByTestId(`triage-card-${approveId}`);
    const dismissedCard = page.getByTestId(`triage-card-${dismissId}`);
    const pendingCard = page.getByTestId(`triage-card-${pendingId}`);
    const header = card.getByTestId(`triage-card-header-${approveId}`);
    await expect(card).toContainText(context);
    await expect(dismissedCard).toBeVisible();
    await expect(pendingCard).toBeVisible();
    await expect(header).toHaveAttribute('aria-expanded', 'false');
    await expect(header).toHaveAttribute('aria-controls', `triage-details-${approveId}`);
    await expectMobileCardGeometry(page, card);

    await pressControl(page, header);
    await expect(header).toHaveAttribute('aria-expanded', 'true');
    await expect(card.locator('.proposed-action')).toHaveText(originalDraft);
    await expectMobileCardGeometry(page, card);
    await pressControl(page, header, 'Space');
    await expect(header).toHaveAttribute('aria-expanded', 'false');
    await expect(card.locator('.proposed-action')).toBeHidden();
    await pressControl(page, header);
    await page.keyboard.press('Tab');
    const review = card.getByTestId(`triage-review-btn-${approveId}`);
    await expect(review).toBeFocused();
    await page.keyboard.press('Enter');
    const textarea = card.getByTestId(`triage-edit-textarea-${approveId}`);
    await expect(textarea).toHaveValue(originalDraft);
    await textarea.fill('Discard this unsaved edit');
    await expectMobileCardGeometry(page, card);
    await pressControl(page, card.getByTestId(`triage-cancel-btn-${approveId}`));
    await expect(textarea).toBeHidden();
    expect(submittedDecisions).toEqual([]);
    expect(await readRows()).toEqual(expectedRows());
    await pressControl(page, review);
    await expect(textarea).toHaveValue(originalDraft);
    await textarea.fill(editedDraft);
    await expectMobileCardGeometry(page, card);

    const decide = async (id: string, control: Locator, approved: boolean, edited?: string) => {
      const [response] = await Promise.all([
        page.waitForResponse(response => {
          const url = new URL(response.url());
          return url.origin === owner.origin && url.pathname === '/api/v1/triage/action'
            && response.request().method() === 'POST' && response.request().postDataJSON()?.triage_item_id === id;
        }),
        pressControl(page, control),
      ]);
      expect(response.status()).toBe(200);
      expect(await response.json()).toMatchObject({
        status: 'success', success: true, decision_recorded: true,
        item: { id, tenant_id: owner.tenantId, lifecycle_state: approved ? 'APPROVED' : 'DISMISSED',
          edited_payload: edited ?? null, proposed_action: edited ?? originalDraft },
        dispatch: { status: 'NOT_REQUESTED' },
      });
      await expect(page.locator('#action-status')).toHaveText(approved
        ? 'Approval recorded. Execution or delivery is not verified by this decision.' : 'Dismissal recorded.');
    };
    await decide(approveId, card.getByTestId(`triage-save-btn-${approveId}`), true, editedDraft);
    expect(await readRows()).toEqual(expectedRows(true));
    await expect(card).toBeHidden();
    await expect(dismissedCard).toBeVisible();
    await expect(pendingCard).toBeVisible();
    await pressControl(page, dismissedCard.getByTestId(`triage-card-header-${dismissId}`), 'Space');
    await expectMobileCardGeometry(page, dismissedCard);
    await decide(dismissId, dismissedCard.getByTestId(`triage-dismiss-${dismissId}`), false);
    expect(await readRows()).toEqual(expectedRows(true, true));
    await expect(dismissedCard).toBeHidden();

    const pendingResponse = await page.request.get(`/api/v1/triage/pending?tenant_id=${encodeURIComponent(owner.tenantId)}`);
    try {
      expect(pendingResponse.status()).toBe(200);
      const items = await pendingResponse.json() as { id: string; action_payload: string }[];
      expect(items.some(item => item.id === approveId || item.id === dismissId)).toBe(false);
      expect(items.find(item => item.id === pendingId)).toMatchObject({ id: pendingId, action_payload: originalDraft });
    } finally { await pendingResponse.dispose(); }
    await page.reload();
    await expect(pendingCard).toBeVisible();
    await expect(card).toBeHidden();
    await expect(dismissedCard).toBeHidden();
    await pressControl(page, pendingCard.getByTestId(`triage-card-header-${pendingId}`));
    await expect(pendingCard.getByTestId(`triage-approve-${pendingId}`)).toBeEnabled();
    await expect(pendingCard.getByTestId(`triage-dismiss-${pendingId}`)).toBeEnabled();
    await expectMobileCardGeometry(page, pendingCard);
    expect(await readRows()).toEqual(expectedRows(true, true));
    expect(submittedDecisions).toEqual([
      { triage_item_id: approveId, approved: true, edited_payload: editedDraft },
      { triage_item_id: dismissId, approved: false },
    ]);
  });
});
