import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { expect } from '@playwright/test';
import { db } from './db_utils';
import { E2E_ADMIN_USER } from './identities';

export async function seedFeedItem(page: Page, data: {
  event_source: string;
  context_payload: Record<string, unknown>;
  proposed_action: Record<string, unknown>;
}, tenantId = E2E_ADMIN_USER.organizationId, options?: { requestOrigin: string }): Promise<string> {
  if (options) {
    const origin = new URL(options.requestOrigin);
    if (!['http:', 'https:'].includes(origin.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(origin.hostname)
      || origin.origin !== options.requestOrigin) throw new Error('Pre-navigation feed fixtures require the isolated local app origin');
  }
  const id = `e2e-feed-${randomUUID()}`;
  await db.query(`INSERT INTO agent_feed_items
    (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
    VALUES ($1, $2, $3, $4::jsonb, $5::jsonb, 'PENDING_APPROVAL', NOW(), NOW())`,
  [id, tenantId, data.event_source, JSON.stringify(data.context_payload), JSON.stringify(data.proposed_action)]);
  // Direct SQL fixture writes bypass the production cache. Publish the pending
  // state through the real authenticated API so every feed view sees the record.
  const result = options ? await (async () => {
    // A case-owned fixture can be admitted before mounting a dashboard whose
    // in-flight reads could refill an invalidated cache with pre-fixture rows.
    // page.request uses the browser context's signed cookie jar and the normal
    // same-origin mutation contract, including CSRF provenance headers.
    const response = await page.request.put(new URL(`/api/v1/agent-feed/${id}`, options.requestOrigin).toString(), {
      headers: { origin: options.requestOrigin, 'sec-fetch-site': 'same-origin' },
      data: { state: 'PENDING_APPROVAL' },
    });
    try { return { status: response.status(), body: await response.text() }; }
    finally { await response.dispose(); }
  })() : await page.evaluate(async (id) => {
    const response = await fetch(`/api/v1/agent-feed/${id}`, {
      method: 'PUT', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ state: 'PENDING_APPROVAL' }),
    });
    return { status: response.status, body: await response.text() };
  }, id);
  expect(result.status, `activate feed fixture: ${result.body}`).toBe(200);
  return id;
}
