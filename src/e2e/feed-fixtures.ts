import { randomUUID } from 'node:crypto';
import type { Page } from '@playwright/test';
import { expect } from '@playwright/test';
import { db } from './db_utils';
import { E2E_ADMIN_USER } from './identities';

export async function seedFeedItem(page: Page, data: {
  event_source: string;
  context_payload: Record<string, unknown>;
  proposed_action: Record<string, unknown>;
}, tenantId = E2E_ADMIN_USER.organizationId): Promise<string> {
  const id = `e2e-feed-${randomUUID()}`;
  await db.query(`INSERT INTO agent_feed_items
    (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
    VALUES ($1, $2, $3, $4::jsonb, $5::jsonb, 'PENDING_APPROVAL', NOW(), NOW())`,
  [id, tenantId, data.event_source, JSON.stringify(data.context_payload), JSON.stringify(data.proposed_action)]);
  // Direct SQL fixture writes bypass the production cache. Publish the pending
  // state through the real authenticated API so every feed view sees the record.
  const result = await page.evaluate(async (id) => {
    const response = await fetch(`/api/v1/agent-feed/${id}`, {
      method: 'PUT', headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ state: 'PENDING_APPROVAL' }),
    });
    return { status: response.status, body: await response.text() };
  }, id);
  expect(result.status, `activate feed fixture: ${result.body}`).toBe(200);
  return id;
}
