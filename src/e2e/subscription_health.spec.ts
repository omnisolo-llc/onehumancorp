import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';
import { e2eDbQuery, e2eDbTransaction } from './db_utils';
import { createGrowthOwner } from './growth_owner';
import { requireSubscriptionHealthDraft } from '../../scripts/subscription-health-readiness.mjs';

test('Worker identifies an owned at-risk subscriber and the owner records approval of its draft', async ({ anonymousPage: page, baseURL }) => {
  // Each run owns its tenant and identifiers. No other browser case can consume
  // its draft or prime this tenant's feed before the real worker acknowledges it.
  const owner = await createGrowthOwner(page, baseURL);
  const suffix = randomUUID();
  const expected = {
    tenantId: owner.tenantId,
    jobId: `health-job-${suffix}`,
    subscriberId: `health-subscriber-${suffix}`,
    customerId: `health-customer-${suffix}`,
    planId: `health-plan-${suffix}`,
  };
  await e2eDbTransaction(async query => {
    // Real owned records, without contact details or invented provider IDs.
    await query(`INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Subscription health test customer')`,
      [expected.customerId, owner.tenantId]);
    await query(`INSERT INTO subscription_plans (id, tenant_id, name, price_cents, frequency)
      VALUES ($1, $2, 'Music Lessons', 5000, 'monthly')`, [expected.planId, owner.tenantId]);
    await query(`INSERT INTO subscribers (id, tenant_id, customer_id, subscription_plan_id, status, health_score)
      VALUES ($1, $2, $3, $4, 'PAST_DUE', 100)`,
    [expected.subscriberId, owner.tenantId, expected.customerId, expected.planId]);
    await query(`INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
      VALUES ($1, $2, 'subscription_health', $3::jsonb, 'PENDING')`,
    [expected.jobId, owner.tenantId, JSON.stringify({ subscriber_id: expected.subscriberId, customer_id: expected.customerId })]);
  });

  let action;
  await expect(async () => {
    const rows = await e2eDbQuery(`SELECT row_to_json(j) AS job, row_to_json(s) AS subscriber,
        row_to_json(c) AS customer, row_to_json(p) AS plan,
        row_to_json(f) AS feed, row_to_json(r) AS request
      FROM ohc_job_queue j
      LEFT JOIN subscribers s ON s.id = j.payload->>'subscriber_id' AND s.tenant_id = j.tenant_id
      LEFT JOIN customers c ON c.id = s.customer_id AND c.tenant_id = s.tenant_id
      LEFT JOIN subscription_plans p ON p.id = s.subscription_plan_id AND p.tenant_id = s.tenant_id
      LEFT JOIN agent_feed_items f ON f.tenant_id = s.tenant_id AND f.proposed_action->>'subscriber_id' = s.id
      LEFT JOIN agent_action_requests r ON r.id = f.id AND r.tenant_id = f.tenant_id
      WHERE j.id = $1 AND j.tenant_id = $2`, [expected.jobId, owner.tenantId]);
    // The legacy mirror commits before publication/cache invalidation and before
    // the job finishes. Its presence alone does not acknowledge the whole worker.
    action = requireSubscriptionHealthDraft(rows, expected);
  }).toPass({ timeout: 15000 });
  expect(action).toBeDefined();
  const actionId = action.id as string;

  const identity = await page.request.get('/api/v1/auth/session-identity');
  expect(identity.status()).toBe(200);
  expect(await identity.json()).toMatchObject({ userId: owner.userId, tenantId: owner.tenantId });
  const feed = await page.request.get('/api/v1/agent-feed');
  expect(feed.status()).toBe(200);
  const feedItems = (await feed.json()).items;
  expect(feedItems.filter((item: { id: string }) => item.id === actionId)).toEqual([
    expect.objectContaining({ id: actionId, tenant_id: owner.tenantId, lifecycle_state: 'PENDING_APPROVAL',
      context_payload: action.context_payload, proposed_action: action.proposed_action }),
  ]);
  expect(await e2eDbQuery(`SELECT action_id FROM agent_feed_decisions WHERE action_id = $1 AND tenant_id = $2`,
    [actionId, owner.tenantId])).toEqual([]);

  await page.goto('/dashboard/unified-feed');
  const card = page.getByTestId('agent-feed-card').filter({ hasText: expected.subscriberId });
  await expect(card).toBeVisible({ timeout: 15000 });
  await expect(card.getByText(`identified subscriber ${expected.subscriberId} as at-risk`)).toBeVisible({ timeout: 15000 });
  await expect(card).toContainText(action.proposed_action.draft_reply);
  const decision = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/agent-feed/${actionId}`
    && response.request().method() === 'PUT');
  await card.getByTestId('feed-approve-btn').click();
  const response = await decision;
  expect(response.status()).toBe(200);
  expect(await response.json()).toMatchObject({ id: actionId, tenant_id: owner.tenantId,
    lifecycle_state: 'APPROVED', decision_recorded: true, proposed_action: action.proposed_action });
  await expect(card).toBeHidden({ timeout: 10000 });
  await expect(page.getByLabel('Decision status')).toHaveText('Approval recorded. Execution or delivery is not verified by this decision.');
  expect(await e2eDbQuery(`SELECT f.lifecycle_state, upper(r.status) AS request_status,
      d.decision_state, d.actor_id
    FROM agent_feed_items f
    JOIN agent_action_requests r ON r.id = f.id AND r.tenant_id = f.tenant_id
    JOIN agent_feed_decisions d ON d.action_id = f.id AND d.tenant_id = f.tenant_id
    WHERE f.id = $1 AND f.tenant_id = $2`, [actionId, owner.tenantId]))
    .toEqual([{ lifecycle_state: 'APPROVED', request_status: 'APPROVED', decision_state: 'APPROVED', actor_id: owner.userId }]);
  const readback = await page.request.get(`/api/v1/agent-feed/${actionId}/decision`);
  expect(readback.status()).toBe(200);
  expect(await readback.json()).toMatchObject({ id: actionId, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', decision_recorded: true });
  await page.reload();
  await expect(page.getByTestId('agent-feed')).toBeVisible();
  await expect(page.getByLabel('Decision status')).toHaveText('Approval recorded. Execution or delivery is not verified by this decision.');
  // Empty is shown only after a successful read with a verified owner and no
  // error/uncertain outcome; an absent card alone also passes on read failure.
  await expect(page.getByTestId('triage-feed-empty')).toBeVisible();
  await expect(card).toBeHidden();
  // The native runner removes this disposable database. Retain its immutable
  // decision/dispatch evidence until then rather than deleting an in-flight job.
});
