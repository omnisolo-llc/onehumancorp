import { test, expect } from '@playwright/test';
import { e2eLogin } from '../db_utils';

test.describe('Autonomous Sync Events (E2E UI Flow)', () => {
  test('should accept sync events via offline proxy after login and handle conflicts', async ({ page }) => {
    // We log into the UI to get a valid authenticated session
    await e2eLogin(page);

    // Give it a moment to stabilize
    await page.waitForTimeout(2000);

    const eventId = `evt-${Date.now()}`;
    const entityId = `ent-${Date.now()}`;

    // 1. Initial successful sync
    const res1 = await page.evaluate(async (payload) => {
      const resp = await fetch('/api/v1/sync/events', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      return { status: resp.status, body: await resp.json() };
    }, {
      events: [
        {
          id: eventId,
          action_type: 'test_action',
          entity_id: entityId,
          base_version: 1,
          payload: JSON.stringify({ key: 'value1' })
        }
      ]
    });

    expect(res1.status).toBe(200);
    expect(res1.body.status).toBe('success');
    expect(res1.body.processed_count).toBe(1);
    expect(res1.body.conflict_count).toBe(0);

    // 2. Idempotent retry
    const res2 = await page.evaluate(async (payload) => {
      const resp = await fetch('/api/v1/sync/events', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      return { status: resp.status, body: await resp.json() };
    }, {
      events: [
        {
          id: eventId,
          action_type: 'test_action',
          entity_id: entityId,
          base_version: 1,
          payload: JSON.stringify({ key: 'value1' })
        }
      ]
    });

    expect(res2.status).toBe(200);
    expect(res2.body.processed_count).toBe(1);
    expect(res2.body.conflict_count).toBe(0);

    // 3. Conflict detection
    const conflictEventId = `evt-conflict-${Date.now()}`;
    const res3 = await page.evaluate(async (payload) => {
      const resp = await fetch('/api/v1/sync/events', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      return { status: resp.status, body: await resp.json() };
    }, {
      events: [
        {
          id: conflictEventId,
          action_type: 'test_action_2',
          entity_id: entityId,
          base_version: 1, // Stale version
          payload: JSON.stringify({ key: 'value2' })
        }
      ]
    });

    expect(res3.status).toBe(200);
    expect(res3.body.processed_count).toBe(0);
    expect(res3.body.conflict_count).toBe(1);
  });
});
