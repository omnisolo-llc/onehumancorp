import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Autonomous Data Synchronization Protocol for Offline-First Edge Clients', () => {
  const tenantId = 'tenant-e2e-sync';
  const entityId = randomUUID();

  test('should process valid sync events idempotently and handle conflicts correctly', async ({ request }) => {
    // We send a mock spiffe id that contains our test tenant
    const headers = {
      'x-spiffe-id': `spiffe://ohc/org/${tenantId}/agent/x`,
      'Content-Type': 'application/json',
    };

    // 1. Success case
    const eventId1 = randomUUID();
    const payload1 = {
      events: [
        {
          id: eventId1,
          entity_id: entityId,
          entity_type: 'test_sync_entity',
          action_type: 'update',
          payload: { status: 'started' },
          base_version: 1,
        }
      ]
    };

    const res1 = await request.post('/api/v1/sync/events', { headers, data: payload1 });
    expect(res1.status()).toBe(200);
    const body1 = await res1.json();
    expect(body1.success).toBe(true);
    expect(body1.applied_count).toBe(1);
    expect(body1.conflict_count).toBe(0);

    // 2. Idempotency case (re-send same event)
    const res2 = await request.post('/api/v1/sync/events', { headers, data: payload1 });
    expect(res2.status()).toBe(200);
    const body2 = await res2.json();
    expect(body2.success).toBe(true);
    expect(body2.applied_count).toBe(0);
    expect(body2.conflict_count).toBe(0);

    // 3. Conflict case (current DB version is now 2 due to success, so base_version 1 will conflict)
    const eventId2 = randomUUID();
    const payloadConflict = {
      events: [
        {
          id: eventId2,
          entity_id: entityId,
          entity_type: 'test_sync_entity',
          action_type: 'update',
          payload: { status: 'cancelled' },
          base_version: 1, // DB has 2 now
        }
      ]
    };

    const res3 = await request.post('/api/v1/sync/events', { headers, data: payloadConflict });
    expect(res3.status()).toBe(200);
    const body3 = await res3.json();
    expect(body3.success).toBe(true);
    expect(body3.applied_count).toBe(0);
    expect(body3.conflict_count).toBe(1);
  });
});
