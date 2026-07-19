import { test, expect } from './fixtures';

test.describe('Autonomous Sync Events API Endpoint', () => {
  test('should accept offline sync events and process idempotency and conflict correctly', async ({ request }) => {
    // 1. Create a tenant and seed it through some API if necessary,
    // or just use the endpoint directly. We'll use the API directly with a mock tenant ID.
    // However, since it relies on x-spiffe-id header for tenant auth, we'll pass that.

    const tenantId = 'e2e-sync-events-tenant';
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/x`;

    // 2. Send initial sync event to create/update an entity
    const eventId1 = 'sync-evt-uuid-1';
    const req1 = {
      events: [
        {
          id: eventId1,
          entity_type: 'booking',
          entity_id: 'booking-1',
          action_type: 'CreateBooking',
          payload: { customer: 'Carlos' },
          base_version: 0
        }
      ]
    };

    const res1 = await request.post('/api/v1/sync/events', {
      headers: {
        'x-spiffe-id': spiffeId
      },
      data: req1
    });

    expect(res1.status()).toBe(200);
    const body1 = await res1.json();
    expect(body1.success).toBe(true);
    expect(body1.applied_count).toBe(1);
    expect(body1.conflict_count).toBe(0);
    expect(body1.failed_count).toBe(0);

    // 3. Send the same event again to test idempotency
    const res2 = await request.post('/api/v1/sync/events', {
      headers: {
        'x-spiffe-id': spiffeId
      },
      data: req1
    });

    expect(res2.status()).toBe(200);
    const body2 = await res2.json();
    expect(body2.success).toBe(true);
    expect(body2.applied_count).toBe(1); // Idempotent events are counted as applied

    // 4. Send a conflicting event (base_version = 0, but DB is now at 1)
    const eventId3 = 'sync-evt-uuid-3';
    const req3 = {
      events: [
        {
          id: eventId3,
          entity_type: 'booking',
          entity_id: 'booking-1',
          action_type: 'UpdateBooking',
          payload: { customer: 'Maya' },
          base_version: 0 // Conflict! Base should be 1 now to succeed
        }
      ]
    };

    const res3 = await request.post('/api/v1/sync/events', {
      headers: {
        'x-spiffe-id': spiffeId
      },
      data: req3
    });

    expect(res3.status()).toBe(200);
    const body3 = await res3.json();
    expect(body3.success).toBe(true);
    expect(body3.applied_count).toBe(0);
    expect(body3.conflict_count).toBe(1);
  });
});
