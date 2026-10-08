import { randomUUID } from 'node:crypto';
import { test, expect } from '../../../../e2e/fixtures';
import { createGrowthOwner } from '../../../../e2e/growth_owner';
import { createOwnerQuoteFromRequest } from '../../../../e2e/playwright/quote_fixture';
import { seedFeedItem } from '../../../../e2e/feed-fixtures';
import { e2eDbQuery } from '../../../../e2e/db_utils';
import { requireLoopbackUrl } from '../../../../e2e/support/recorded_invitation';

const approvalNotice = 'Approval recorded. Execution or delivery is not verified by this decision.';

test.describe('Quote Draft Feed UI', () => {
  test.beforeEach(({ baseURL }) => {
    requireLoopbackUrl(baseURL ?? '');
    expect(process.env.E2E_POSTGRES_CONTAINER).toMatch(/^ohc-e2e-pg-/);
    // The native runner owns the database and omits provider credentials. This
    // tests owner review and durable queue admission, never customer delivery.
    expect(process.env.STRIPE_API_KEY).toBeUndefined();
  });

  test('reviews an owned saved quote and records its exact approval without claiming delivery', async ({ page, baseURL, adminUser, loginAs }, testInfo) => {
    const origin = new URL(baseURL!).origin;
    const owner = await createGrowthOwner(page, baseURL);
    const description = `Quote review ${randomUUID()}`;
    const { quoteId, customerId } = await createOwnerQuoteFromRequest(page.request, baseURL!, {
      username: owner.email, password: adminUser.password, organizationId: owner.tenantId,
    }, { description, priceCents: 150000 });
    const payload = {
      feature_type: 'quote_draft', action_type: `Review quote ${quoteId}`,
      quote_id: quoteId, customer_id: customerId, service: description,
      customer_inquiry: `Inquiry for ${description}`, scope: description, suggested_price: 1500,
      line_items: [{ description, unit_price_cents: 150000, quantity: 1 }],
    };
    // Seed only in the owned disposable DB before mounting the dashboard.
    // POST /agent-feed runs a model and ignores supplied proposed_action; the
    // existing fixture instead persists the exact record and invalidates caches
    // through the real signed-cookie API, without authorizing execution.
    const feedId = await seedFeedItem(page, {
      event_source: 'Sales Agent', context_payload: { description }, proposed_action: payload,
    }, owner.tenantId, { requestOrigin: origin });
    await loginAs(page, { ...adminUser, email: owner.email, organizationId: owner.tenantId });

    const identity = await page.request.get('/api/v1/auth/session-identity');
    expect(identity.status()).toBe(200);
    expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
    await identity.dispose();
    const quote = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(quote.status()).toBe(200);
    expect(await quote.json()).toMatchObject({ quote: {
      id: quoteId, tenant_id: owner.tenantId, customer_id: customerId,
      status: 'DRAFT', total_amount_cents: 150000,
    }, line_items: [{ description, unit_price_cents: 150000, quantity: 1 }] });
    await quote.dispose();

    const endpoint = `/api/v1/agent-feed/${feedId}`;
    const writes: string[] = [];
    page.on('request', request => {
      if (request.method() === 'PUT' && new URL(request.url()).origin === origin
          && new URL(request.url()).pathname === endpoint) writes.push(request.url());
    });
    const card = page.getByTestId(`triage-card-${feedId}`);
    await expect(card).toBeVisible();
    await expect(card.getByTestId('quote-draft-card')).toContainText(description);
    await expect(card.getByTestId('quote-draft-card')).toContainText(payload.customer_inquiry);
    await card.getByTestId('review-quote-draft').click();
    const review = page.getByTestId('quote-review-dialog');
    await expect(review).toBeVisible();
    await expect(review.getByRole('heading', { name: 'Review Quote', exact: true })).toBeVisible();
    await expect(review.getByPlaceholder('Description')).toHaveValue(description);
    await expect(review.getByTestId('modal-quote-total')).toHaveText('$1500.00');
    await review.getByRole('button', { name: 'Close', exact: true }).click();
    await expect(review).toHaveCount(0);
    expect(writes).toEqual([]);
    expect(await e2eDbQuery(
      'SELECT status, total_amount_cents::int AS total_amount_cents FROM quotes WHERE id::text=$1 AND tenant_id=$2',
      [quoteId, owner.tenantId],
    )).toEqual([{ status: 'DRAFT', total_amount_cents: 150000 }]);
    expect(await e2eDbQuery(
      'SELECT decision_state, dispatch_status, job_id FROM agent_feed_decisions WHERE action_id=$1 AND tenant_id=$2',
      [feedId, owner.tenantId],
    )).toEqual([{ decision_state: 'PENDING_APPROVAL', dispatch_status: 'NOT_REQUESTED', job_id: null }]);

    const committed = page.waitForResponse(response => new URL(response.url()).origin === origin
      && new URL(response.url()).pathname === endpoint && response.request().method() === 'PUT');
    await card.getByTestId('approve-quote-draft').click();
    const response = await committed;
    expect(response.status()).toBe(200);
    expect(response.request().postDataJSON()).toEqual({ state: 'APPROVED' });
    const receipt = await response.json();
    expect(receipt).toMatchObject({ id: feedId, tenant_id: owner.tenantId,
      lifecycle_state: 'APPROVED', decision_recorded: true,
      proposed_action: payload, dispatch: { status: 'PENDING' } });
    expect(receipt.dispatch.job_id).toMatch(/^[0-9a-f-]{36}$/);
    const jobId = receipt.dispatch.job_id as string;
    await expect(page.getByRole('status', { name: 'Decision status', exact: true }).filter({ hasText: approvalNotice })).toBeVisible({ timeout: 15000 });
    await expect(card).toHaveCount(0, { timeout: 15000 });

    const readDecision = async () => {
      const readback = await page.request.get(`${endpoint}/decision`);
      try {
        expect(readback.status()).toBe(200);
        const saved = await readback.json();
        expect(saved).toMatchObject({ id: feedId, tenant_id: owner.tenantId,
          lifecycle_state: 'APPROVED', decision_recorded: true,
          proposed_action: payload, dispatch: { job_id: jobId } });
        return saved;
      } finally { await readback.dispose(); }
    };
    await readDecision();
    expect(await e2eDbQuery(
      'SELECT decision_state, job_id FROM agent_feed_decisions WHERE action_id=$1 AND tenant_id=$2',
      [feedId, owner.tenantId],
    )).toEqual([{ decision_state: 'APPROVED', job_id: jobId }]);
    const jobs = await e2eDbQuery(
      `SELECT id, tenant_id, job_type, payload->>'action_id' AS action_id,
              payload->'payload'->>'quote_id' AS quote_id
       FROM ohc_job_queue WHERE tenant_id=$1 AND job_type='agent_feed_action'
         AND payload->>'action_id'=$2`, [owner.tenantId, feedId],
    );
    expect(jobs).toEqual([{ id: jobId, tenant_id: owner.tenantId,
      job_type: 'agent_feed_action', action_id: feedId, quote_id: quoteId }]);
    const reloaded = page.waitForResponse(result => new URL(result.url()).origin === origin
      && new URL(result.url()).pathname === '/api/v1/agent-feed' && result.request().method() === 'GET');
    await page.reload();
    expect((await reloaded).status()).toBe(200);
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(card).toHaveCount(0, { timeout: 15000 });
    const reconciled = await readDecision();
    expect(writes).toEqual([`${origin}${endpoint}`]);
    // A recorded approval/job is evidence of admission only. No assertion here
    // turns a worker return, disappearing card or modal animation into delivery.
    await testInfo.attach('quote-draft-recorded-decision', {
      body: JSON.stringify({ owner, quoteId, customerId, feedId, receipt, reconciled, jobs, writes }),
      contentType: 'application/json',
    });
  });
});
